use crate::api::MihomoClient;
use crate::core::{CoreProcess, SystemProxy, TunMode};
use crate::profile::ProfileManager;
use anyhow::Result;

pub async fn handle_status(client: &MihomoClient) -> Result<()> {
    let version = client.get_version().await;
    let config = client.get_config().await;
    let proxies = client.get_proxies().await;

    println!("\x1b[1;36m=== Mimo / Mihomo Status ===\x1b[0m");
    println!("API Target : {}", client.base_url());
    println!("Systemd Core: {}", if CoreProcess::is_systemd_active() { "\x1b[32mActive\x1b[0m" } else { "\x1b[33mInactive / User\x1b[0m" });
    println!("Sys Proxy  : {}", if SystemProxy::is_enabled() { "\x1b[32mEnabled (GSettings)\x1b[0m" } else { "\x1b[90mDisabled\x1b[0m" });
    println!("TUN Privs  : {}", if TunMode::check_privilege() { "\x1b[32mOK (cap_net_admin)\x1b[0m" } else { "\x1b[31mInsufficient (Needs root/cap)\x1b[0m" });

    match version {
        Ok(v) => println!("Core Ver   : \x1b[1;32m{}\x1b[0m (Meta: {})", v.version, v.meta),
        Err(e) => println!("Core Ver   : \x1b[1;31mOffline / Error ({})\x1b[0m", e),
    }

    match config {
        Ok(c) => {
            println!("Mode       : \x1b[1;33m{}\x1b[0m", c.mode.unwrap_or_else(|| "Unknown".into()));
            if let Some(port) = c.http_port {
                println!("HTTP Port  : {}", port);
            }
            if let Some(port) = c.socks_port {
                println!("SOCKS Port : {}", port);
            }
            if let Some(port) = c.mixed_port {
                println!("Mixed Port : {}", port);
            }
        }
        Err(e) => println!("Config     : Error fetching config: {}", e),
    }

    if let Ok(p_resp) = proxies {
        if let Some(global) = p_resp.proxies.get("GLOBAL") {
            println!("Active Node: \x1b[1;32m{}\x1b[0m", global.now.as_deref().unwrap_or("None"));
        }
        println!("Total Proxies/Groups: {}", p_resp.proxies.len());
    }

    Ok(())
}

pub async fn handle_core_action(action: &str) -> Result<()> {
    match action.to_lowercase().as_str() {
        "start" => {
            println!("Starting Mihomo core service...");
            let profiles = ProfileManager::list_profiles()?;
            if let Some(active) = profiles.iter().find(|p| p.is_active).or_else(|| profiles.first()) {
                CoreProcess::start_with_config(&active.file_path)?;
            } else {
                CoreProcess::restart()?;
            }
            println!("\x1b[32mDone.\x1b[0m");
        }
        "stop" => {
            println!("Stopping Mihomo core service...");
            CoreProcess::stop()?;
            println!("\x1b[32mDone.\x1b[0m");
        }
        "restart" => {
            println!("Restarting Mihomo core service...");
            CoreProcess::restart()?;
            println!("\x1b[32mDone.\x1b[0m");
        }
        "download" | "install" => {
            println!("Downloading & Installing Mihomo Core binary automatically...");
            let path = crate::core::CoreDownloader::download_and_install(|msg| {
                println!(" -> {}", msg);
            }).await?;
            println!("\x1b[32mSuccessfully installed Mihomo Core binary to {:?}\x1b[0m", path);
        }
        _ => anyhow::bail!("Unknown core action"),
    }
    Ok(())
}

pub async fn handle_sysproxy_toggle(enable: bool, http_port: u16, socks_port: u16) -> Result<()> {
    if enable {
        println!("Enabling Linux System Proxy (HTTP: {}, SOCKS: {})...", http_port, socks_port);
        SystemProxy::enable(http_port, socks_port)?;
        println!("\x1b[32mSystem proxy enabled.\x1b[0m");
    } else {
        println!("Disabling Linux System Proxy...");
        SystemProxy::disable()?;
        println!("\x1b[32mSystem proxy disabled.\x1b[0m");
    }
    Ok(())
}

