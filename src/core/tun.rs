use std::process::Command;

pub struct TunMode;

impl TunMode {
    /// Check if current user is root or process has cap_net_admin capability
    pub fn check_privilege() -> bool {
        // 1. Check if UID is 0 (root)
        if unsafe { libc::getuid() } == 0 {
            return true;
        }

        // 2. Check if cap_net_admin capability is present on binary or process
        if let Ok(output) = Command::new("getcap")
            .arg("/usr/bin/mihomo")
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            if text.contains("cap_net_admin") {
                return true;
            }
        }

        false
    }
}
