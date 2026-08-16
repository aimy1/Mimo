use crate::api::MihomoClient;
use crate::app::action::Action;
use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyModifiers};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;

pub mod action;
pub mod state;

pub use state::{AppState, FocusZone, ProxySubFocus, Tab};
use futures_util::StreamExt;

pub struct App {
    pub state: AppState,
    pub client: MihomoClient,
    pub action_tx: mpsc::Sender<Action>,
    pub action_rx: mpsc::Receiver<Action>,
}

impl App {
    pub fn new() -> Result<Self> {
        let state = AppState::default();
        let client = MihomoClient::new(&state.settings_api_url, if state.settings_secret.is_empty() { None } else { Some(state.settings_secret.clone()) })?;
        let (action_tx, action_rx) = mpsc::channel(100);

        Ok(Self {
            state,
            client,
            action_tx,
            action_rx,
        })
    }

    pub async fn run(&mut self, terminal: &mut ratatui::Terminal<impl ratatui::backend::Backend>) -> Result<()> {
        let mut reader = EventStream::new();
        let mut tick_interval = interval(Duration::from_millis(self.state.settings_refresh_ms));

        // Start WebSocket Background Listeners
        let ws_tx = self.action_tx.clone();
        let ws_url = self.client.base_url().to_string();
        let ws_secret = self.client.secret().map(|s| s.to_string());
        tokio::spawn(async move {
            let (tx, mut rx) = mpsc::channel(100);
            tokio::spawn(async move {
                let _ = crate::api::ws::stream_traffic(&ws_url, ws_secret.as_deref(), tx).await;
            });
            while let Some(msg) = rx.recv().await {
                let _ = ws_tx.send(Action::TrafficReceived(msg)).await;
            }
        });

        let ws_log_tx = self.action_tx.clone();
        let ws_log_url = self.client.base_url().to_string();
        let ws_log_secret = self.client.secret().map(|s| s.to_string());
        tokio::spawn(async move {
            let (tx, mut rx) = mpsc::channel(500);
            tokio::spawn(async move {
                let _ = crate::api::ws::stream_logs(&ws_log_url, ws_log_secret.as_deref(), "info", tx).await;
            });
            while let Some(msg) = rx.recv().await {
                let _ = ws_log_tx.send(Action::LogReceived(msg)).await;
            }
        });

        // Initial Data Fetch
        self.fetch_version();
        self.fetch_config();
        self.fetch_proxies();
        self.fetch_profiles();
        self.fetch_rules();
        self.fetch_connections();
        let _ = self.action_tx.try_send(Action::FetchOutboundIp);

        loop {
            tokio::select! {
                _ = tick_interval.tick() => {
                    if self.action_tx.send(Action::Tick).await.is_err() {
                        break;
                    }
                }
                maybe_event = reader.next() => {
                    match maybe_event {
                        Some(Ok(CrosstermEvent::Key(key))) => {
                            if key.kind == crossterm::event::KeyEventKind::Press
                                && self.action_tx.send(Action::Key(key)).await.is_err() {
                                    break;
                                }
                        }
                        Some(Ok(CrosstermEvent::Mouse(mouse))) => {
                            if self.action_tx.send(Action::Mouse(mouse)).await.is_err() {
                                break;
                            }
                        }
                        Some(Ok(_)) => {}
                        Some(Err(e)) => {
                            self.state.push_toast(format!("Input error: {}", e));
                        }
                        None => break,
                    }
                }
                action = self.action_rx.recv() => {
                    if let Some(action) = action {
                        if matches!(action, Action::ClearScreen) {
                            let _ = terminal.clear();
                        }
                        let should_quit = self.update(action).await?;
                        if should_quit {
                            break;
                        }
                    }
                }
            }

            terminal.draw(|f| crate::ui::render(f, &self.state))?;
        }

        Ok(())
    }