pub async fn handle_tun_action(client: &MihomoClient, action: &str) -> Result<()> {
    match action.to_lowercase().as_str() {
        "on" | "enable" | "true" | "1" => {
            if !TunMode::check_privilege() {
                println!("\x1b[33mWarning: Mihomo binary lacks CAP_NET_ADMIN privilege. Attempting privilege escalation...\x1b[0m");
                if let Err(e) = TunMode::grant_privilege() {
                    println!("\x1b[31mFailed to grant privileges: {}\x1b[0m", e);
                    println!("Please run 'mimo tun grant' or authorize via TUI.");
                }
            }
            println!("Setting TUN mode to \x1b[32mON\x1b[0m (stack: system, auto-route: true)...");
            client.set_tun_enabled(true).await?;
            let details = TunMode::get_interface_details();
            println!("\x1b[1;32mTUN mode enabled successfully! (Interface: {}, MTU: {}, Up: {})\x1b[0m", details.name, details.mtu, details.is_up);
        }
        "off" | "disable" | "false" | "0" => {
            println!("Setting TUN mode to \x1b[33mOFF\x1b[0m...");
            client.set_tun_enabled(false).await?;
            println!("\x1b[1;32mTUN mode disabled successfully.\x1b[0m");
        }
        "system" | "gvisor" | "lwip" => {
            println!("Switching TUN stack to \x1b[33m{}\x1b[0m...", action);
            client.set_tun_config(true, action).await?;
            println!("\x1b[1;32mTUN network stack updated to '{}'.\x1b[0m", action);
        }
        "status" => {
            let is_priv = TunMode::check_privilege();
            let details = TunMode::get_interface_details();
            let config = client.get_config().await.ok();
            let is_enabled = config.as_ref().and_then(|c| c.tun.as_ref()).map(|t| t.enable).unwrap_or(false);
            let stack = config.as_ref().and_then(|c| c.tun.as_ref()).and_then(|t| t.stack.clone()).unwrap_or_else(|| "system".into());

            println!("\x1b[1;36m=== Linux TUN Mode Status & Interface Metrics ===\x1b[0m");
            println!("CAP_NET_ADMIN Privileges : {}", if is_priv { "\x1b[32mOK (Authorized)\x1b[0m" } else { "\x1b[31mMissing (Needs setcap)\x1b[0m" });
            println!("Mihomo TUN Configuration : {}", if is_enabled { "\x1b[32mEnabled (ON)\x1b[0m" } else { "\x1b[90mDisabled (OFF)\x1b[0m" });
            println!("TUN Network Stack        : \x1b[33m{}\x1b[0m", stack);
            println!("Active Interface Name    : \x1b[1;32m{}\x1b[0m ({})", details.name, if details.is_up { "\x1b[32mUP\x1b[0m" } else { "\x1b[31mDOWN / None\x1b[0m" });
            if details.name != "None" {
                println!("Interface MTU            : {}", details.mtu);
                println!("Traffic RX / TX          : {} RX / {} TX", crate::ui::theme::format_bytes(details.rx_bytes), crate::ui::theme::format_bytes(details.tx_bytes));
            }
        }
        "grant" => {
            println!("Attempting non-interactive privilege escalation for TUN mode...");
            TunMode::grant_privilege()?;
            println!("\x1b[1;32mCAP_NET_ADMIN privilege granted successfully!\x1b[0m");
        }
        "revoke" => {
            println!("Revoking CAP_NET_ADMIN privilege from Mihomo binary...");
            TunMode::revoke_privilege()?;
            println!("\x1b[1;32mPrivilege revoked successfully.\x1b[0m");
        }
        _ => anyhow::bail!("Unknown TUN action '{}'. Supported actions: on, off, status, grant, revoke, system, gvisor, lwip", action),
    }
    Ok(())
}

pub async fn handle_profile_list() -> Result<()> {
    let profiles = ProfileManager::list_profiles()?;
    println!("\x1b[1;36m{:<2} {:<20} {:<45} {:<15}\x1b[0m", "", "NAME", "URL / PATH", "UPDATED");
    println!("{}", "-".repeat(85));

    for p in profiles {
        let mark = if p.is_active { "\x1b[1;32m●\x1b[0m" } else { " " };
        let url_str = p.url.as_deref().unwrap_or("[Local]");
        let updated_str = p.updated_at.map(|t| format!("{}s ago", t)).unwrap_or_else(|| "Never".to_string());
        println!("{:<2} {:<20} {:<45} {:<15}", mark, p.name, url_str, updated_str);
    }
    Ok(())
}

pub async fn handle_profile_add(name: &str, url: &str) -> Result<()> {
    println!("Downloading subscription for '\x1b[1;36m{}\x1b[0m'...", name);
    ProfileManager::download_profile(name, url).await?;
    println!("\x1b[1;32mSuccessfully saved profile '{}'!\x1b[0m", name);
    Ok(())
}

pub async fn handle_profile_use(client: &MihomoClient, name: &str) -> Result<()> {
    println!("Activating profile '\x1b[1;36m{}\x1b[0m'...", name);
    let dir = ProfileManager::profiles_dir()?;
    let file_path = dir.join(format!("{}.yaml", name));
    if !file_path.exists() {
        anyhow::bail!("Profile '{}' does not exist in ~/.config/mimo/profiles/", name);
    }

    client.reload_config(&file_path.to_string_lossy()).await?;
    ProfileManager::set_active_profile(name)?;
    println!("\x1b[1;32mSuccessfully activated profile '{}'!\x1b[0m", name);
    Ok(())
}

