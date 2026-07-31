use anyhow::{bail, Result};
use std::io::Write;
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
