use anyhow::{bail, Result};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct TunMode;

impl TunMode {
    /// Check if current user is root or Mihomo binary has cap_net_admin capability
    pub fn check_privilege() -> bool {
        // 1. Check if UID is 0 (root)
        if unsafe { libc::getuid() } == 0 {
            return true;
        }

        // 2. Check if cap_net_admin capability is present on the Mihomo binary
        if let Some(binary) = crate::core::CoreProcess::find_mihomo_binary() {
            if let Ok(output) = Command::new("getcap").arg(&binary).output() {
                let text = String::from_utf8_lossy(&output.stdout);
                if text.contains("cap_net_admin") {
                    return true;
                }
            }
        }

        false
    }

    /// Detect active TUN interface status (e.g. tun0, utun, mihomo) on Linux
    pub fn get_interface_info() -> (String, bool) {
        let candidate_ifaces = ["tun0", "mihomo", "utun", "clash"];
        
        for iface in candidate_ifaces {
            let sys_path = format!("/sys/class/net/{}", iface);
            if Path::new(&sys_path).exists() {
                let operstate_path = format!("{}/operstate", sys_path);
                let is_up = if let Ok(state) = fs::read_to_string(&operstate_path) {
                    let s = state.trim().to_lowercase();
                    s == "up" || s == "unknown"
                } else {
                    true
                };
                return (iface.to_string(), is_up);
            }
        }

        // Search any interface starting with tun
        if let Ok(entries) = fs::read_dir("/sys/class/net/") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("tun") {
                    return (name, true);
                }
            }
        }

        ("None".to_string(), false)
    }

    /// Grant TUN capability via password passed to sudo -S
    pub fn grant_privilege_with_password(password: &str) -> Result<()> {
        let binary = match crate::core::CoreProcess::find_mihomo_binary() {
            Some(b) => b,
            None => bail!("Mihomo binary not found to grant privileges"),
        };

        let binary_str = binary.to_string_lossy().to_string();

        let mut child = Command::new("sudo")
            .args(["-S", "setcap", "cap_net_admin+ep", &binary_str])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(password.as_bytes());
            let _ = stdin.write_all(b"\n");
        }

        let output = child.wait_with_output()?;
        if output.status.success() && Self::check_privilege() {
            return Ok(());
        }

        let err_msg = String::from_utf8_lossy(&output.stderr);
        let msg = err_msg.trim();
        if msg.is_empty() {
            bail!("System authorization failed. Please check your password.")
        } else {
            bail!("Authorization error: {}", msg)
        }
    }

    /// Grant TUN capability via Polkit GUI (pkexec without TTY fallback)
    pub fn grant_privilege_pkexec() -> Result<()> {
        let binary = match crate::core::CoreProcess::find_mihomo_binary() {
            Some(b) => b,
            None => bail!("Mihomo binary not found to grant privileges"),
        };

        let binary_str = binary.to_string_lossy().to_string();

        // Pass Stdio::null() so pkexec NEVER writes text to TTY terminal stdout/stderr!
        if let Ok(status) = Command::new("pkexec")
            .args(["setcap", "cap_net_admin+ep", &binary_str])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            if status.success() && Self::check_privilege() {
                return Ok(());
            }
        }

        bail!("Failed to obtain privileges via Linux Polkit GUI")
    }

    /// Revoke TUN capability from Mihomo binary (`setcap -r`)
    pub fn revoke_privilege() -> Result<()> {
        let binary = match crate::core::CoreProcess::find_mihomo_binary() {
            Some(b) => b,
            None => bail!("Mihomo binary not found to revoke privileges"),
        };

        let binary_str = binary.to_string_lossy().to_string();

        if let Ok(status) = Command::new("pkexec")
            .args(["setcap", "-r", &binary_str])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            if status.success() {
                return Ok(());
            }
        }

        let output = Command::new("sudo")
            .args(["-n", "setcap", "-r", &binary_str])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            bail!("Failed to revoke privileges")
        }
    }

    /// Try non-interactive privilege escalation (e.g. Polkit GUI or passwordless sudo)
    pub fn grant_privilege() -> Result<()> {
        if Self::grant_privilege_pkexec().is_ok() {
            return Ok(());
        }

        // Try passwordless sudo -n
        let binary = match crate::core::CoreProcess::find_mihomo_binary() {
            Some(b) => b,
            None => bail!("Mihomo binary not found to grant privileges"),
        };
        let binary_str = binary.to_string_lossy().to_string();

        if let Ok(output) = Command::new("sudo")
            .args(["-n", "setcap", "cap_net_admin+ep", &binary_str])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
        {
            if output.status.success() && Self::check_privilege() {
                return Ok(());
            }
        }

        bail!("Non-interactive privilege escalation failed")
    }
}