pub async fn handle_profile_del(name: &str) -> Result<()> {
    println!("Deleting profile '\x1b[1;36m{}\x1b[0m'...", name);
    ProfileManager::delete_profile(name)?;
    println!("\x1b[1;32mSuccessfully deleted profile '{}'!\x1b[0m", name);
    Ok(())
}

pub async fn handle_rules_list(client: &MihomoClient) -> Result<()> {
    println!("\x1b[1;36mFetching active rules from Mihomo core...\x1b[0m");
    match client.get_rules().await {
        Ok(resp) => {
            println!("\x1b[1;36m{:<6} {:<20} {:<45} {:<20}\x1b[0m", "INDEX", "TYPE", "PAYLOAD / MATCH RULE", "PROXY TARGET");
            println!("{}", "-".repeat(95));
            for (idx, rule) in resp.rules.iter().enumerate() {
                println!("{:<6} {:<20} {:<45} {:<20}", format!("#{}", idx + 1), rule.rule_type, rule.payload, rule.proxy);
            }
        }
        Err(e) => println!("Error fetching rules: {}", e),
    }
    Ok(())
}

pub async fn handle_proxy_list(client: &MihomoClient, group_filter: Option<&str>) -> Result<()> {
    let resp = client.get_proxies().await?;

    println!("\x1b[1;36m{:<25} {:<12} {:<25} {:<10}\x1b[0m", "NAME", "TYPE", "CURRENT SELECT", "LATENCY");
    println!("{}", "-".repeat(75));

    let mut keys: Vec<&String> = resp.proxies.keys().collect();
    keys.sort();

    for key in keys {
        let proxy = &resp.proxies[key];

        if let Some(filter) = group_filter
            && !proxy.name.eq_ignore_ascii_case(filter) {
                continue;
            }

        let delay_str = match proxy.last_delay() {
            Some(ms) if ms < 200 => format!("\x1b[32m{} ms\x1b[0m", ms),
            Some(ms) if ms < 500 => format!("\x1b[33m{} ms\x1b[0m", ms),
            Some(ms) => format!("\x1b[31m{} ms\x1b[0m", ms),
            None => "\x1b[90m-\x1b[0m".to_string(),
        };

        let now_str = proxy.now.as_deref().unwrap_or("-");

        println!(
            "{:<25} {:<12} {:<25} {:<10}",
            proxy.name, proxy.proxy_type, now_str, delay_str
        );
    }

    Ok(())
}

pub async fn handle_proxy_use(client: &MihomoClient, group: &str, node: &str) -> Result<()> {
    println!("Switching proxy in group '\x1b[1;33m{}\x1b[0m' to '\x1b[1;32m{}\x1b[0m'...", group, node);
    client.select_proxy(group, node).await?;
    println!("\x1b[1;32mSuccessfully switched!\x1b[0m");
    Ok(())
}

pub async fn handle_latency(client: &MihomoClient, node: Option<&str>) -> Result<()> {
    if let Some(target) = node {
        println!("Testing delay for node '\x1b[1;36m{}\x1b[0m'...", target);
        match client.test_delay(target, None, Some(3000)).await {
            Ok(ms) => println!("Node '\x1b[1;36m{}\x1b[0m': \x1b[1;32m{} ms\x1b[0m", target, ms),
            Err(e) => println!("Node '\x1b[1;36m{}\x1b[0m': \x1b[1;31mTimeout / Error ({})\x1b[0m", target, e),
        }
    } else {
        println!("Testing delay for active proxy groups...");
        let resp = client.get_proxies().await?;
        for (name, proxy) in resp.proxies {
            if (proxy.proxy_type == "Selector" || proxy.proxy_type == "URLTest")
                && let Some(nodes) = &proxy.all {
                    println!("\x1b[1;33mGroup: {}\x1b[0m", name);
                    if let Ok(delays) = client.test_group_delay(&name, None, Some(3000)).await {
                        for node_name in nodes {
                            if let Some(ms) = delays.get(node_name) {
                                println!("  ├─ {:<25} : \x1b[32m{} ms\x1b[0m", node_name, ms);
                            } else {
                                println!("  ├─ {:<25} : \x1b[31mTimeout / Offline\x1b[0m", node_name);
                            }
                        }
                    } else {
                        for node_name in nodes {
                            match client.test_delay(node_name, None, Some(2000)).await {
                                Ok(ms) => println!("  ├─ {:<25} : \x1b[32m{} ms\x1b[0m", node_name, ms),
                                Err(_) => println!("  ├─ {:<25} : \x1b[31mTimeout\x1b[0m", node_name),
                            }
                        }
                    }
                }
        }
    }
    Ok(())
}

