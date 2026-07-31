use anyhow::Result;
use std::process::Command;

pub struct SystemProxy;

impl SystemProxy {
    /// Check if GNOME gsettings proxy mode is set to manual
    pub fn is_enabled() -> bool {
        if let Ok(output) = Command::new("gsettings")
            .args(["get", "org.gnome.system.proxy", "mode"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            if text.contains("'manual'") {
                return true;
            }
        }
        false
    }

    /// Enable Linux GNOME system proxy
    pub fn enable(http_port: u16, socks_port: u16) -> Result<()> {
        // Set gsettings HTTP proxy
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.http", "host", "'127.0.0.1'"])
            .status();
        let _ = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.system.proxy.http",
                "port",
                &http_port.to_string(),
            ])
            .status();

        // Set gsettings HTTPS proxy
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.https", "host", "'127.0.0.1'"])
            .status();
        let _ = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.system.proxy.https",
                "port",
                &http_port.to_string(),
            ])
            .status();

        // Set gsettings SOCKS proxy
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.socks", "host", "'127.0.0.1'"])
            .status();
        let _ = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.system.proxy.socks",
                "port",
                &socks_port.to_string(),
            ])
            .status();

        // Enable manual mode
        let status = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", "'manual'"])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to set GNOME system proxy via gsettings");
        }

        Ok(())
    }

    /// Disable Linux GNOME system proxy
    pub fn disable() -> Result<()> {
        let status = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", "'none'"])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to clear GNOME system proxy via gsettings");
        }

        Ok(())
    }
}
