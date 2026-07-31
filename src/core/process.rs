use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Child, Command as StdCommand, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use tracing::{info, warn};

static MIHOMO_PID: AtomicU32 = AtomicU32::new(0);
static CHILD_PROCESS: Mutex<Option<Child>> = Mutex::new(None);

pub struct CoreProcess;

impl CoreProcess {
    /// Detect if Mihomo binary is installed in $PATH or standard Linux system locations
    pub fn find_mihomo_binary() -> Option<PathBuf> {
        let candidates = [
            "mihomo",
            "clash-meta",
            "clash",
            "/usr/bin/mihomo",
            "/usr/local/bin/mihomo",
            "/usr/bin/clash-meta",
            "/usr/local/bin/clash-meta",
        ];

        for cand in candidates {
            let p = Path::new(cand);
            if p.is_absolute() && p.exists() {
                return Some(p.to_path_buf());
            }
            if let Ok(path) = which::which(cand) {
                return Some(path);
            }
        }
        None
    }

    /// Check if Mihomo process is active (via spawned child or systemctl)
    pub fn is_running() -> bool {
        let pid = MIHOMO_PID.load(Ordering::Relaxed);
        if pid > 0 {
            unsafe {
                if libc::kill(pid as i32, 0) == 0 {
                    return true;
                }
            }
        }

        Self::is_systemd_active()
    }

    pub fn is_systemd_active() -> bool {
        let output = StdCommand::new("systemctl")
            .args(["is-active", "mihomo"])
            .output();

        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.trim() == "active"
        } else {
            false
        }
    }

    /// Start Mihomo Core process with `mihomo -f profile.yaml`
    pub fn start_with_config(config_path: &Path) -> Result<u32> {
        if Self::is_running() {
            Self::stop()?;
        }

        let binary = Self::find_mihomo_binary()
            .context("Mihomo binary not found in $PATH. Please install mihomo or clash-meta.")?;

        info!("Starting Mihomo Core: {:?} -f {:?}", binary, config_path);

        let work_dir = config_path.parent().and_then(|p| p.parent()).unwrap_or_else(|| Path::new("/home/fd/.config/mimo"));

        let child = StdCommand::new(&binary)
            .arg("-d")
            .arg(work_dir)
            .arg("-f")
            .arg(config_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn Mihomo Core process")?;

        let pid = child.id();
        MIHOMO_PID.store(pid, Ordering::Relaxed);

        if let Ok(mut guard) = CHILD_PROCESS.lock() {
            *guard = Some(child);
        }

        info!("Mihomo Core spawned successfully with PID {}", pid);
        Ok(pid)
    }

    pub fn stop() -> Result<()> {
        let pid = MIHOMO_PID.swap(0, Ordering::Relaxed);
        if pid > 0 {
            unsafe {
                libc::kill(pid as i32, libc::SIGTERM);
            }
            info!("Sent SIGTERM to Mihomo PID {}", pid);
        }

        if let Ok(mut guard) = CHILD_PROCESS.lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }

        if Self::is_systemd_active() {
            let _ = StdCommand::new("systemctl").args(["stop", "mihomo"]).output();
        }

        Ok(())
    }

    pub fn restart() -> Result<()> {
        let pid = MIHOMO_PID.load(Ordering::Relaxed);
        if pid > 0 {
            unsafe {
                libc::kill(pid as i32, libc::SIGHUP);
            }
            info!("Sent SIGHUP to Mihomo PID {}", pid);
            return Ok(());
        }

        let output = StdCommand::new("systemctl")
            .args(["restart", "mihomo"])
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                return Ok(());
            }
        }

        bail!("Failed to restart Mihomo Core process")
    }

    /// Crash recovery checker: if child died unexpectedly, restart it with config_path
    pub fn check_crash_recovery(config_path: Option<&Path>) -> bool {
        let pid = MIHOMO_PID.load(Ordering::Relaxed);
        if pid > 0 {
            unsafe {
                if libc::kill(pid as i32, 0) != 0 {
                    warn!("Mihomo process PID {} crashed, recovering...", pid);
                    MIHOMO_PID.store(0, Ordering::Relaxed);
                    if let Some(path) = config_path {
                        let _ = Self::start_with_config(path);
                        return true;
                    }
                }
            }
        }
        false
    }
}
