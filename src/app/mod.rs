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
                            if key.kind == crossterm::event::KeyEventKind::Press {
                                if self.action_tx.send(Action::Key(key)).await.is_err() {
                                    break;
                                }
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
                // Layer 0: Global Application Quit
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    return Ok(true);
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
                        }
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
                if self.state.is_searching {
                    match key.code {
                        KeyCode::Esc => {
                            self.state.is_searching = false;
                            self.state.search_query.clear();
                        }
                        KeyCode::Enter => {
                            self.state.is_searching = false;
                        }
                        KeyCode::Backspace => {
                            self.state.search_query.pop();
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

                // Layer 3: Global Hotkeys & Tab Switching
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('?') => self.state.show_help = !self.state.show_help,

                    // Universal Esc: Reset Search & Dialogs
                    KeyCode::Esc => {
                        self.state.is_searching = false;
                        self.state.search_query.clear();
                    }

                    // Direct Tab Switch 1-8
                    KeyCode::Char('1') => self.state.active_tab = Tab::Dashboard,
                    KeyCode::Char('2') => self.state.active_tab = Tab::Proxies,
                    KeyCode::Char('3') => self.state.active_tab = Tab::Profiles,
                    KeyCode::Char('4') => self.state.active_tab = Tab::Rules,
                    KeyCode::Char('5') => self.state.active_tab = Tab::Connections,
                    KeyCode::Char('6') => self.state.active_tab = Tab::Traffic,
                    KeyCode::Char('7') => self.state.active_tab = Tab::Logs,
                    KeyCode::Char('8') => self.state.active_tab = Tab::Settings,

                    // Tab / Shift+Tab Navigation
                    KeyCode::Tab => {
                        if self.state.active_tab == Tab::Settings {
                            self.state.settings_focus = (self.state.settings_focus + 1) % 12;
                        } else {
                            self.next_tab();
                        }
                    }
                    KeyCode::BackTab => {
                        if self.state.active_tab == Tab::Settings {
                            self.state.settings_focus = if self.state.settings_focus == 0 { 11 } else { self.state.settings_focus - 1 };
                        } else {
                            self.prev_tab();
                        }
                    }

                    // Global Core Control Hotkeys
                    KeyCode::Char('m') | KeyCode::Char('M') => self.cycle_mode(),
                    KeyCode::Char('p') | KeyCode::Char('P') if self.state.active_tab != Tab::Privileges => { let _ = self.action_tx.try_send(Action::ToggleSystemProxy); }
                    KeyCode::Char('x') | KeyCode::Char('X') => { let _ = self.action_tx.try_send(Action::ToggleTunMode); }
                    KeyCode::Char('r') | KeyCode::Char('R') if self.state.active_tab != Tab::Privileges => { let _ = self.action_tx.try_send(Action::RestartCore); }

                    // Layer 4: View-Specific & Standard List Navigation Keybindings
                    _ => {
                        // Specialized Input Handling for Settings View
                        if self.state.active_tab == Tab::Settings {
                            match key.code {
                                KeyCode::Up | KeyCode::Char('k') => self.state.settings_focus = if self.state.settings_focus == 0 { 11 } else { self.state.settings_focus - 1 },
                                KeyCode::Down | KeyCode::Char('j') => self.state.settings_focus = (self.state.settings_focus + 1) % 12,
                                KeyCode::Enter => {
                                    if self.state.settings_focus == 11 {
                                        let _ = self.action_tx.try_send(Action::SaveSettings);
                                    } else {
                                        self.state.settings_focus = (self.state.settings_focus + 1) % 12;
                                    }
                                }
                                KeyCode::Char(' ') => {
                                    match self.state.settings_focus {
                                        0 => self.state.settings_api_url.push(' '),
                                        1 => self.state.settings_secret.push(' '),
                                        4 => self.state.settings_test_url.push(' '),
                                        5 => self.state.settings_tun_stack = match self.state.settings_tun_stack.as_str() {
                                            "system" => "gvisor".into(),
                                            "gvisor" => "lwip".into(),
                                            _ => "system".into(),
                                        },
                                        6 => self.state.settings_log_level = match self.state.settings_log_level.as_str() {
                                            "info" => "warning".into(),
                                            "warning" => "error".into(),
                                            "error" => "debug".into(),
                                            "debug" => "silent".into(),
                                            _ => "info".into(),
                                        },
                                        7 => self.state.settings_allow_lan = !self.state.settings_allow_lan,
                                        8 => self.state.settings_ipv6 = !self.state.settings_ipv6,
                                        9 => self.state.settings_lang = if self.state.settings_lang == "zh" { "en".into() } else { "zh".into() },
                                        10 => self.state.settings_refresh_ms = match self.state.settings_refresh_ms {
                                            500 => 1000,
                                            1000 => 2000,
                                            _ => 500,
                                        },
                                        11 => { let _ = self.action_tx.try_send(Action::SaveSettings); }
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
                                        4 => { self.state.settings_test_url.pop(); }
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
                                    4 => self.state.settings_test_url.push(c),
                                    _ => match c {
                                        'k' => self.state.settings_focus = if self.state.settings_focus == 0 { 11 } else { self.state.settings_focus - 1 },
                                        'j' => self.state.settings_focus = (self.state.settings_focus + 1) % 12,
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

                            // Top & Bottom Jumps
                            KeyCode::Char('g') | KeyCode::Home => self.jump_top(),
                            KeyCode::Char('G') | KeyCode::End => self.jump_bottom(),

                            // Horizontal Pane Movement
                            KeyCode::Left | KeyCode::Char('h') => {
                                if self.state.active_tab == Tab::Proxies {
                                    self.state.proxy_sub_focus = ProxySubFocus::Groups;
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
                            KeyCode::Char('t') | KeyCode::Char('T') if self.state.active_tab == Tab::Proxies => self.test_selected_group_latency(),
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
                                if let Some(p) = self.state.profiles.get(self.state.selected_profile_idx) {
                                    if let Some(url) = p.url.clone() {
                                        let name = p.name.clone();
                                        let _ = self.action_tx.try_send(Action::AddProfile { name, url });
                                    }
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

            Action::Mouse(mouse) => {
                use crossterm::event::{MouseButton, MouseEventKind};
                if self.state.show_tun_modal {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        if !self.state.is_granting_privilege {
                            if !self.state.tun_password_input.is_empty() {
                                let pass = self.state.tun_password_input.clone();
                                let _ = self.action_tx.try_send(Action::GrantTunPrivilegeWithPassword(pass));
                            } else {
                                self.state.tun_input_focus = 0;
                            }
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
                            if row >= 4 && row <= 12 {
                                let tab_idx = (row - 4) as usize;
                                if let Some(tab) = Tab::ALL.get(tab_idx) {
                                    self.state.active_tab = *tab;
                                    self.state.focus_zone = FocusZone::Sidebar;
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
                        // 3. Tab Specific Workspace Clicks (Switches Focus to Workspace)
                        else {
                            self.state.focus_zone = FocusZone::Workspace;

                            if self.state.active_tab == Tab::Settings {
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
                                } else if row >= 21 && row <= 23 {
                                    self.state.settings_focus = 6;
                                } else if row >= 24 {
                                    self.state.settings_focus = 7;
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
                                    if let Some(resp) = &self.state.rules_resp {
                                        if click_idx < resp.rules.len() {
                                            self.state.selected_rule_idx = click_idx;
                                        }
                                    }
                                }
                            } else if self.state.active_tab == Tab::Connections {
                                if row >= 4 {
                                    let click_idx = (row - 4) as usize;
                                    if let Some(resp) = &self.state.connections_resp {
                                        if click_idx < resp.connections.len() {
                                            self.state.selected_conn_idx = click_idx;
                                        }
                                    }
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

                    if let Some(active) = self.state.profiles.iter().find(|p| p.is_active).or_else(|| self.state.profiles.first()) {
                        if let Ok(content) = std::fs::read_to_string(&active.file_path) {
                            if let Ok(parsed) = crate::profile::ProfileParser::parse_yaml(&content) {
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
                                    let _ = crate::core::CoreProcess::start_with_config(&active.file_path);
                                }
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
                self.state.push_toast("🔐 正在验证 Sudo 密码并开启 CAP_NET_ADMIN 权限...".to_string());

                let tx = self.action_tx.clone();
                tokio::task::spawn_blocking(move || {
                    let res = crate::core::TunMode::grant_privilege_with_password(&password)
                        .map_err(|e| e.to_string());
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
                match res {
                    Ok(_) => {
                        self.state.is_tun_privileged = crate::core::TunMode::check_privilege();
                        if self.state.is_tun_privileged {
                            self.state.show_tun_modal = false;
                            self.state.push_toast("🎉 系统权限授权成功 (CAP_NET_ADMIN)".to_string());

                            let client = self.client.clone();
                            let tx = self.action_tx.clone();
                            tokio::spawn(async move {
                                if client.set_tun_enabled(true).await.is_ok() {
                                    let _ = tx.send(Action::FetchConfig).await;
                                }
                            });
                        } else {
                            self.state.push_toast("⚠️ 提权成功但未检测到 CAP_NET_ADMIN 权限".to_string());
                        }
                    }
                    Err(err) => {
                        self.state.push_toast(format!("❌ 提权失败: {}", err));
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
                match res {
                    Ok(_) => {
                        self.state.is_tun_privileged = crate::core::TunMode::check_privilege();
                        self.state.push_toast("已成功撤销 CAP_NET_ADMIN 系统权限".to_string());
                    }
                    Err(err) => {
                        self.state.push_toast(format!("❌ 撤销权限失败: {}", err));
                    }
                }
            }

            Action::ToggleTunMode => {
                if !self.state.is_tun_privileged {
                    let _ = self.action_tx.try_send(Action::ShowTunModal);
                } else {
                    let client = self.client.clone();
                    let new_state = !self.state.is_tun_enabled;
                    let tx = self.action_tx.clone();
                    tokio::spawn(async move {
                        if client.set_tun_enabled(new_state).await.is_ok() {
                            let _ = tx.send(Action::FetchConfig).await;
                        }
                    });
                    if new_state {
                        self.state.push_toast("TUN 虚拟网卡模式已开启".to_string());
                    } else {
                        self.state.push_toast("TUN 虚拟网卡模式已关闭".to_string());
                    }
                }
            }

            Action::FetchConfig => {
                self.fetch_config();
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
                cfg.tun_stack = self.state.settings_tun_stack.clone();
                cfg.log_level = self.state.settings_log_level.clone();
                cfg.allow_lan = self.state.settings_allow_lan;
                cfg.ipv6 = self.state.settings_ipv6;

                if cfg.save().is_ok() {
                    let client = self.client.clone();
                    let stack = self.state.settings_tun_stack.clone();
                    let is_tun = self.state.is_tun_enabled;
                    tokio::spawn(async move {
                        let _ = client.set_tun_config(is_tun, &stack).await;
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
                tokio::spawn(async move {
                    let client = reqwest::Client::builder()
                        .timeout(Duration::from_secs(3))
                        .build()
                        .unwrap_or_default();
                    let res = match client.get("https://api.ipify.org?format=json").send().await {
                        Ok(resp) => match resp.json::<serde_json::Value>().await {
                            Ok(v) => Ok(v["ip"].as_str().unwrap_or("Unknown").to_string()),
                            Err(e) => Err(e.to_string()),
                        },
                        Err(e) => Err(e.to_string()),
                    };
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
                    if let Some(resp) = &self.state.connections_resp {
                        if !resp.connections.is_empty() && self.state.selected_conn_idx >= resp.connections.len() {
                            self.state.selected_conn_idx = resp.connections.len() - 1;
                        }
                    }
                }
                Err(e) => self.state.status_error = Some(e),
            },

            Action::LatencyResult { node, result } => {
                let delay = result.ok();
                self.state.latency_map.insert(node, delay);
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
                    let nodes = self.state.current_group_nodes();
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
                if let Some(resp) = &self.state.connections_resp {
                    let len = resp.connections.len();
                    if len > 0 {
                        self.state.selected_conn_idx = (self.state.selected_conn_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                    }
                }
            }
            Tab::Logs => {
                let current = self.state.log_scroll as i32;
                self.state.log_scroll = (current + delta).max(0) as usize;
            }
            Tab::Settings => {
                self.state.settings_focus = (self.state.settings_focus as i32 + delta).rem_euclid(8) as usize;
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
                    let len = self.state.current_group_nodes().len();
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
                if let Some(resp) = &self.state.connections_resp {
                    let len = resp.connections.len();
                    if len > 0 { self.state.selected_conn_idx = len - 1; }
                }
            }
            Tab::Logs => {
                let len = self.state.logs.len();
                if len > 0 { self.state.log_scroll = len - 1; }
            }
            Tab::Settings => self.state.settings_focus = 7,
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
                let nodes = self.state.filtered_group_nodes();
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
        let _group = match self.state.selected_group_name() {
            Some(g) => g.to_string(),
            None => return,
        };

        let nodes = self.state.current_group_nodes();
        let client = self.client.clone();
        let tx = self.action_tx.clone();

        tokio::spawn(async move {
            for node in nodes {
                let c = client.clone();
                let t = tx.clone();
                let node_name = node.clone();
                tokio::spawn(async move {
                    let delay = c.test_delay(&node_name, None, None).await.map_err(|e| e.to_string());
                    let _ = t.send(Action::LatencyResult { node: node_name, result: delay }).await;
                });
            }
        });
    }

    fn test_single_node_latency(&self) {
        let nodes = self.state.current_group_nodes();
        if let Some(node) = nodes.get(self.state.selected_node_idx) {
            let client = self.client.clone();
            let tx = self.action_tx.clone();
            let node_name = (*node).clone();
            tokio::spawn(async move {
                let delay = client.test_delay(&node_name, None, None).await.map_err(|e| e.to_string());
                let _ = tx.send(Action::LatencyResult { node: node_name, result: delay }).await;
            });
        }
    }

    async fn close_selected_connection(&mut self) {
        let id = match &self.state.connections_resp {
            Some(resp) => match resp.connections.get(self.state.selected_conn_idx) {
                Some(conn) => conn.id.clone(),
                None => return,
            },
            None => return,
        };

        if self.client.close_connection(&id).await.is_ok() {
            self.fetch_connections();
            self.state.push_toast("Closed connection".to_string());
        }
    }
}