    pub fn fetch_version(&self) {
        let client = self.client.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            let res = client.get_version().await.map_err(|e| e.to_string());
            let _ = tx.send(Action::VersionFetched(res)).await;
        });
    }

    pub fn fetch_config(&self) {
        let client = self.client.clone();
        let tx = self.action_tx.clone();
        tokio::spawn(async move {
            let res = client.get_config().await.map_err(|e| e.to_string());
            let _ = tx.send(Action::ConfigFetched(res)).await;
        });
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
            let res = crate::profile::ProfileManager::list_profiles()
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

                // Refresh TUN Privilege & Interface status
                self.state.is_tun_privileged = crate::core::TunMode::check_privilege();
                let (iface, is_up) = crate::core::TunMode::get_interface_info();
                self.state.tun_interface_name = iface;
                self.state.is_tun_interface_up = is_up;

                // Periodic polling for proxies and connections
                self.fetch_proxies();
                if self.state.active_tab == Tab::Connections {
                    self.fetch_connections();
                }
            }

            Action::Key(key) => {
                // Layer 0: Global Application Quit & Screen Refresh
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    return Ok(true);
                }
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('l') {
                    let _ = self.action_tx.try_send(Action::ClearScreen);
                    return Ok(false);
                }

                // Layer 1: Active Input Modal Processing (Search Bar or Subscription Input)
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
                            self.confirm_selection().await;
                        }
                        KeyCode::Up => self.move_selection(-1),
                        KeyCode::Down => self.move_selection(1),
                        KeyCode::Char(c) => {
                            self.state.search_query.push(c);
                        }
                        _ => {}
                    }
                    return Ok(false);
                }

                if self.state.show_profile_input {
                    match key.code {
                        KeyCode::Esc => self.state.show_profile_input = false,
                        KeyCode::Tab | KeyCode::Down => {
                            self.state.profile_input_focus = (self.state.profile_input_focus + 1) % 2;
                        }
                        KeyCode::Up => {
                            self.state.profile_input_focus = if self.state.profile_input_focus == 0 { 1 } else { 0 };
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
                            if self.state.profile_input_focus == 0 {
                                if !name.is_empty() {
                                    self.state.profile_input_focus = 1;
                                }
                            } else {
                                if !name.is_empty() && !url.is_empty() {
                                    self.state.show_profile_input = false;
                                    let _ = self.action_tx.try_send(Action::AddProfile { name, url });
                                }
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

                // Layer 2: Help Overlay Processing
                if self.state.show_help {
                    if key.code == KeyCode::Char('?') || key.code == KeyCode::Esc {
                        self.state.show_help = false;
                    }
                    return Ok(false);
                }

                // Layer 2.5: TUN Privilege Modal Processing
                if self.state.show_tun_modal {
                    if !self.state.is_granting_privilege {
                        match key.code {
                            KeyCode::Esc => {
                                let _ = self.action_tx.try_send(Action::HideTunModal);
                            }
                            KeyCode::Tab => {
                                self.state.tun_input_focus = (self.state.tun_input_focus + 1) % 3;
                            }
                            KeyCode::Backspace => {
                                self.state.tun_password_input.pop();
                            }
                            KeyCode::Enter => {
                                if !self.state.tun_password_input.is_empty() {
                                    let pass = self.state.tun_password_input.clone();
                                    let _ = self.action_tx.try_send(Action::GrantTunPrivilegeWithPassword(pass));
                                } else if self.state.tun_input_focus == 2 {
                                    let _ = self.action_tx.try_send(Action::GrantTunPrivilegePkexec);
                                }
                            }
                            KeyCode::Char('p') | KeyCode::Char('P') => {
                                let _ = self.action_tx.try_send(Action::GrantTunPrivilegePkexec);
                            }
                            KeyCode::Char(c) => {
                                self.state.tun_password_input.push(c);
                            }
                            _ => {}
                        }
                    }
                    return Ok(false);
                }

                // Layer 3: Global Hotkeys, Ctrl Commands & Tab Switching
                if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('c') => return Ok(true),

                        KeyCode::Char('d') => { self.move_selection(5); return Ok(false); }
                        KeyCode::Char('u') => { self.move_selection(-5); return Ok(false); }
                        KeyCode::Char('s') => {
                            if self.state.active_tab == Tab::Settings {
                                let _ = self.action_tx.try_send(Action::SaveSettings);
                            }
                            return Ok(false);
                        }
                        _ => {}
                    }
                }

                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('?') => self.state.show_help = !self.state.show_help,

                    // Universal Esc: Reset Search & Dialogs & Reset Focus
                    KeyCode::Esc => {
                        self.state.is_searching = false;
                        self.state.search_query.clear();
                        self.state.focus_zone = FocusZone::Workspace;
                    }

                    // Direct Tab Switch 1-9 & 0
                    KeyCode::Char('1') => self.state.active_tab = Tab::Dashboard,
                    KeyCode::Char('2') => self.state.active_tab = Tab::Proxies,
                    KeyCode::Char('3') => self.state.active_tab = Tab::Profiles,
                    KeyCode::Char('4') => self.state.active_tab = Tab::Rules,
                    KeyCode::Char('5') => self.state.active_tab = Tab::Connections,
                    KeyCode::Char('6') => self.state.active_tab = Tab::Traffic,
                    KeyCode::Char('7') => self.state.active_tab = Tab::Logs,
                    KeyCode::Char('8') => self.state.active_tab = Tab::Settings,
                    KeyCode::Char('9') => self.state.active_tab = Tab::Privileges,
                    KeyCode::Char('0') => self.state.active_tab = Tab::About,

                    // Tab / Shift+Tab / [ / ] Navigation
                    KeyCode::Tab | KeyCode::Char(']') => {
                        if self.state.active_tab == Tab::Settings {
                            self.state.settings_focus = (self.state.settings_focus + 1) % 20;
                        } else {
                            self.next_tab();
                        }
                    }
                    KeyCode::BackTab | KeyCode::Char('[') => {
                        if self.state.active_tab == Tab::Settings {
                            self.state.settings_focus = if self.state.settings_focus == 0 { 19 } else { self.state.settings_focus - 1 };
                        } else {
                            self.prev_tab();
                        }
                    }

                    // Global Core Control Hotkeys
                    KeyCode::Char('m') | KeyCode::Char('M') => self.cycle_mode(),
                    KeyCode::Char('p') | KeyCode::Char('P') if self.state.active_tab != Tab::Privileges => { let _ = self.action_tx.try_send(Action::ToggleSystemProxy); }
                    KeyCode::Char('x') | KeyCode::Char('X') => { let _ = self.action_tx.try_send(Action::ToggleTunMode); }
                    KeyCode::Char('r') | KeyCode::Char('R') if self.state.active_tab != Tab::Privileges => { let _ = self.action_tx.try_send(Action::RestartCore); }

                    // Layer 4: View-Specific & Directional Key Navigation
                    _ => match self.state.focus_zone {
                        FocusZone::Sidebar => match key.code {
                            KeyCode::Up | KeyCode::Char('k') => self.prev_tab(),
                            KeyCode::Down | KeyCode::Char('j') => self.next_tab(),
                            KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter => {
                                self.state.focus_zone = FocusZone::Workspace;
                            }
                            _ => {}
                        },
                        FocusZone::Workspace => {
                            // Specialized Input Handling for Settings View
                            if self.state.active_tab == Tab::Settings {
                                match key.code {
                                    KeyCode::Up | KeyCode::Char('k') => self.state.settings_focus = if self.state.settings_focus == 0 { 19 } else { self.state.settings_focus - 1 },
                                    KeyCode::Down | KeyCode::Char('j') => self.state.settings_focus = (self.state.settings_focus + 1) % 20,
                                    KeyCode::Left | KeyCode::Char('h') => {
                                        if self.state.settings_focus >= 11 && self.state.settings_focus <= 18 {
                                            self.state.settings_focus -= 11;
                                        } else {
                                            self.state.focus_zone = FocusZone::Sidebar;
                                        }
                                    }
                                    KeyCode::Right | KeyCode::Char('l') => {
                                        if self.state.settings_focus <= 10 {
                                            self.state.settings_focus = (self.state.settings_focus + 11).min(18);
                                        }
                                    }
                                    KeyCode::Enter => {
                                        if self.state.settings_focus == 15 {
                                            let _ = self.action_tx.try_send(Action::DownloadCore);
                                        } else if self.state.settings_focus == 19 {
                                            let _ = self.action_tx.try_send(Action::SaveSettings);
                                        } else {
                                            self.state.settings_focus = (self.state.settings_focus + 1) % 20;
                                        }
                                    }
                                    KeyCode::Char(' ') => {
                                        match self.state.settings_focus {
                                            0 => self.state.settings_api_url.push(' '),
                                            1 => self.state.settings_secret.push(' '),
                                            5 => self.state.settings_test_url.push(' '),
                                            6 => self.state.settings_dns_mode = if self.state.settings_dns_mode == "fake-ip" { "redir-host".into() } else { "fake-ip".into() },
                                            7 => self.state.settings_sniffing = !self.state.settings_sniffing,
                                            8 => self.state.settings_tcp_concurrent = !self.state.settings_tcp_concurrent,
                                            9 => self.state.settings_auto_sysproxy = !self.state.settings_auto_sysproxy,
                                            10 => self.state.settings_sub_update_hours = match self.state.settings_sub_update_hours {
                                                0 => 6,
                                                6 => 12,
                                                12 => 24,
                                                _ => 0,
                                            },
                                            11 => self.state.settings_tun_stack = match self.state.settings_tun_stack.as_str() {
                                                "system" => "gvisor".into(),
                                                "gvisor" => "lwip".into(),
                                                _ => "system".into(),
                                            },
                                            12 => self.state.settings_log_level = match self.state.settings_log_level.as_str() {
                                                "info" => "warning".into(),
                                                "warning" => "error".into(),
                                                "error" => "debug".into(),
                                                "debug" => "silent".into(),
                                                _ => "info".into(),
                                            },
                                            13 => self.state.settings_allow_lan = !self.state.settings_allow_lan,
                                            14 => self.state.settings_ipv6 = !self.state.settings_ipv6,
                                            15 => { let _ = self.action_tx.try_send(Action::DownloadCore); }
                                            16 => self.state.settings_lang = if self.state.settings_lang == "zh" { "en".into() } else { "zh".into() },
                                            17 => self.state.settings_ui_theme = match self.state.settings_ui_theme.as_str() {
                                                "Catppuccin" => "Nord".into(),
                                                "Nord" => "TokyoNight".into(),
                                                "TokyoNight" => "Gruvbox".into(),
                                                _ => "Catppuccin".into(),
                                            },
                                            18 => self.state.settings_refresh_ms = match self.state.settings_refresh_ms {
                                                500 => 1000,
                                                1000 => 2000,
                                                _ => 500,
                                            },
                                            19 => { let _ = self.action_tx.try_send(Action::SaveSettings); }
                                            _ => {}
                                        }
                                    }
                                    KeyCode::Backspace => {
                                        match self.state.settings_focus {
                                            0 => { self.state.settings_api_url.pop(); }
                                            1 => { self.state.settings_secret.pop(); }
                                            2 => {
                                                let mut s = self.state.settings_http_port.to_string();
                                                s.pop();
                                                self.state.settings_http_port = s.parse::<u16>().unwrap_or(0);
                                            }
                                            3 => {
                                                let mut s = self.state.settings_socks_port.to_string();
                                                s.pop();
                                                self.state.settings_socks_port = s.parse::<u16>().unwrap_or(0);
                                            }
                                            4 => {
                                                let mut s = self.state.settings_mixed_port.to_string();
                                                s.pop();
                                                self.state.settings_mixed_port = s.parse::<u16>().unwrap_or(0);
                                            }
                                            5 => { self.state.settings_test_url.pop(); }
                                            _ => {}
                                        }
                                    }
                                    KeyCode::Char(c) => match self.state.settings_focus {
                                        0 => self.state.settings_api_url.push(c),
                                        1 => self.state.settings_secret.push(c),
                                        2 => if c.is_ascii_digit() {
                                            let mut s = self.state.settings_http_port.to_string();
                                            if s == "0" { s.clear(); }
                                            s.push(c);
                                            if let Ok(p) = s.parse::<u16>() {
                                                self.state.settings_http_port = p;
                                            }
                                        },
                                        3 => if c.is_ascii_digit() {
                                            let mut s = self.state.settings_socks_port.to_string();
                                            if s == "0" { s.clear(); }
                                            s.push(c);
                                            if let Ok(p) = s.parse::<u16>() {
                                                self.state.settings_socks_port = p;
                                            }
                                        },
                                        4 => if c.is_ascii_digit() {
                                            let mut s = self.state.settings_mixed_port.to_string();
                                            if s == "0" { s.clear(); }
                                            s.push(c);
                                            if let Ok(p) = s.parse::<u16>() {
                                                self.state.settings_mixed_port = p;
                                            }
                                        },
                                        5 => self.state.settings_test_url.push(c),
                                        _ => match c {
                                            'k' => self.state.settings_focus = if self.state.settings_focus == 0 { 19 } else { self.state.settings_focus - 1 },
                                            'j' => self.state.settings_focus = (self.state.settings_focus + 1) % 20,
                                            _ => {}
                                        },
                                    },
                                    _ => {}
                                }
                                return Ok(false);
                            }

                            match key.code {
                                // Movement in Active View List / Table
                                KeyCode::Up | KeyCode::Char('k') => self.move_selection(-1),
                                KeyCode::Down | KeyCode::Char('j') => self.move_selection(1),
                                KeyCode::PageUp => self.move_selection(-5),
                                KeyCode::PageDown => self.move_selection(5),

                                // Top & Bottom Jumps
                                KeyCode::Char('g') | KeyCode::Home => self.jump_top(),
                                KeyCode::Char('G') | KeyCode::End => self.jump_bottom(),

                                // Horizontal Pane Movement & Focus Zone Switching
                                KeyCode::Left | KeyCode::Char('h') => {
                                    if self.state.active_tab == Tab::Proxies {
                                        if self.state.proxy_sub_focus == ProxySubFocus::Nodes {
                                            self.state.proxy_sub_focus = ProxySubFocus::Groups;
                                        } else {
                                            self.state.focus_zone = FocusZone::Sidebar;
                                        }
                                    } else {
                                        self.state.focus_zone = FocusZone::Sidebar;
                                    }
                                }
                                KeyCode::Right | KeyCode::Char('l') => {
                                    if self.state.active_tab == Tab::Proxies {
                                        self.state.proxy_sub_focus = ProxySubFocus::Nodes;
                                    }
                                }

                                // View Action Keybindings
                                KeyCode::Char('/') => {
                                    self.state.is_searching = !self.state.is_searching;
                                    if !self.state.is_searching {
                                        self.state.search_query.clear();
                                    }
                                }
                                KeyCode::Char('t') | KeyCode::Char('T') => {
                                    if self.state.active_tab == Tab::Proxies {
                                        self.test_selected_group_latency();
                                    } else if self.state.active_tab == Tab::Dashboard {
                                        self.test_all_sites_latency();
                                    }
                                }
                                KeyCode::Char('o') | KeyCode::Char('O') if self.state.active_tab == Tab::Proxies => {
                                    self.state.sort_nodes_by_latency = !self.state.sort_nodes_by_latency;
                                    let status = if self.state.sort_nodes_by_latency { "已开启延迟升序排序" } else { "已恢复默认节点顺序" };
                                    self.state.push_toast(status.to_string());
                                }
                                KeyCode::Char('s') | KeyCode::Char('S') => {
                                    if self.state.active_tab == Tab::Connections {
                                        self.state.sort_connections_by_traffic = !self.state.sort_connections_by_traffic;
                                        let status = if self.state.sort_connections_by_traffic { "已开启连接流量降序排序" } else { "已恢复默认连接顺序" };
                                        self.state.push_toast(status.to_string());
                                    } else {
                                        self.state.is_searching = !self.state.is_searching;
                                        if !self.state.is_searching {
                                            self.state.search_query.clear();
                                        }
                                    }
                                }
                                KeyCode::Enter => {
                                    self.confirm_selection().await;
                                }
                                KeyCode::Char('a') | KeyCode::Char('A') if self.state.active_tab == Tab::Profiles => {
                                    self.state.profile_name_input.clear();
                                    self.state.profile_url_input.clear();
                                    self.state.profile_input_focus = 0;
                                    self.state.show_profile_input = true;
                                }
                                KeyCode::Char('u') | KeyCode::Char('U') if self.state.active_tab == Tab::Profiles => {
                                    if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx)
                                        && let Some(url) = p.url.clone() {
                                            let name = p.name.clone();
                                            let _ = self.action_tx.try_send(Action::AddProfile { name, url });
                                        }
                                }
                                KeyCode::Char('d') | KeyCode::Char('D') => {
                                    if self.state.active_tab == Tab::Proxies {
                                        self.test_single_node_latency();
                                    } else if self.state.active_tab == Tab::Profiles {
                                        if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx) {
                                            let name = p.name.clone();
                                            let _ = self.action_tx.try_send(Action::DeleteProfile(name));
                                        }
                                    } else if self.state.active_tab == Tab::Connections {
                                        if key.code == KeyCode::Char('D') {
                                            let _ = self.action_tx.try_send(Action::CloseAllConnections);
                                        } else {
                                            self.close_selected_connection().await;
                                        }
                                    }
                                }
                                KeyCode::Char('c') | KeyCode::Char('C') if self.state.active_tab == Tab::Logs => {
                                    self.state.logs.clear();
                                    self.state.push_toast("Logs cleared".to_string());
                                }
                                KeyCode::Char('r') | KeyCode::Char('R') if self.state.active_tab == Tab::Privileges => {
                                    let _ = self.action_tx.try_send(Action::RevokeTunPrivilege);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            Action::Mouse(mouse) => {
                use crossterm::event::{MouseButton, MouseEventKind};
                if self.state.show_tun_modal {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind
                        && !self.state.is_granting_privilege {
                            if !self.state.tun_password_input.is_empty() {
                                let pass = self.state.tun_password_input.clone();
                                let _ = self.action_tx.try_send(Action::GrantTunPrivilegeWithPassword(pass));
                            } else {
                                self.state.tun_input_focus = 0;
                            }
                        }
                    return Ok(false);
                }
                match mouse.kind {
                    MouseEventKind::ScrollDown => self.move_selection(1),
                    MouseEventKind::ScrollUp => self.move_selection(-1),
                    MouseEventKind::Down(MouseButton::Left) => {
                        let col = mouse.column;
                        let row = mouse.row;

                        // 1. Click on Left Sidebar Navigation Bar (Column < 18)
                        if col < 18 {
                            if (4..=14).contains(&row) {
                                let tab_idx = (row - 5) as usize;
                                if let Some(tab) = Tab::ALL.get(tab_idx) {
                                    self.state.active_tab = *tab;
                                    self.state.focus_zone = FocusZone::Sidebar;
                                }
                            }
                        }
                        // 2. Click on Top Control Pills Bar (Row <= 2, Column >= 18)
                        else if row <= 2 {
                            if (18..42).contains(&col) {
                                self.cycle_mode();
                            } else if (42..66).contains(&col) {
                                let _ = self.action_tx.try_send(Action::ToggleSystemProxy);
                            } else if (66..84).contains(&col) {
                                let _ = self.action_tx.try_send(Action::ToggleTunMode);
                            } else if col >= 84 {
                                let _ = self.action_tx.try_send(Action::RestartCore);
                            }
                        }
                        // 3. Tab Specific Workspace Clicks (Switches Focus to Workspace)
                        else {
                            self.state.focus_zone = FocusZone::Workspace;

                            if self.state.active_tab == Tab::Dashboard {
                                if (10..=16).contains(&row) {
                                    self.test_all_sites_latency();
                                }
                            } else if self.state.active_tab == Tab::Settings {
                                if col < 55 {
                                    if (4..=6).contains(&row) {
                                        self.state.settings_focus = 0;
                                    } else if (7..=9).contains(&row) {
                                        self.state.settings_focus = 1;
                                    } else if (10..=12).contains(&row) {
                                        self.state.settings_focus = 2;
                                    } else if (13..=15).contains(&row) {
                                        self.state.settings_focus = 3;
                                    } else if row >= 16 {
                                        self.state.settings_focus = 4;
                                    }
                                } else {
                                    if (4..=6).contains(&row) {
                                        self.state.settings_focus = 5;
                                        self.state.settings_tun_stack = match self.state.settings_tun_stack.as_str() {
                                            "system" => "gvisor".into(),
                                            "gvisor" => "lwip".into(),
                                            _ => "system".into(),
                                        };
                                    } else if (7..=9).contains(&row) {
                                        self.state.settings_focus = 6;
                                        self.state.settings_log_level = match self.state.settings_log_level.as_str() {
                                            "info" => "warning".into(),
                                            "warning" => "error".into(),
                                            "error" => "debug".into(),
                                            "debug" => "silent".into(),
                                            _ => "info".into(),
                                        };
                                    } else if (10..=12).contains(&row) {
                                        self.state.settings_focus = 7;
                                        self.state.settings_allow_lan = !self.state.settings_allow_lan;
                                    } else if (13..=15).contains(&row) {
                                        self.state.settings_focus = 8;
                                        self.state.settings_ipv6 = !self.state.settings_ipv6;
                                    } else if (16..=18).contains(&row) {
                                        self.state.settings_focus = 9;
                                        self.state.settings_lang = if self.state.settings_lang == "zh" { "en".into() } else { "zh".into() };
                                    } else if (19..=21).contains(&row) {
                                        self.state.settings_focus = 10;
                                        self.state.settings_refresh_ms = match self.state.settings_refresh_ms {
                                            500 => 1000,
                                            1000 => 2000,
                                            _ => 500,
                                        };
                                    } else if row >= 22 {
                                        self.state.settings_focus = 11;
                                        let _ = self.action_tx.try_send(Action::SaveSettings);
                                    }
                                }
                            } else if self.state.active_tab == Tab::Profiles {
                                if row <= 5 {
                                    if (18..35).contains(&col) {
                                        self.state.profile_name_input.clear();
                                        self.state.profile_url_input.clear();
                                        self.state.profile_input_focus = 0;
                                        self.state.show_profile_input = true;
                                    } else if (35..52).contains(&col) {
                                        if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx)
                                            && let Some(url) = p.url.clone() {
                                                let name = p.name.clone();
                                                let _ = self.action_tx.try_send(Action::AddProfile { name, url });
                                            }
                                    } else if (52..68).contains(&col)
                                        && let Some(p) = self.state.profiles.get(self.state.selected_profile_idx) {
                                            let name = p.name.clone();
                                            let _ = self.action_tx.try_send(Action::DeleteProfile(name));
                                        }
                                } else {
                                    let click_idx = (row - 5) as usize;
                                    if click_idx < self.state.profiles.len() {
                                        self.state.selected_profile_idx = click_idx;
                                        self.confirm_selection().await;
                                    }
                                }
                            } else if self.state.active_tab == Tab::Proxies {
                                if col < 50 {
                                    self.state.proxy_sub_focus = ProxySubFocus::Groups;
                                    if row >= 4 {
                                        let click_idx = (row - 4) as usize;
                                        if click_idx < self.state.proxy_groups.len() {
                                            self.state.selected_group_idx = click_idx;
                                            self.state.selected_node_idx = 0;
                                        }
                                    }
                                } else {
                                    self.state.proxy_sub_focus = ProxySubFocus::Nodes;
                                    if row >= 4 {
                                        let click_idx = (row - 4) as usize;
                                        let nodes = self.state.filtered_group_nodes();
                                        if click_idx < nodes.len() {
                                            self.state.selected_node_idx = click_idx;
                                            self.confirm_selection().await;
                                        }
                                    }
                                }
                            } else if self.state.active_tab == Tab::Rules {
                                if row >= 4 {
                                    let click_idx = (row - 4) as usize;
                                    if let Some(resp) = &self.state.rules_resp
                                        && click_idx < resp.rules.len() {
                                            self.state.selected_rule_idx = click_idx;
                                        }
                                }
                            } else if self.state.active_tab == Tab::Connections
                                && row >= 4 {
                                    let click_idx = (row - 4) as usize;
                                    if let Some(resp) = &self.state.connections_resp
                                        && click_idx < resp.connections.len() {
                                            self.state.selected_conn_idx = click_idx;
                                        }
                                }
                        }
                    }
                    _ => {}
                }
            }

            Action::VersionFetched(res) => match res {
                Ok(v) => self.state.version = Some(v),
                Err(e) => self.state.status_error = Some(e),
            },

            Action::ConfigFetched(res) => match res {
                Ok(cfg) => {
                    if let Some(tun) = &cfg.tun {
                        self.state.is_tun_enabled = tun.enable;
                    }
                    self.state.config = Some(cfg);
                }
                Err(e) => self.state.status_error = Some(e),
            },

            Action::ProxiesFetched(res) => match res {
                Ok(resp) => {
                    self.state.proxy_groups = resp
                        .proxies
                        .iter()
                        .filter(|(_, p)| p.proxy_type.eq_ignore_ascii_case("Selector") || p.proxy_type.eq_ignore_ascii_case("URLTest") || p.all.is_some())
                        .map(|(name, _)| name.clone())
                        .collect();
                    self.state.proxy_groups.sort();
                    self.state.proxies_resp = Some(resp);
                }
                Err(e) => self.state.status_error = Some(e),
            },

            Action::RulesFetched(res) => match res {
                Ok(resp) => {
                    self.state.rules_resp = Some(resp);
                    if let Some(r) = &self.state.rules_resp
                        && !r.rules.is_empty() && self.state.selected_rule_idx >= r.rules.len() {
                            self.state.selected_rule_idx = r.rules.len() - 1;
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

                    if let Some(active) = self.state.profiles.iter().find(|p| p.is_active).or_else(|| self.state.profiles.first())
                        && let Ok(content) = std::fs::read_to_string(&active.file_path)
                            && let Ok(parsed) = crate::profile::ProfileParser::parse_yaml(&content) {
                                // Auto adapt MihomoClient API URL if external_controller is defined
                                if let Some(ext_ctrl) = &parsed.external_controller {
                                    let formatted_url = if ext_ctrl.starts_with("http://") || ext_ctrl.starts_with("https://") {
                                        ext_ctrl.clone()
                                    } else {
                                        format!("http://{}", ext_ctrl)
                                    };
                                    if self.state.settings_api_url != formatted_url {
                                        self.state.settings_api_url = formatted_url.clone();
                                        if let Ok(new_client) = crate::api::MihomoClient::new(&formatted_url, parsed.secret.clone().or_else(|| if self.state.settings_secret.is_empty() { None } else { Some(self.state.settings_secret.clone()) })) {
                                            self.client = new_client;
                                        }
                                    }
                                }

                                if self.state.proxy_groups.is_empty() {
                                    let mut groups: Vec<String> = parsed.proxy_groups.iter().map(|g| g.name.clone()).collect();
                                    if !groups.contains(&"GLOBAL".to_string()) {
                                        groups.insert(0, "GLOBAL".to_string());
                                    }
                                    self.state.proxy_groups = groups;
                                }
                                self.state.parsed_active_profile = Some(parsed);

                                // Auto-start Mihomo Core if not running
                                if !crate::core::CoreProcess::is_running() {
                                    if crate::core::CoreProcess::find_mihomo_binary().is_none() {
                                        let _ = self.action_tx.try_send(Action::DownloadCore);
                                    } else {
                                        let _ = crate::core::CoreProcess::start_with_config(&active.file_path);
                                    }
                                }
                            }
                }
                Err(e) => self.state.push_toast(format!("Profile Error: {}", e)),
            },

            Action::AddProfile { name, url } => {
                self.state.push_toast(format!("Downloading profile '{}'...", name));
                let tx = self.action_tx.clone();
                let client = self.client.clone();
                tokio::spawn(async move {
                    match crate::profile::ProfileManager::download_profile(&name, &url).await {
                        Ok(file_path) => {
                            let path_str = file_path.to_string_lossy().to_string();
                            let _ = client.reload_config(&path_str).await;
                            let _ = tx.send(Action::FetchProfiles).await;
                            let _ = tx.send(Action::FetchProxies).await;
                        }
                        Err(e) => {
                            let _ = tx.send(Action::ProfilesFetched(Err(format!("Download failed: {}", e)))).await;
                        }
                    }
                });
            }

            Action::UpdateProfile(name) => {
                let tx = self.action_tx.clone();
                let client = self.client.clone();
                let target_name = name.clone();
                tokio::spawn(async move {
                    if let Ok(dir) = crate::profile::ProfileManager::profiles_dir() {
                        let path = dir.join(format!("{}.yaml", target_name));
                        if path.exists() {
                            let path_str = path.to_string_lossy().to_string();
                            if client.reload_config(&path_str).await.is_err() {
                                let _ = crate::core::CoreProcess::start_with_config(&path);
                            }
                            let _ = crate::profile::ProfileManager::set_active_profile(&target_name);
                            let _ = tx.send(Action::FetchProfiles).await;
                            let _ = tx.send(Action::FetchProxies).await;
                            let _ = tx.send(Action::FetchRules).await;
                        }
                    }
                });
            }

            Action::DeleteProfile(name) => {
                let _ = crate::profile::ProfileManager::delete_profile(&name);
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

            Action::FetchProxies => {
                self.fetch_proxies();
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

            Action::ShowTunModal => {
                self.state.active_tab = Tab::Privileges;
                self.state.focus_zone = FocusZone::Workspace;
                self.state.show_tun_modal = true;
                self.state.tun_password_input.clear();
                self.state.tun_input_focus = 0;
            }

            Action::HideTunModal => {
                self.state.show_tun_modal = false;
                self.state.is_granting_privilege = false;
                self.state.tun_password_input.clear();
            }

            Action::GrantTunPrivilegeWithPassword(password) => {
                self.state.is_granting_privilege = true;
                let is_priv = self.state.is_tun_privileged;
                if is_priv {
                    self.state.push_toast("🔐 正在验证 Sudo 密码并撤销 CAP_NET_ADMIN 权限...".to_string());
                } else {
                    self.state.push_toast("🔐 正在验证 Sudo 密码并开启 CAP_NET_ADMIN 权限...".to_string());
                }

                let tx = self.action_tx.clone();
                tokio::task::spawn_blocking(move || {
                    let res = if is_priv {
                        crate::core::TunMode::revoke_privilege_with_password(&password)
                            .map_err(|e| e.to_string())
                    } else {
                        crate::core::TunMode::grant_privilege_with_password(&password)
                            .map_err(|e| e.to_string())
                    };
                    let _ = tx.blocking_send(Action::TunPrivilegeResult(res));
                });
            }

            Action::GrantTunPrivilegePkexec => {
                self.state.is_granting_privilege = true;
                self.state.push_toast("🔐 正在尝试唤起 Linux Polkit GUI 系统窗口...".to_string());

                let tx = self.action_tx.clone();
                tokio::task::spawn_blocking(move || {
                    let res = crate::core::TunMode::grant_privilege_pkexec()
                        .map_err(|e| e.to_string());
                    let _ = tx.blocking_send(Action::TunPrivilegeResult(res));
                });
            }

            Action::TunPrivilegeResult(res) => {
                self.state.is_granting_privilege = false;
                self.state.tun_password_input.clear();
                let _ = self.action_tx.try_send(Action::ClearScreen);
                match res {
                    Ok(_) => {
                        self.state.is_tun_privileged = crate::core::TunMode::check_privilege();
                        if self.state.is_tun_privileged {
                            self.state.show_tun_modal = false;
                            self.state.push_toast("🎉 系统权限授权成功 (CAP_NET_ADMIN)".to_string());

                            let client = self.client.clone();
                            let stack = self.state.settings_tun_stack.clone();
                            let tx = self.action_tx.clone();
                            tokio::spawn(async move {
                                let _ = crate::core::CoreProcess::restart();
                                tokio::time::sleep(std::time::Duration::from_millis(800)).await;
                                if client.set_tun_config(true, &stack).await.is_ok() {
                                    let _ = tx.send(Action::FetchConfig).await;
                                }
                            });
                        } else {
                            self.state.show_tun_modal = false;
                            self.state.is_tun_enabled = false;
                            self.state.push_toast("🚫 已成功撤销 CAP_NET_ADMIN 系统权限".to_string());

                            let client = self.client.clone();
                            let stack = self.state.settings_tun_stack.clone();
                            let tx = self.action_tx.clone();
                            tokio::spawn(async move {
                                let _ = client.set_tun_config(false, &stack).await;
                                let _ = crate::core::CoreProcess::restart();
                                let _ = tx.send(Action::FetchConfig).await;
                            });
                        }
                    }
                    Err(err) => {
                        self.state.push_toast(format!("❌ 操作失败: {}", err));
                    }
                }
            }

            Action::RevokeTunPrivilege => {
                self.state.push_toast("🚫 正在撤销 CAP_NET_ADMIN 系统权限...".to_string());
                let tx = self.action_tx.clone();
                tokio::task::spawn_blocking(move || {
                    let res = crate::core::TunMode::revoke_privilege().map_err(|e| e.to_string());
                    let _ = tx.blocking_send(Action::RevokeTunPrivilegeResult(res));
                });
            }

            Action::RevokeTunPrivilegeResult(res) => {
                let _ = self.action_tx.try_send(Action::ClearScreen);
                match res {
                    Ok(_) => {
                        self.state.is_tun_privileged = crate::core::TunMode::check_privilege();
                        if !self.state.is_tun_privileged {
                            self.state.is_tun_enabled = false;
                            self.state.push_toast("🚫 已成功撤销 CAP_NET_ADMIN 系统权限".to_string());
                            let client = self.client.clone();
                            let stack = self.state.settings_tun_stack.clone();
                            let tx = self.action_tx.clone();
                            tokio::spawn(async move {
                                let _ = client.set_tun_config(false, &stack).await;
                                let _ = crate::core::CoreProcess::restart();
                                let _ = tx.send(Action::FetchConfig).await;
                            });
                        } else {
                            self.state.push_toast("已成功撤销 CAP_NET_ADMIN 系统权限".to_string());
                        }
                    }
                    Err(_) => {
                        self.state.is_tun_privileged = crate::core::TunMode::check_privilege();
                        if !self.state.is_tun_privileged {
                            self.state.is_tun_enabled = false;
                            self.state.push_toast("🚫 已成功撤销 CAP_NET_ADMIN 系统权限".to_string());
                            let client = self.client.clone();
                            let stack = self.state.settings_tun_stack.clone();
                            let tx = self.action_tx.clone();
                            tokio::spawn(async move {
                                let _ = client.set_tun_config(false, &stack).await;
                                let _ = crate::core::CoreProcess::restart();
                                let _ = tx.send(Action::FetchConfig).await;
                            });
                        } else {
                            // Non-interactive Polkit GUI / Sudo failed: pop up password modal!
                            self.state.show_tun_modal = true;
                            self.state.push_toast("⚠️ 请输入 Sudo 密码以完成权限撤销".to_string());
                        }
                    }
                }
            }

            Action::ToggleTunMode => {
                let _ = self.action_tx.try_send(Action::ClearScreen);
                self.state.is_tun_privileged = crate::core::TunMode::check_privilege();
                if !self.state.is_tun_privileged {
                    let _ = self.action_tx.try_send(Action::ShowTunModal);
                    let msg = if self.state.settings_lang == "zh" { "⚠️ 开启 TUN 模式需先完成 CAP_NET_ADMIN 权限授权" } else { "⚠️ TUN mode requires CAP_NET_ADMIN capability authorization" };
                    self.state.push_toast(msg.to_string());
                } else {
                    let client = self.client.clone();
                    let new_state = !self.state.is_tun_enabled;
                    let stack = self.state.settings_tun_stack.clone();
                    let tx = self.action_tx.clone();
                    tokio::spawn(async move {
                        if client.set_tun_config(new_state, &stack).await.is_ok() {
                            let _ = tx.send(Action::FetchConfig).await;
                        }
                    });
                    if new_state {
                        let msg = if self.state.settings_lang == "zh" { "TUN 虚拟网卡模式已开启" } else { "TUN Virtual Adapter Mode Enabled" };
                        self.state.push_toast(msg.to_string());
                    } else {
                        let msg = if self.state.settings_lang == "zh" { "TUN 虚拟网卡模式已关闭" } else { "TUN Virtual Adapter Mode Disabled" };
                        self.state.push_toast(msg.to_string());
                    }
                }
            }

            Action::FetchConfig => {
                self.fetch_config();
            }

            Action::DownloadCore => {
                self.state.push_toast("正在自动为您下载与安装 Mihomo 核心...".to_string());
                let tx = self.action_tx.clone();
                tokio::spawn(async move {
                    let res = crate::core::CoreDownloader::download_and_install(|_msg| {}).await
                        .map(|p| p.to_string_lossy().to_string())
                        .map_err(|e| e.to_string());
                    let _ = tx.send(Action::DownloadCoreResult(res)).await;
                });
            }

            Action::DownloadCoreResult(res) => match res {
                Ok(path) => {
                    self.state.push_toast(format!("🎉 Mihomo 核心自动安装成功: {}", path));
                    self.state.status_error = None;
                    self.fetch_profiles();
                    self.fetch_version();
                }
                Err(e) => {
                    self.state.push_toast(format!("❌ Mihomo 自动下载失败: {}", e));
                }
            },

            Action::SaveSettings => {
                let mut cfg = crate::config::Config::load().unwrap_or_default();
                cfg.language = self.state.settings_lang.clone();
                cfg.api_url = self.state.settings_api_url.clone();
                cfg.secret = if self.state.settings_secret.trim().is_empty() { None } else { Some(self.state.settings_secret.clone()) };
                cfg.refresh_interval_ms = self.state.settings_refresh_ms;
                cfg.http_port = self.state.settings_http_port;
                cfg.socks_port = self.state.settings_socks_port;
                cfg.mixed_port = self.state.settings_mixed_port;
                cfg.sniffing = self.state.settings_sniffing;
                cfg.tcp_concurrent = self.state.settings_tcp_concurrent;
                cfg.test_url = self.state.settings_test_url.clone();
                cfg.tun_stack = self.state.settings_tun_stack.clone();
                cfg.log_level = self.state.settings_log_level.clone();
                cfg.allow_lan = self.state.settings_allow_lan;
                cfg.ipv6 = self.state.settings_ipv6;
                cfg.dns_mode = self.state.settings_dns_mode.clone();
                cfg.auto_sysproxy = self.state.settings_auto_sysproxy;
                cfg.sub_update_hours = self.state.settings_sub_update_hours;
                cfg.ui_theme = self.state.settings_ui_theme.clone();

                if cfg.save().is_ok() {
                    let client = self.client.clone();
                    let stack = self.state.settings_tun_stack.clone();
                    let is_tun = self.state.is_tun_enabled;
                    let is_priv = self.state.is_tun_privileged;
                    tokio::spawn(async move {
                        if is_priv || !is_tun {
                            let _ = client.set_tun_config(is_tun, &stack).await;
                        }
                    });
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
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    let _ = tx.send(Action::FetchVersion).await;
                    let _ = tx.send(Action::FetchConfig).await;
                });
            }

            Action::FetchVersion => self.fetch_version(),
            Action::FetchOutboundIp => {
                let tx = self.action_tx.clone();
                let port = self.state.config.as_ref().and_then(|c| c.mixed_port.or(c.http_port)).or(Some(self.state.settings_http_port));
                tokio::spawn(async move {
                    let res = crate::api::MihomoClient::get_outbound_ip(port).await.map_err(|e| e.to_string());
                    let _ = tx.send(Action::OutboundIpFetched(res)).await;
                });
            }
            Action::OutboundIpFetched(res) => {
                if let Ok(ip) = res {
                    self.state.outbound_ip = Some(ip);
                }
            }

            Action::TrafficReceived(msg) => {
                self.state.current_traffic = msg.clone();
                self.state.up_history.push_back(msg.up);
                if self.state.up_history.len() > 40 {
                    self.state.up_history.pop_front();
                }
                self.state.down_history.push_back(msg.down);
                if self.state.down_history.len() > 40 {
                    self.state.down_history.pop_front();
                }
            }

            Action::LogReceived(log) => {
                self.state.logs.push_back(log);
                if self.state.logs.len() > 500 {
                    self.state.logs.pop_front();
                }
            }

            Action::ConnectionsFetched(res) => match res {
                Ok(resp) => {
                    self.state.connections_resp = Some(resp);
                    if let Some(resp) = &self.state.connections_resp
                        && !resp.connections.is_empty() && self.state.selected_conn_idx >= resp.connections.len() {
                            self.state.selected_conn_idx = resp.connections.len() - 1;
                        }
                }
                Err(e) => self.state.status_error = Some(e),
            },

            Action::LatencyResult { node, result } => {
                let delay = result.ok();
                self.state.latency_map.insert(node, delay);
            }

            Action::TestSiteLatencies => {
                self.test_all_sites_latency();
            }

            Action::SiteLatencyResult { site, result } => {
                let delay = result.ok();
                self.state.site_latencies.insert(site, delay);
            }

            _ => {}
        }

        Ok(false)
    }

    fn move_selection(&mut self, delta: i32) {
        match self.state.active_tab {
            Tab::Proxies => match self.state.proxy_sub_focus {
                ProxySubFocus::Groups => {
                    let len = self.state.proxy_groups.len();
                    if len > 0 {
                        self.state.selected_group_idx = (self.state.selected_group_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                        self.state.selected_node_idx = 0;
                    }
                }
                ProxySubFocus::Nodes => {
                    let nodes = self.state.display_group_nodes();
                    let len = nodes.len();
                    if len > 0 {
                        self.state.selected_node_idx = (self.state.selected_node_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                    }
                }
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
                let len = self.state.filtered_sorted_connections().len();
                if len > 0 {
                    self.state.selected_conn_idx = (self.state.selected_conn_idx as i32 + delta)
                        .clamp(0, len as i32 - 1) as usize;
                }
            }
            Tab::Logs => {
                let current = self.state.log_scroll as i32;
                let max_scroll = self.state.logs.len().saturating_sub(1) as i32;
                self.state.log_scroll = (current + delta).clamp(0, max_scroll.max(0)) as usize;
            }
            Tab::Settings => {
                let current = self.state.settings_focus as i32;
                self.state.settings_focus = (current + delta).clamp(0, 11) as usize;
            }
            _ => {}
        }
    }

    fn jump_top(&mut self) {
        match self.state.active_tab {
            Tab::Proxies => match self.state.proxy_sub_focus {
                ProxySubFocus::Groups => self.state.selected_group_idx = 0,
                ProxySubFocus::Nodes => self.state.selected_node_idx = 0,
            },
            Tab::Profiles => self.state.selected_profile_idx = 0,
            Tab::Rules => self.state.selected_rule_idx = 0,
            Tab::Connections => self.state.selected_conn_idx = 0,
            Tab::Logs => self.state.log_scroll = 0,
            Tab::Settings => self.state.settings_focus = 0,
            _ => {}
        }
    }

    fn jump_bottom(&mut self) {
        match self.state.active_tab {
            Tab::Proxies => match self.state.proxy_sub_focus {
                ProxySubFocus::Groups => {
                    let len = self.state.proxy_groups.len();
                    if len > 0 { self.state.selected_group_idx = len - 1; }
                }
                ProxySubFocus::Nodes => {
                    let len = self.state.display_group_nodes().len();
                    if len > 0 { self.state.selected_node_idx = len - 1; }
                }
            },
            Tab::Profiles => {
                let len = self.state.profiles.len();
                if len > 0 { self.state.selected_profile_idx = len - 1; }
            }
            Tab::Rules => {
                if let Some(resp) = &self.state.rules_resp {
                    let len = resp.rules.len();
                    if len > 0 { self.state.selected_rule_idx = len - 1; }
                }
            }
            Tab::Connections => {
                let len = self.state.filtered_sorted_connections().len();
                if len > 0 { self.state.selected_conn_idx = len - 1; }
            }
            Tab::Logs => {
                let len = self.state.logs.len();
                if len > 0 { self.state.log_scroll = len - 1; }
            }
            Tab::Settings => self.state.settings_focus = 11,
            _ => {}
        }
    }

    async fn confirm_selection(&mut self) {
        match self.state.active_tab {
            Tab::Proxies => {
                if self.state.proxy_sub_focus == ProxySubFocus::Groups {
                    self.state.proxy_sub_focus = ProxySubFocus::Nodes;
                    self.state.selected_node_idx = 0;
                    return;
                }

                let group = match self.state.selected_group_name() {
                    Some(g) => g.to_string(),
                    None => return,
                };
                let nodes = self.state.display_group_nodes();
                let node = match nodes.get(self.state.selected_node_idx) {
                    Some(n) => n.to_string(),
                    None => return,
                };

                let client = self.client.clone();
                let groups = self.state.proxy_groups.clone();
                let target_node = node.clone();
                let tx = self.action_tx.clone();

                tokio::spawn(async move {
                    // 1. Update selected group directly
                    let _ = client.select_proxy(&group, &target_node).await;

                    // 2. Broadcast selection to ALL proxy selector groups
                    for g in &groups {
                        let _ = client.select_proxy(g, &target_node).await;
                    }

                    let _ = tx.send(Action::FetchProxies).await;
                });

                self.state.push_toast(format!("Selected '{}' (Applied to all groups)", node));
            }
            Tab::Profiles => {
                if let Some(profile) = self.state.profiles.get(self.state.selected_profile_idx) {
                    let name = profile.name.clone();
                    let tx = self.action_tx.clone();
                    let _ = tx.send(Action::UpdateProfile(name)).await;
                }
            }
            _ => {}
        }
    }

    fn cycle_mode(&mut self) {
        let current = self
            .state
            .config
            .as_ref()
            .and_then(|c| c.mode.clone())
            .unwrap_or_else(|| "Rule".into());

        let next = match current.to_lowercase().as_str() {
            "rule" => "Global",
            "global" => "Direct",
            _ => "Rule",
        };

        let client = self.client.clone();
        let tx = self.action_tx.clone();
        let next_mode = next.to_string();
        tokio::spawn(async move {
            if client.set_mode(&next_mode).await.is_ok() {
                let _ = tx.send(Action::FetchConfig).await;
            }
        });
    }

    fn test_selected_group_latency(&self) {
        let group = match self.state.selected_group_name() {
            Some(g) => g.to_string(),
            None => return,
        };

        let nodes = self.state.current_group_nodes();
        let client = self.client.clone();
        let tx = self.action_tx.clone();
        let test_url = self.state.settings_test_url.clone();

        tokio::spawn(async move {
            // 1. Try native core group delay testing API
            if let Ok(delays) = client.test_group_delay(&group, Some(&test_url), Some(3000)).await {
                for (node_name, ms) in delays {
                    let _ = tx.send(Action::LatencyResult { node: node_name, result: Ok(ms) }).await;
                }
                return;
            }

            // 2. Fallback to concurrent individual tests
            for node in nodes {
                let c = client.clone();
                let t = tx.clone();
                let node_name = node.clone();
                let url = test_url.clone();
                tokio::spawn(async move {
                    let delay = c.test_delay(&node_name, Some(&url), Some(3000)).await.map_err(|e| e.to_string());
                    let _ = t.send(Action::LatencyResult { node: node_name, result: delay }).await;
                });
            }
        });
    }

    fn test_single_node_latency(&self) {
        let nodes = self.state.display_group_nodes();
        if let Some(node) = nodes.get(self.state.selected_node_idx) {
            let client = self.client.clone();
            let tx = self.action_tx.clone();
            let node_name = (*node).clone();
            let test_url = self.state.settings_test_url.clone();
            tokio::spawn(async move {
                let delay = client.test_delay(&node_name, Some(&test_url), Some(3000)).await.map_err(|e| e.to_string());
                let _ = tx.send(Action::LatencyResult { node: node_name, result: delay }).await;
            });
        }
    }

    pub fn test_all_sites_latency(&mut self) {
        self.state.push_toast("正在刷新 6 大常用网站连通性延迟...".to_string());
        for key in ["Google", "GitHub", "YouTube", "OpenAI", "Bilibili", "Baidu"] {
            self.state.site_latencies.insert(key.to_string(), None);
        }

        let sites = [
            ("Google", "https://www.google.com/generate_204"),
            ("GitHub", "https://github.com"),
            ("YouTube", "https://www.youtube.com"),
            ("OpenAI", "https://chatgpt.com"),
            ("Bilibili", "https://www.bilibili.com"),
            ("Baidu", "https://www.baidu.com"),
        ];

        let client = self.client.clone();
        let tx = self.action_tx.clone();
        let proxy_port = self.state.config.as_ref().and_then(|c| c.mixed_port.or(c.http_port)).or(Some(self.state.settings_http_port));

        for (site_name, url) in sites {
            let site_name = site_name.to_string();
            let url = url.to_string();
            let client = client.clone();
            let tx = tx.clone();
            let port = proxy_port;

            tokio::spawn(async move {
                let start = std::time::Instant::now();
                let res = match client.test_delay("GLOBAL", Some(&url), Some(3000)).await {
                    Ok(ms) => Ok(ms),
                    Err(_) => {
                        let mut builder = reqwest::Client::builder().timeout(Duration::from_millis(3000));
                        if let Some(p) = port {
                            if let Ok(proxy) = reqwest::Proxy::all(format!("http://127.0.0.1:{}", p)) {
                                builder = builder.proxy(proxy);
                            }
                        }
                        let http_client = builder.build().unwrap_or_default();
                        if http_client.get(&url).send().await.is_ok() {
                            Ok(start.elapsed().as_millis() as u16)
                        } else {
                            Err("Timeout".to_string())
                        }
                    }
                };
                let _ = tx.send(Action::SiteLatencyResult { site: site_name, result: res }).await;
            });
        }
    }


    async fn close_selected_connection(&mut self) {
        let conns = self.state.filtered_sorted_connections();
        let id = match conns.get(self.state.selected_conn_idx) {
            Some(conn) => conn.id.clone(),
            None => return,
        };

        if self.client.close_connection(&id).await.is_ok() {
            self.fetch_connections();
            self.state.push_toast("Closed connection".to_string());
        }
    }
}
