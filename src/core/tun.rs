use anyhow::{bail, Result};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Default)]
pub struct TunInterfaceDetails {
    pub name: String,
    pub is_up: bool,
    pub mtu: u32,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

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

    /// Detect active TUN interface status and detailed metrics on Linux
    pub fn get_interface_details() -> TunInterfaceDetails {
        let (name, is_up) = Self::get_interface_info();
        if name == "None" {
            return TunInterfaceDetails {
                name,
                is_up: false,
                mtu: 0,
                rx_bytes: 0,
                tx_bytes: 0,
            };
        }

        let sys_path = format!("/sys/class/net/{}", name);
        let mtu = fs::read_to_string(format!("{}/mtu", sys_path))
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(1500);

        let rx_bytes = fs::read_to_string(format!("{}/statistics/rx_bytes", sys_path))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);

        let tx_bytes = fs::read_to_string(format!("{}/statistics/tx_bytes", sys_path))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);

        TunInterfaceDetails {
            name,
            is_up,
            mtu,
            rx_bytes,
            tx_bytes,
        }
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

    /// Revoke TUN capability via password passed to sudo -S
    pub fn revoke_privilege_with_password(password: &str) -> Result<()> {
        let binary = match crate::core::CoreProcess::find_mihomo_binary() {
            Some(b) => b,
            None => bail!("Mihomo binary not found to revoke privileges"),
        };

        let binary_str = binary.to_string_lossy().to_string();

        let mut child = Command::new("sudo")
            .args(["-S", "setcap", "-r", &binary_str])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(password.as_bytes());
            let _ = stdin.write_all(b"\n");
        }

        let output = child.wait_with_output()?;
        if output.status.success() || !Self::check_privilege() {
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

    /// Grant TUN capability via Polkit GUI (pkexec --disable-internal-agent without TTY fallback)
    pub fn grant_privilege_pkexec() -> Result<()> {
        let binary = match crate::core::CoreProcess::find_mihomo_binary() {
            Some(b) => b,
            None => bail!("Mihomo binary not found to grant privileges"),
        };

        let binary_str = binary.to_string_lossy().to_string();

        // Use --disable-internal-agent so pkexec NEVER spawns text pkttyagent in controlling TTY!
        if let Ok(status) = Command::new("pkexec")
            .args(["--disable-internal-agent", "setcap", "cap_net_admin+ep", &binary_str])
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
        if !Self::check_privilege() {
            return Ok(()); // Already revoked
        }

        let binary = match crate::core::CoreProcess::find_mihomo_binary() {
            Some(b) => b,
            None => bail!("Mihomo binary not found to revoke privileges"),
        };

        let binary_str = binary.to_string_lossy().to_string();

        if let Ok(status) = Command::new("pkexec")
            .args(["--disable-internal-agent", "setcap", "-r", &binary_str])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            if status.success() || !Self::check_privilege() {
                return Ok(());
            }
        }

        if let Ok(output) = Command::new("sudo")
            .args(["-n", "setcap", "-r", &binary_str])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
        {
            if output.status.success() || !Self::check_privilege() {
                return Ok(());
            }
        }

        bail!("Failed to revoke privileges via non-interactive Polkit/Sudo")
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
