pub mod action;
pub mod state;

pub use action::Action;
pub use state::{AppState, PaneFocus, Tab};

use crate::api::MihomoClient;
use crate::config::Config;
use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyModifiers};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;

pub struct App {
    pub config: Config,
    pub client: MihomoClient,
    pub state: AppState,
    pub action_tx: mpsc::Sender<Action>,
    pub action_rx: mpsc::Receiver<Action>,
}

impl App {
    pub fn new(config: Config, client: MihomoClient) -> Self {
        let (action_tx, action_rx) = mpsc::channel(250);
        Self {
            config,
            client,
            state: AppState::default(),
            action_tx,
            action_rx,
        }
    }

    /// Spawn async background tasks (WS subscriptions, tick loop, crossterm input)
    pub async fn run_event_loop(&mut self) -> Result<()> {
        let action_tx = self.action_tx.clone();

        // 1. Spawning Tick Loop
        let tick_tx = action_tx.clone();
        let interval_ms = self.config.refresh_interval_ms;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
            loop {
                interval.tick().await;
                if tick_tx.send(Action::Tick).await.is_err() {
                    break;
                }
            }
        });

        // 2. Spawning Terminal Input Stream Loop (Key & Mouse)
        let event_tx = action_tx.clone();
        tokio::spawn(async move {
            let mut reader = EventStream::new();
            while let Some(Ok(event)) = reader.next().await {
                match event {
                    CrosstermEvent::Key(key_event) => {
                        if event_tx.send(Action::Key(key_event)).await.is_err() {
                            break;
                        }
                    }
                    CrosstermEvent::Mouse(mouse_event) => {
                        if event_tx.send(Action::Mouse(mouse_event)).await.is_err() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        });

        // 3. Spawning Traffic WebSocket Stream Loop
        let traffic_tx = action_tx.clone();
        let base_url = self.client.base_url().to_string();
        let secret = self.client.secret().map(String::from);
        tokio::spawn(async move {
            let (tx, mut rx) = mpsc::channel(100);
            let url = base_url.clone();
            let sec = secret.clone();
            let _ = tokio::spawn(async move {
                let _ = crate::api::ws::stream_traffic(&url, sec.as_deref(), tx).await;
            });
            while let Some(msg) = rx.recv().await {
                if traffic_tx.send(Action::TrafficReceived(msg)).await.is_err() {
                    break;
                }
            }
        });

        // 4. Spawning Logs WebSocket Stream Loop
        let logs_tx = action_tx.clone();
        let base_url = self.client.base_url().to_string();
        let secret = self.client.secret().map(String::from);
        tokio::spawn(async move {
            let (tx, mut rx) = mpsc::channel(200);
            let url = base_url.clone();
            let sec = secret.clone();
            let _ = tokio::spawn(async move {
                let _ = crate::api::ws::stream_logs(&url, sec.as_deref(), "info", tx).await;
            });
            while let Some(msg) = rx.recv().await {
                if logs_tx.send(Action::LogReceived(msg)).await.is_err() {
                    break;
                }
            }
        });

        // Initial Data Fetch
        self.fetch_initial_data().await;

        Ok(())
    }

    pub async fn fetch_initial_data(&self) {
        let client = self.client.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            let res = client.get_version().await.map_err(|e| e.to_string());
            let _ = tx.send(Action::VersionFetched(res)).await;
        });

        let client = self.client.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            let res = client.get_config().await.map_err(|e| e.to_string());
            let _ = tx.send(Action::ConfigFetched(res)).await;
        });

        self.fetch_proxies();
        self.fetch_connections();
        self.fetch_profiles();
        self.fetch_rules();
    }

    pub fn fetch_rules(&self) {
        let client = self.client.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            let res = client.get_rules().await.map_err(|e| e.to_string());
            let _ = tx.send(Action::RulesFetched(res)).await;
        });
    }

    pub fn fetch_profiles(&self) {
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            let res = crate::config::profile::ProfileManager::list_profiles()
                .map_err(|e| e.to_string());
            let _ = tx.send(Action::ProfilesFetched(res)).await;
        });
    }

    pub fn fetch_proxies(&self) {
        let client = self.client.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            let res = client.get_proxies().await.map_err(|e| e.to_string());
            let _ = tx.send(Action::ProxiesFetched(res)).await;
        });
    }

    pub fn fetch_connections(&self) {
        let client = self.client.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            let res = client.get_connections().await.map_err(|e| e.to_string());
            let _ = tx.send(Action::ConnectionsFetched(res)).await;
        });
    }

    pub fn next_tab(&mut self) {
        let current_idx = self.state.active_tab as usize;
        let next_idx = (current_idx + 1) % Tab::ALL.len();
        self.state.active_tab = Tab::ALL[next_idx];
    }

    pub fn prev_tab(&mut self) {
        let current_idx = self.state.active_tab as usize;
        let prev_idx = if current_idx == 0 { Tab::ALL.len() - 1 } else { current_idx - 1 };
        self.state.active_tab = Tab::ALL[prev_idx];
    }

    /// Process a single Action and update state
    pub async fn update(&mut self, action: Action) -> Result<bool> {
        self.state.check_toast_expiration();

        match action {
            Action::Quit => return Ok(true), // Exit main loop

            Action::Tick => {
                // Refresh System Metrics (CPU & RAM)
                let mut sys = sysinfo::System::new();
                sys.refresh_cpu_usage();
                sys.refresh_memory();
                self.state.cpu_usage = sys.global_cpu_usage();
                self.state.memory_used_bytes = sys.used_memory();
                self.state.memory_total_bytes = sys.total_memory();

                // Periodic polling for proxies and connections
                self.fetch_proxies();
                if self.state.active_tab == Tab::Connections {
                    self.fetch_connections();
                }
            }

            Action::Key(key) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    return Ok(true);
                }

                // Realtime Search Input Buffer
                if self.state.is_searching {
                    match key.code {
                        KeyCode::Esc => {
                            self.state.is_searching = false;
                            self.state.search_query.clear();
                        }
                        KeyCode::Backspace => {
                            self.state.search_query.pop();
                        }
                        KeyCode::Enter => {
                            self.state.is_searching = false;
                        }
                        KeyCode::Char(c) => {
                            self.state.search_query.push(c);
                        }
                        _ => {}
                    }
                    return Ok(false);
                }

                // Handle Profile Input Modal Typing
                if self.state.show_profile_input {
                    match key.code {
                        KeyCode::Esc => self.state.show_profile_input = false,
                        KeyCode::Tab => {
                            self.state.profile_input_focus = (self.state.profile_input_focus + 1) % 2;
                        }
                        KeyCode::Backspace => {
                            if self.state.profile_input_focus == 0 {
                                self.state.profile_name_input.pop();
                            } else {
                                self.state.profile_url_input.pop();
                            }
                        }
                        KeyCode::Enter => {
                            let name = self.state.profile_name_input.trim().to_string();
                            let url = self.state.profile_url_input.trim().to_string();
                            if !name.is_empty() && !url.is_empty() {
                                self.state.show_profile_input = false;
                                let _ = self.action_tx.try_send(Action::AddProfile { name, url });
                            }
                        }
                        KeyCode::Char(c) => {
                            if self.state.profile_input_focus == 0 {
                                self.state.profile_name_input.push(c);
                            } else {
                                self.state.profile_url_input.push(c);
                            }
                        }
                        _ => {}
                    }
                    return Ok(false);
                }

                if self.state.show_help {
                    if key.code == KeyCode::Char('?') || key.code == KeyCode::Esc {
                        self.state.show_help = false;
                    }
                    return Ok(false);
                }

                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('?') => self.state.show_help = !self.state.show_help,

                    // Tab Navigation with Tab / Shift+Tab
                    KeyCode::Tab => {
                        if self.state.active_tab == Tab::Settings {
                            self.state.settings_focus = (self.state.settings_focus + 1) % 7;
                        } else {
                            self.next_tab();
                        }
                    }
                    KeyCode::BackTab => {
                        if self.state.active_tab == Tab::Settings {
                            self.state.settings_focus = if self.state.settings_focus == 0 { 6 } else { self.state.settings_focus - 1 };
                        } else {
                            self.prev_tab();
                        }
                    }

                    // Direct Tab Jump 1-8
                    KeyCode::Char('1') => self.state.active_tab = Tab::Dashboard,
                    KeyCode::Char('2') => self.state.active_tab = Tab::Proxies,
                    KeyCode::Char('3') => self.state.active_tab = Tab::Profiles,
                    KeyCode::Char('4') => self.state.active_tab = Tab::Rules,
                    KeyCode::Char('5') => self.state.active_tab = Tab::Connections,
                    KeyCode::Char('6') => self.state.active_tab = Tab::Traffic,
                    KeyCode::Char('7') => self.state.active_tab = Tab::Logs,
                    KeyCode::Char('8') => self.state.active_tab = Tab::Settings,

                    // Item Navigation
                    KeyCode::Up | KeyCode::Char('k') => self.move_selection(-1),
                    KeyCode::Down | KeyCode::Char('j') => self.move_selection(1),

                    // Left/Right or Vim h/l
                    KeyCode::Left | KeyCode::Char('h') => {
                        if self.state.active_tab == Tab::Proxies {
                            self.state.focus = PaneFocus::Groups;
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if self.state.active_tab == Tab::Proxies {
                            self.state.focus = PaneFocus::Nodes;
                        }
                    }

                    // Profile View Hotkeys
                    KeyCode::Char('a') if self.state.active_tab == Tab::Profiles => {
                        self.state.profile_name_input.clear();
                        self.state.profile_url_input.clear();
                        self.state.profile_input_focus = 0;
                        self.state.show_profile_input = true;
                    }
                    KeyCode::Char('u') if self.state.active_tab == Tab::Profiles => {
                        if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx) {
                            if let Some(url) = p.url.clone() {
                                let name = p.name.clone();
                                let _ = self.action_tx.try_send(Action::AddProfile { name, url });
                            }
                        }
                    }

                    // Log Buffer Clear
                    KeyCode::Char('c') if self.state.active_tab == Tab::Logs => {
                        let _ = self.action_tx.try_send(Action::ClearLogs);
                    }

                    // Close All Connections
                    KeyCode::Char('D') if self.state.active_tab == Tab::Connections => {
                        let _ = self.action_tx.try_send(Action::CloseAllConnections);
                    }

                    // Key Actions
                    KeyCode::Enter => {
                        if self.state.active_tab == Tab::Settings {
                            if self.state.settings_focus == 6 {
                                let _ = self.action_tx.try_send(Action::SaveSettings);
                            } else {
                                self.state.settings_focus = (self.state.settings_focus + 1) % 7;
                            }
                        } else {
                            self.confirm_selection().await;
                        }
                    }
                    KeyCode::Char(' ') if self.state.active_tab == Tab::Settings => {
                        if self.state.settings_focus == 0 {
                            self.state.settings_lang = if self.state.settings_lang == "zh" { "en".into() } else { "zh".into() };
                        } else if self.state.settings_focus == 3 {
                            self.state.settings_refresh_ms = match self.state.settings_refresh_ms {
                                500 => 1000,
                                1000 => 2000,
                                _ => 500,
                            };
                        }
                    }
                    KeyCode::Char('s') | KeyCode::Char('/') => {
                        self.state.is_searching = !self.state.is_searching;
                        if !self.state.is_searching {
                            self.state.search_query.clear();
                        }
                    }
                    KeyCode::Char('t') => self.test_selected_group_latency(),
                    KeyCode::Char('m') => self.cycle_mode(),
                    KeyCode::Char('p') => {
                        let _ = self.action_tx.try_send(Action::ToggleSystemProxy);
                    }
                    KeyCode::Char('x') => {
                        let _ = self.action_tx.try_send(Action::ToggleTunMode);
                    }
                    KeyCode::Char('r') => {
                        let _ = self.action_tx.try_send(Action::RestartCore);
                    }
                    KeyCode::Char('d') => {
                        if self.state.active_tab == Tab::Profiles {
                            if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx) {
                                let name = p.name.clone();
                                let _ = self.action_tx.try_send(Action::DeleteProfile(name));
                            }
                        } else if self.state.active_tab == Tab::Connections {
                            self.close_selected_connection().await;
                        }
                    }

                    _ => {}
                }
            }

            Action::Mouse(mouse) => {
                use crossterm::event::{MouseButton, MouseEventKind};
                match mouse.kind {
                    MouseEventKind::ScrollDown => self.move_selection(1),
                    MouseEventKind::ScrollUp => self.move_selection(-1),
                    MouseEventKind::Down(MouseButton::Left) => {
                        let col = mouse.column;
                        let row = mouse.row;

                        // 1. Click on Left Sidebar Navigation Bar (Column < 18)
                        if col < 18 {
                            if row >= 4 && row <= 11 {
                                let tab_idx = (row - 4) as usize;
                                if let Some(tab) = Tab::ALL.get(tab_idx) {
                                    self.state.active_tab = *tab;
                                }
                            }
                        }
                        // 2. Click on Top Control Pills Bar (Row <= 2, Column >= 18)
                        else if row <= 2 {
                            if col >= 18 && col < 48 {
                                self.cycle_mode();
                            } else if col >= 48 && col < 73 {
                                let _ = self.action_tx.try_send(Action::ToggleSystemProxy);
                            } else if col >= 73 && col < 93 {
                                let _ = self.action_tx.try_send(Action::ToggleTunMode);
                            }
                        }
                        // 3. Tab Specific Workspace Clicks
                        else if self.state.active_tab == Tab::Settings {
                            if row >= 3 && row <= 5 {
                                self.state.settings_focus = 0;
                                self.state.settings_lang = if self.state.settings_lang == "zh" { "en".into() } else { "zh".into() };
                            } else if row >= 6 && row <= 8 {
                                self.state.settings_focus = 1;
                            } else if row >= 9 && row <= 11 {
                                self.state.settings_focus = 2;
                            } else if row >= 12 && row <= 14 {
                                self.state.settings_focus = 3;
                                self.state.settings_refresh_ms = match self.state.settings_refresh_ms {
                                    500 => 1000,
                                    1000 => 2000,
                                    _ => 500,
                                };
                            } else if row >= 15 && row <= 17 {
                                self.state.settings_focus = 4;
                            } else if row >= 18 && row <= 20 {
                                self.state.settings_focus = 5;
                            } else if row >= 21 {
                                self.state.settings_focus = 6;
                                let _ = self.action_tx.try_send(Action::SaveSettings);
                            }
                        } else if self.state.active_tab == Tab::Profiles {
                            if row <= 5 {
                                if col >= 18 && col < 35 {
                                    self.state.profile_name_input.clear();
                                    self.state.profile_url_input.clear();
                                    self.state.profile_input_focus = 0;
                                    self.state.show_profile_input = true;
                                } else if col >= 35 && col < 52 {
                                    if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx) {
                                        if let Some(url) = p.url.clone() {
                                            let name = p.name.clone();
                                            let _ = self.action_tx.try_send(Action::AddProfile { name, url });
                                        }
                                    }
                                } else if col >= 52 && col < 68 {
                                    if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx) {
                                        let name = p.name.clone();
                                        let _ = self.action_tx.try_send(Action::DeleteProfile(name));
                                    }
                                }
                            } else {
                                let click_idx = (row - 6) as usize;
                                if click_idx < self.state.profiles.len() {
                                    self.state.selected_profile_idx = click_idx;
                                    self.confirm_selection().await;
                                }
                            }
                        } else if self.state.active_tab == Tab::Logs {
                            if row <= 5 {
                                if col >= 30 && col < 36 {
                                    let _ = self.action_tx.try_send(Action::SetLogFilter("all".into()));
                                } else if col >= 37 && col < 43 {
                                    let _ = self.action_tx.try_send(Action::SetLogFilter("info".into()));
                                } else if col >= 44 && col < 50 {
                                    let _ = self.action_tx.try_send(Action::SetLogFilter("warn".into()));
                                } else if col >= 51 && col < 57 {
                                    let _ = self.action_tx.try_send(Action::SetLogFilter("error".into()));
                                } else if col >= 58 && col < 64 {
                                    let _ = self.action_tx.try_send(Action::SetLogFilter("debug".into()));
                                } else if col >= 65 {
                                    let _ = self.action_tx.try_send(Action::ClearLogs);
                                }
                            }
                        } else if self.state.active_tab == Tab::Proxies {
                            if col < 45 {
                                self.state.focus = PaneFocus::Groups;
                                if row >= 4 {
                                    let click_idx = (row - 4) as usize;
                                    if click_idx < self.state.proxy_groups.len() {
                                        self.state.selected_group_idx = click_idx;
                                        self.state.selected_node_idx = 0;
                                    }
                                }
                            } else {
                                self.state.focus = PaneFocus::Nodes;
                                if row >= 4 {
                                    let click_idx = (row - 4) as usize;
                                    let nodes = self.state.current_group_nodes();
                                    if click_idx < nodes.len() {
                                        self.state.selected_node_idx = click_idx;
                                        self.confirm_selection().await;
                                    }
                                }
                            }
                        } else if self.state.active_tab == Tab::Rules {
                            if row >= 4 {
                                let click_idx = (row - 4) as usize;
                                if let Some(resp) = &self.state.rules_resp {
                                    if click_idx < resp.rules.len() {
                                        self.state.selected_rule_idx = click_idx;
                                    }
                                }
                            }
                        } else if self.state.active_tab == Tab::Connections {
                            if row >= 2 {
                                let click_idx = (row - 2) as usize;
                                if let Some(resp) = &self.state.connections_resp {
                                    if click_idx < resp.connections.len() {
                                        self.state.selected_conn_idx = click_idx;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            Action::NextTab => self.next_tab(),
            Action::PrevTab => self.prev_tab(),
            Action::SelectTab(idx) => {
                if let Some(tab) = Tab::ALL.get(idx) {
                    self.state.active_tab = *tab;
                }
            }

            Action::VersionFetched(res) => match res {
                Ok(v) => self.state.version = Some(v),
                Err(e) => self.state.status_error = Some(e),
            },

            Action::RulesFetched(res) => match res {
                Ok(resp) => {
                    self.state.rules_resp = Some(resp);
                    if let Some(r) = &self.state.rules_resp {
                        if !r.rules.is_empty() && self.state.selected_rule_idx >= r.rules.len() {
                            self.state.selected_rule_idx = r.rules.len() - 1;
                        }
                    }
                }
                Err(e) => self.state.status_error = Some(e),
            },

            Action::ProfilesFetched(res) => match res {
                Ok(list) => {
                    self.state.profiles = list;
                    if !self.state.profiles.is_empty() && self.state.selected_profile_idx >= self.state.profiles.len() {
                        self.state.selected_profile_idx = self.state.profiles.len() - 1;
                    }
                }
                Err(e) => self.state.push_toast(format!("Profile Error: {}", e)),
            },

            Action::AddProfile { name, url } => {
                self.state.push_toast(format!("Downloading profile '{}'...", name));
                let tx = self.action_tx.clone();
                tokio::spawn(async move {
                    match crate::config::profile::ProfileManager::download_profile(&name, &url).await {
                        Ok(_) => {
                            let _ = tx.send(Action::FetchProfiles).await;
                        }
                        Err(e) => {
                            let _ = tx.send(Action::ProfilesFetched(Err(format!("Download failed: {}", e)))).await;
                        }
                    }
                });
            }

            Action::ActivateProfile(name) => {
                self.state.push_toast(format!("Activating profile '{}'...", name));
                let client = self.client.clone();
                let tx = self.action_tx.clone();
                let target_name = name.clone();
                tokio::spawn(async move {
                    if let Ok(dir) = crate::config::profile::ProfileManager::profiles_dir() {
                        let path = dir.join(format!("{}.yaml", target_name));
                        if path.exists() {
                            let path_str = path.to_string_lossy().to_string();
                            if client.reload_config(&path_str).await.is_ok() {
                                let _ = crate::config::profile::ProfileManager::set_active_profile(&target_name);
                                let _ = tx.send(Action::FetchProfiles).await;
                            }
                        }
                    }
                });
            }

            Action::DeleteProfile(name) => {
                let _ = crate::config::profile::ProfileManager::delete_profile(&name);
                self.fetch_profiles();
                self.state.push_toast(format!("Deleted profile '{}'", name));
            }

            Action::CloseAllConnections => {
                self.state.push_toast("Closing all connections...".to_string());
                let client = self.client.clone();
                let tx = self.action_tx.clone();
                tokio::spawn(async move {
                    if client.close_all_connections().await.is_ok() {
                        let _ = tx.send(Action::FetchConnections).await;
                    }
                });
            }

            Action::ClearLogs => {
                self.state.logs.clear();
                self.state.push_toast("Log buffer cleared".to_string());
            }

            Action::SetLogFilter(filter) => {
                self.state.log_filter = filter;
            }

            Action::FetchProfiles => {
                self.fetch_profiles();
            }

            Action::FetchConnections => {
                self.fetch_connections();
            }

            Action::ToggleSystemProxy => {
                let http_port = self.state.config.as_ref().and_then(|c| c.mixed_port.or(c.http_port)).unwrap_or(7890);
                let socks_port = self.state.config.as_ref().and_then(|c| c.mixed_port.or(c.socks_port)).unwrap_or(7891);
                if self.state.is_sysproxy_enabled {
                    if crate::core::SystemProxy::disable().is_ok() {
                        self.state.is_sysproxy_enabled = false;
                        self.state.push_toast("System Proxy Disabled".to_string());
                    }
                } else {
                    if crate::core::SystemProxy::enable(http_port, socks_port).is_ok() {
                        self.state.is_sysproxy_enabled = true;
                        self.state.push_toast("System Proxy Enabled".to_string());
                    } else {
                        self.state.push_toast("Failed to set System Proxy (GSettings)".to_string());
                    }
                }
            }

            Action::ToggleTunMode => {
                let target = !self.state.is_tun_enabled;
                let client = self.client.clone();
                let tx = self.action_tx.clone();
                self.state.push_toast(format!("Toggling TUN mode to {}...", target));
                tokio::spawn(async move {
                    if client.set_tun_enabled(target).await.is_ok() {
                        if let Ok(c) = client.get_config().await {
                            let _ = tx.send(Action::ConfigFetched(Ok(c))).await;
                        }
                    }
                });
            }

            Action::SaveSettings => {
                let mut cfg = crate::config::Config::load().unwrap_or_default();
                cfg.language = self.state.settings_lang.clone();
                cfg.api_url = self.state.settings_api_url.clone();
                cfg.secret = if self.state.settings_secret.trim().is_empty() { None } else { Some(self.state.settings_secret.clone()) };
                cfg.refresh_interval_ms = self.state.settings_refresh_ms;
                cfg.http_port = self.state.settings_http_port;
                cfg.socks_port = self.state.settings_socks_port;
                cfg.test_url = self.state.settings_test_url.clone();

                if cfg.save().is_ok() {
                    let msg = if cfg.language == "zh" { "配置已成功保存至 ~/.config/mimo/config.toml" } else { "Settings saved to ~/.config/mimo/config.toml" };
                    self.state.push_toast(msg.to_string());
                } else {
                    self.state.push_toast("Failed to save config file".to_string());
                }
            }

            Action::RestartCore => {
                self.state.push_toast("Restarting Mihomo Core...".to_string());
                let tx = self.action_tx.clone();
                tokio::spawn(async move {
                    let _ = crate::core::CoreProcess::restart();
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    let _ = tx.send(Action::Tick).await;
                });
            }

            Action::ConfigFetched(res) => match res {
                Ok(c) => self.state.config = Some(c),
                Err(e) => self.state.status_error = Some(e),
            },

            Action::ProxiesFetched(res) => match res {
                Ok(resp) => {
                    // Extract sorted proxy groups (Selectors / URLTest)
                    let mut groups: Vec<String> = resp
                        .proxies
                        .iter()
                        .filter(|(_, item)| item.proxy_type == "Selector" || item.proxy_type == "URLTest" || item.name == "GLOBAL")
                        .map(|(k, _)| k.clone())
                        .collect();
                    groups.sort();

                    self.state.proxy_groups = groups;
                    self.state.proxies_resp = Some(resp);

                    // Clamp group and node indexes
                    if !self.state.proxy_groups.is_empty() {
                        if self.state.selected_group_idx >= self.state.proxy_groups.len() {
                            self.state.selected_group_idx = self.state.proxy_groups.len() - 1;
                        }
                        let nodes = self.state.current_group_nodes();
                        if !nodes.is_empty() && self.state.selected_node_idx >= nodes.len() {
                            self.state.selected_node_idx = nodes.len() - 1;
                        }
                    }
                }
                Err(e) => self.state.status_error = Some(e),
            },

            Action::ConnectionsFetched(res) => match res {
                Ok(resp) => {
                    self.state.connections_resp = Some(resp);
                    if let Some(c) = &self.state.connections_resp {
                        if !c.connections.is_empty() && self.state.selected_conn_idx >= c.connections.len() {
                            self.state.selected_conn_idx = c.connections.len() - 1;
                        }
                    }
                }
                Err(e) => self.state.status_error = Some(e),
            },

            Action::TrafficReceived(msg) => {
                self.state.current_traffic = msg.clone();
                self.state.up_history.pop_front();
                self.state.up_history.push_back(msg.up);
                self.state.down_history.pop_front();
                self.state.down_history.push_back(msg.down);
            }

            Action::LogReceived(log) => {
                if self.state.logs.len() >= 500 {
                    self.state.logs.pop_front();
                }
                self.state.logs.push_back(log);
            }

            Action::LatencyResult { node, result } => {
                match result {
                    Ok(ms) => {
                        self.state.latency_map.insert(node.clone(), Some(ms));
                    }
                    Err(_) => {
                        self.state.latency_map.insert(node, None);
                    }
                }
            }

            Action::ChangeMode(mode) => {
                let client = self.client.clone();
                let tx = self.action_tx.clone();
                tokio::spawn(async move {
                    if let Ok(()) = client.set_mode(&mode).await {
                        if let Ok(c) = client.get_config().await {
                            let _ = tx.send(Action::ConfigFetched(Ok(c))).await;
                        }
                    }
                });
            }

            _ => {}
        }

        Ok(false)
    }

    fn move_selection(&mut self, delta: i32) {
        match self.state.active_tab {
            Tab::Proxies => match self.state.focus {
                PaneFocus::Groups => {
                    let len = self.state.proxy_groups.len();
                    if len > 0 {
                        let new_idx = (self.state.selected_group_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                        if new_idx != self.state.selected_group_idx {
                            self.state.selected_group_idx = new_idx;
                            self.state.selected_node_idx = 0; // Reset node selection on group change
                        }
                    }
                }
                PaneFocus::Nodes => {
                    let nodes = self.state.current_group_nodes();
                    let len = nodes.len();
                    if len > 0 {
                        self.state.selected_node_idx = (self.state.selected_node_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                    }
                }
                _ => {}
            },
            Tab::Profiles => {
                let len = self.state.profiles.len();
                if len > 0 {
                    self.state.selected_profile_idx = (self.state.selected_profile_idx as i32 + delta)
                        .clamp(0, len as i32 - 1) as usize;
                }
            }
            Tab::Rules => {
                if let Some(resp) = &self.state.rules_resp {
                    let len = resp.rules.len();
                    if len > 0 {
                        self.state.selected_rule_idx = (self.state.selected_rule_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                    }
                }
            }
            Tab::Connections => {
                if let Some(resp) = &self.state.connections_resp {
                    let len = resp.connections.len();
                    if len > 0 {
                        self.state.selected_conn_idx = (self.state.selected_conn_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                    }
                }
            }
            Tab::Logs => {
                let len = self.state.logs.len();
                if len > 0 {
                    self.state.log_scroll = (self.state.log_scroll as i32 + delta)
                        .clamp(0, len as i32 - 1) as usize;
                }
            }
            _ => {}
        }
    }

    async fn confirm_selection(&mut self) {
        if self.state.active_tab == Tab::Profiles {
            if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx) {
                let name = p.name.clone();
                let _ = self.action_tx.try_send(Action::ActivateProfile(name));
            }
        } else if self.state.active_tab == Tab::Proxies {
            if let Some(group) = self.state.selected_group_name().map(String::from) {
                let nodes = self.state.current_group_nodes();
                if let Some(node) = nodes.get(self.state.selected_node_idx).cloned() {
                    let client = self.client.clone();
                    let tx = self.action_tx.clone();
                    self.state.push_toast(format!("Selecting '{}' in '{}'", node, group));
                    tokio::spawn(async move {
                        if client.select_proxy(&group, &node).await.is_ok() {
                            if let Ok(resp) = client.get_proxies().await {
                                let _ = tx.send(Action::ProxiesFetched(Ok(resp))).await;
                            }
                        }
                    });
                }
            }
        }
    }

    fn test_selected_group_latency(&mut self) {
        if self.state.active_tab == Tab::Proxies {
            let nodes = self.state.current_group_nodes();
            if nodes.is_empty() {
                return;
            }
            self.state.push_toast("Testing latencies...".to_string());
            let client = self.client.clone();
            let tx = self.action_tx.clone();
            tokio::spawn(async move {
                for node in nodes {
                    let res = client.test_delay(&node, None, Some(2000)).await.map_err(|e| e.to_string());
                    let _ = tx.send(Action::LatencyResult { node, result: res }).await;
                }
            });
        }
    }

    fn cycle_mode(&mut self) {
        if let Some(config) = &self.state.config {
            let current = config.mode.as_deref().unwrap_or("Rule");
            let next_mode = match current {
                "Rule" => "Global",
                "Global" => "Direct",
                _ => "Rule",
            };
            self.state.push_toast(format!("Switching mode to {}", next_mode));
            let client = self.client.clone();
            let tx = self.action_tx.clone();
            let target_mode = next_mode.to_string();
            tokio::spawn(async move {
                if let Ok(()) = client.set_mode(&target_mode).await {
                    if let Ok(c) = client.get_config().await {
                        let _ = tx.send(Action::ConfigFetched(Ok(c))).await;
                    }
                }
            });
        }
    }

    async fn close_selected_connection(&mut self) {
        if self.state.active_tab == Tab::Connections {
            if let Some(resp) = &self.state.connections_resp {
                if let Some(conn) = resp.connections.get(self.state.selected_conn_idx) {
                    let id = conn.id.clone();
                    let client = self.client.clone();
                    self.state.push_toast(format!("Closing connection {}", &id[..8.min(id.len())]));
                    tokio::spawn(async move {
                        let _ = client.close_connection(&id).await;
                    });
                }
            }
        }
    }
}
