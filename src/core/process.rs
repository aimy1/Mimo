use anyhow::Result;
use std::process::Command;

pub struct CoreProcess;

impl CoreProcess {
    /// Check if systemd service 'mihomo' or 'clash' is active
    pub fn is_systemd_active() -> bool {
        if let Ok(output) = Command::new("systemctl")
            .args(["is-active", "mihomo"])
            .output()
        {
            if String::from_utf8_lossy(&output.stdout).trim() == "active" {
                return true;
            }
        }
        if let Ok(output) = Command::new("systemctl")
            .args(["is-active", "clash"])
            .output()
        {
            if String::from_utf8_lossy(&output.stdout).trim() == "active" {
                return true;
            }
        }
        false
    }

    /// Try starting Mihomo via systemd or background process
    pub fn start() -> Result<()> {
        if Self::is_systemd_active() {
            return Ok(());
        }

        // Try systemd start
        let status = Command::new("systemctl")
            .args(["start", "mihomo"])
            .status();

        if let Ok(st) = status {
            if st.success() {
                return Ok(());
            }
        }

        // Fallback: systemctl --user start mihomo
        let user_status = Command::new("systemctl")
            .args(["--user", "start", "mihomo"])
            .status();

        if let Ok(st) = user_status {
            if st.success() {
                return Ok(());
            }
        }

        anyhow::bail!("Failed to start Mihomo via systemctl (system or --user)")
    }

    pub fn stop() -> Result<()> {
        let _ = Command::new("systemctl").args(["stop", "mihomo"]).status();
        let _ = Command::new("systemctl")
            .args(["--user", "stop", "mihomo"])
            .status();
        Ok(())
    }

    pub fn restart() -> Result<()> {
        Self::stop()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
        Self::start()
    }
}
