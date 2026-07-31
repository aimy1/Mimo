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
    match action {
        "start" => {
            println!("Starting Mihomo core service...");
            let index = ProfileManager::load_index()?;
            if let Some(active) = index.active_profile {
                let dir = ProfileManager::profiles_dir()?;
                let path = dir.join(format!("{}.yaml", active));
                CoreProcess::start_with_config(&path)?;
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

pub async fn handle_tun_toggle(client: &MihomoClient, enable: bool) -> Result<()> {
    println!("Setting TUN mode to {}...", enable);
    client.set_tun_enabled(enable).await?;
    println!("\x1b[32mTUN mode updated successfully.\x1b[0m");
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

pub async fn handle_proxy_list(client: &MihomoClient, group_filter: Option<&str>) -> Result<()> {
    let resp = client.get_proxies().await?;

    println!("\x1b[1;36m{:<25} {:<12} {:<25} {:<10}\x1b[0m", "NAME", "TYPE", "CURRENT SELECT", "LATENCY");
    println!("{}", "-".repeat(75));

    let mut keys: Vec<&String> = resp.proxies.keys().collect();
    keys.sort();

    for key in keys {
        let proxy = &resp.proxies[key];

        if let Some(filter) = group_filter {
            if !proxy.name.eq_ignore_ascii_case(filter) {
                continue;
            }
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
            if proxy.proxy_type == "Selector" || proxy.proxy_type == "URLTest" {
                if let Some(nodes) = &proxy.all {
                    println!("\x1b[1;33mGroup: {}\x1b[0m", name);
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
