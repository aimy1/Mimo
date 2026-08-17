use anyhow::Result;
use std::process::Command;

pub struct SystemProxy;

impl SystemProxy {
    /// Detect if system proxy is enabled across GNOME, KDE Plasma, or environment
    pub fn is_enabled() -> bool {
        // 1. Check GNOME gsettings
        if let Ok(output) = Command::new("gsettings")
            .args(["get", "org.gnome.system.proxy", "mode"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            if text.contains("'manual'") {
                return true;
            }
        }

        // 2. Check KDE Plasma (kreadconfig6 or kreadconfig5)
        for cmd in ["kreadconfig6", "kreadconfig5"] {
            if let Ok(output) = Command::new(cmd)
                .args(["--file", "kioslaverc", "--group", "Proxy Settings", "--key", "ProxyType"])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if text == "1" {
                    return true;
                }
            }
        }

        // 3. Check environment proxy variable
        if let Ok(val) = std::env::var("http_proxy").or_else(|_| std::env::var("HTTP_PROXY")) {
            if !val.is_empty() {
                return true;
            }
        }

        false
    }

    /// Enable Linux system proxy for GNOME, KDE, and related desktop environments
    pub fn enable(http_port: u16, socks_port: u16) -> Result<()> {
        let mut any_success = false;

        // 1. Set GNOME / Cinnamon / Unity proxy via gsettings
        let http_port_str = http_port.to_string();
        let socks_port_str = socks_port.to_string();

        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.http", "host", "'127.0.0.1'"])
            .status();
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.http", "port", &http_port_str])
            .status();
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.https", "host", "'127.0.0.1'"])
            .status();
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.https", "port", &http_port_str])
            .status();
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.socks", "host", "'127.0.0.1'"])
            .status();
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy.socks", "port", &socks_port_str])
            .status();

        if let Ok(status) = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", "'manual'"])
            .status()
        {
            if status.success() {
                any_success = true;
            }
        }

        // 2. Set KDE Plasma 5/6 proxy via kwriteconfig
        for cmd in ["kwriteconfig6", "kwriteconfig5"] {
            let http_proxy_val = format!("http://127.0.0.1 {}", http_port);
            let socks_proxy_val = format!("socks://127.0.0.1 {}", socks_port);

            let s1 = Command::new(cmd)
                .args(["--file", "kioslaverc", "--group", "Proxy Settings", "--key", "ProxyType", "1"])
                .status();
            let _ = Command::new(cmd)
                .args(["--file", "kioslaverc", "--group", "Proxy Settings", "--key", "httpProxy", &http_proxy_val])
                .status();
            let _ = Command::new(cmd)
                .args(["--file", "kioslaverc", "--group", "Proxy Settings", "--key", "httpsProxy", &http_proxy_val])
                .status();
            let _ = Command::new(cmd)
                .args(["--file", "kioslaverc", "--group", "Proxy Settings", "--key", "socksProxy", &socks_proxy_val])
                .status();

            if let Ok(st) = s1 {
                if st.success() {
                    any_success = true;
                    // Notify KDE KIO of proxy configuration change
                    let _ = Command::new("dbus-send")
                        .args(["--type=signal", "/KIO/Scheduler", "org.kde.KIO.Scheduler.reparseSlaveConfiguration", "string:"])
                        .status();
                }
            }
        }

        if !any_success {
            // If neither gsettings nor kwriteconfig succeeded, we still set process env variables
            unsafe {
                std::env::set_var("http_proxy", format!("http://127.0.0.1:{}", http_port));
                std::env::set_var("https_proxy", format!("http://127.0.0.1:{}", http_port));
                std::env::set_var("all_proxy", format!("socks5://127.0.0.1:{}", socks_port));
            }
        }

        Ok(())
    }

    /// Disable Linux system proxy for GNOME, KDE, and environment
    pub fn disable() -> Result<()> {
        // 1. Disable GNOME gsettings
        let _ = Command::new("gsettings")
            .args(["set", "org.gnome.system.proxy", "mode", "'none'"])
            .status();

        // 2. Disable KDE Plasma kioslaverc (ProxyType 0 = No Proxy)
        for cmd in ["kwriteconfig6", "kwriteconfig5"] {
            if let Ok(st) = Command::new(cmd)
                .args(["--file", "kioslaverc", "--group", "Proxy Settings", "--key", "ProxyType", "0"])
                .status()
            {
                if st.success() {
                    let _ = Command::new("dbus-send")
                        .args(["--type=signal", "/KIO/Scheduler", "org.kde.KIO.Scheduler.reparseSlaveConfiguration", "string:"])
                        .status();
                }
            }
        }

        // 3. Clear environment variables
        unsafe {
            std::env::remove_var("http_proxy");
            std::env::remove_var("https_proxy");
            std::env::remove_var("all_proxy");
            std::env::remove_var("HTTP_PROXY");
            std::env::remove_var("HTTPS_PROXY");
            std::env::remove_var("ALL_PROXY");
        }

        Ok(())
    }
}
