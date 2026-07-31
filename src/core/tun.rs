use anyhow::{bail, Result};
use std::process::Command;

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

    /// Grant TUN capability (cap_net_admin+ep) to the Mihomo binary via pkexec or sudo
    pub fn grant_privilege() -> Result<()> {
        let binary = match crate::core::CoreProcess::find_mihomo_binary() {
            Some(b) => b,
            None => bail!("Mihomo binary not found to grant privileges"),
        };

        let binary_str = binary.to_string_lossy().to_string();

        // 1. Try pkexec setcap cap_net_admin+ep <binary> (Pops up Linux Polkit GUI password dialog)
        if let Ok(status) = Command::new("pkexec")
            .args(["setcap", "cap_net_admin+ep", &binary_str])
            .status()
        {
            if status.success() {
                return Ok(());
            }
        }

        // 2. Fallback to sudo setcap cap_net_admin+ep <binary>
        if let Ok(status) = Command::new("sudo")
            .args(["setcap", "cap_net_admin+ep", &binary_str])
            .status()
        {
            if status.success() {
                return Ok(());
            }
        }

        bail!("Failed to obtain system privileges for TUN mode")
    }
}
