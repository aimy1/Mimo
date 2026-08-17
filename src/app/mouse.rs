use crate::app::{state::FocusZone, state::ProxySubFocus, state::Tab, Action, App};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Check if a point (col, row) is within a Rect
#[inline]
pub fn rect_contains(r: Rect, col: u16, row: u16) -> bool {
    col >= r.x && col < r.x + r.width && row >= r.y && row < r.y + r.height
}

/// Calculate centered Rect for overlays and modals
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Main layout calculation matching ui::render
pub struct LayoutZones {
    pub area: Rect,
    pub sidebar: Rect,
    pub header: Rect,
    pub content: Rect,
    pub footer: Rect,
}

pub fn compute_layout_zones(area: Rect) -> LayoutZones {
    let root_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(18), // Left Sidebar Navigation
            Constraint::Min(0),     // Right Workspace Area
        ])
        .split(area);

    let workspace_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top Control Pills Bar
            Constraint::Min(0),    // Main Content View
            Constraint::Length(1), // Footer Keyhints
        ])
        .split(root_layout[1]);

    LayoutZones {
        area,
        sidebar: root_layout[0],
        header: workspace_layout[0],
        content: workspace_layout[1],
        footer: workspace_layout[2],
    }
}

/// Dispatches mouse events with dynamic layout hit-testing
pub async fn handle_mouse_event(app: &mut App, mouse: MouseEvent) -> anyhow::Result<bool> {
    let term_area = if app.state.last_area.width > 0 && app.state.last_area.height > 0 {
        app.state.last_area
    } else {
        let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
        Rect::new(0, 0, w, h)
    };

    let col = mouse.column;
    let row = mouse.row;

    // Detect double click (within 400ms at same cell)
    let is_double_click = if let Some((instant, last_col, last_row)) = app.state.last_click {
        instant.elapsed().as_millis() < 400 && last_col == col && last_row == row
    } else {
        false
    };

    if matches!(
        mouse.kind,
        MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Down(MouseButton::Right)
    ) {
        app.state.last_click = Some((std::time::Instant::now(), col, row));
    }

    // 1. Handle Active Overlays / Modals (Highest Priority)
    if app.state.show_help {
        return handle_help_modal_mouse(app, mouse, term_area);
    }
    if app.state.show_profile_input {
        return handle_profile_input_modal_mouse(app, mouse, term_area);
    }
    if app.state.show_tun_modal {
        return handle_tun_modal_mouse(app, mouse, term_area);
    }

    let zones = compute_layout_zones(term_area);

    // 2. Mouse Wheel Scroll (Context-Aware based on cursor position)
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            handle_wheel_scroll(app, 1, col, row, &zones);
            return Ok(false);
        }
        MouseEventKind::ScrollUp => {
            handle_wheel_scroll(app, -1, col, row, &zones);
            return Ok(false);
        }
        _ => {}
    }

    // 3. Mouse Clicks (Left / Right)
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            handle_left_click(app, col, row, is_double_click, &zones).await?;
        }
        MouseEventKind::Down(MouseButton::Right) => {
            handle_right_click(app, col, row, &zones).await?;
        }
        _ => {}
    }

    Ok(false)
}

// -----------------------------------------------------------------------------
// MODALS / OVERLAYS MOUSE HANDLERS
// -----------------------------------------------------------------------------

fn handle_help_modal_mouse(app: &mut App, mouse: MouseEvent, _area: Rect) -> anyhow::Result<bool> {
    if let MouseEventKind::Down(_) = mouse.kind {
        // Any click inside or outside help modal closes it
        app.state.show_help = false;
    }
    Ok(false)
}

fn handle_profile_input_modal_mouse(
    app: &mut App,
    mouse: MouseEvent,
    area: Rect,
) -> anyhow::Result<bool> {
    let popup = centered_rect(65, 45, area);
    let col = mouse.column;
    let row = mouse.row;

    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
        if !rect_contains(popup, col, row) {
            // Click outside closes modal
            app.state.show_profile_input = false;
            return Ok(false);
        }

        let inner_popup = Rect::new(
            popup.x + 1,
            popup.y + 1,
            popup.width.saturating_sub(2),
            popup.height.saturating_sub(2),
        );
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Name input
                Constraint::Length(3), // URL input
                Constraint::Min(0),    // Instructions & Buttons
            ])
            .split(inner_popup);

        if rect_contains(chunks[0], col, row) {
            app.state.profile_input_focus = 0;
        } else if rect_contains(chunks[1], col, row) {
            app.state.profile_input_focus = 1;
        } else if rect_contains(chunks[2], col, row) {
            // Buttons row: [Tab: Switch]   [Enter: Save]   [Esc: Cancel]
            let center_x = chunks[2].x + chunks[2].width / 2;
            if col < center_x.saturating_sub(8) {
                app.state.profile_input_focus = (app.state.profile_input_focus + 1) % 2;
            } else if col <= center_x + 8 {
                // Submit if non-empty
                if !app.state.profile_name_input.is_empty() && !app.state.profile_url_input.is_empty() {
                    let name = app.state.profile_name_input.trim().to_string();
                    let url = app.state.profile_url_input.trim().to_string();
                    let _ = app.action_tx.try_send(Action::AddProfile { name, url });
                    app.state.show_profile_input = false;
                }
            } else {
                app.state.show_profile_input = false;
            }
        }
    }

    Ok(false)
}

fn handle_tun_modal_mouse(app: &mut App, mouse: MouseEvent, area: Rect) -> anyhow::Result<bool> {
    let popup = centered_rect(70, 60, area);
    let col = mouse.column;
    let row = mouse.row;

    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
        if !rect_contains(popup, col, row) {
            if !app.state.is_granting_privilege {
                app.state.show_tun_modal = false;
            }
            return Ok(false);
        }

        if app.state.is_granting_privilege {
            return Ok(false);
        }

        let inner_popup = Rect::new(
            popup.x + 1,
            popup.y + 1,
            popup.width.saturating_sub(2),
            popup.height.saturating_sub(2),
        );
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(4), // Header Status Badges
                Constraint::Length(3), // Target Binary Path Card
                Constraint::Length(3), // Password Input Box
                Constraint::Length(3), // Action Buttons
                Constraint::Min(2),    // Footer Instructions
            ])
            .split(inner_popup);

        // Click Password Input Box
        if rect_contains(chunks[2], col, row) {
            app.state.tun_input_focus = 0;
        }
        // Click Action Buttons: [ Enter: 密码授权 ]   [ P: Polkit GUI ]   [ Esc: 取消 ]
        else if rect_contains(chunks[3], col, row) {
            let center_x = chunks[3].x + chunks[3].width / 2;
            if col < center_x.saturating_sub(6) {
                // Enter / Sudo Auth
                if !app.state.tun_password_input.is_empty() {
                    let pass = app.state.tun_password_input.clone();
                    let _ = app.action_tx.try_send(Action::GrantTunPrivilegeWithPassword(pass));
                } else {
                    app.state.tun_input_focus = 0;
                }
            } else if col <= center_x + 10 {
                // Polkit Dialog
                let _ = app.action_tx.try_send(Action::GrantTunPrivilegePkexec);
            } else {
                // Cancel
                app.state.show_tun_modal = false;
            }
        }
    }

    Ok(false)
}

// -----------------------------------------------------------------------------
// WHEEL SCROLL (CONTEXT AWARE)
// -----------------------------------------------------------------------------

fn handle_wheel_scroll(app: &mut App, delta: i32, col: u16, row: u16, zones: &LayoutZones) {
    // 1. Cursor over Left Sidebar -> Switch active tab
    if rect_contains(zones.sidebar, col, row) {
        let current_idx = app.state.active_tab as i32;
        let new_idx = (current_idx + delta).rem_euclid(Tab::ALL.len() as i32) as usize;
        if let Some(tab) = Tab::ALL.get(new_idx) {
            app.state.active_tab = *tab;
            app.state.focus_zone = FocusZone::Sidebar;
        }
        return;
    }

    // 2. Cursor over Top Header -> Cycle proxy mode
    if rect_contains(zones.header, col, row) {
        app.cycle_mode();
        return;
    }

    // 3. Cursor over Workspace Content -> Tab specific scrolling
    if rect_contains(zones.content, col, row) {
        match app.state.active_tab {
            Tab::Proxies => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(28), Constraint::Percentage(72)])
                    .split(zones.content);

                if rect_contains(chunks[0], col, row) {
                    // Scroll groups list
                    let len = app.state.proxy_groups.len();
                    if len > 0 {
                        app.state.selected_group_idx = (app.state.selected_group_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                        app.state.selected_node_idx = 0;
                        app.state.proxy_sub_focus = ProxySubFocus::Groups;
                    }
                } else {
                    // Scroll nodes list
                    let len = app.state.display_group_nodes().len();
                    if len > 0 {
                        app.state.selected_node_idx = (app.state.selected_node_idx as i32 + delta)
                            .clamp(0, len as i32 - 1) as usize;
                        app.state.proxy_sub_focus = ProxySubFocus::Nodes;
                    }
                }
            }
            Tab::Profiles => {
                let len = app.state.profiles.len();
                if len > 0 {
                    app.state.selected_profile_idx = (app.state.selected_profile_idx as i32 + delta)
                        .clamp(0, len as i32 - 1) as usize;
                }
            }
            Tab::Rules => {
                let count = app
                    .state
                    .rules_resp
                    .as_ref()
                    .map(|r| r.rules.len())
                    .or_else(|| app.state.parsed_active_profile.as_ref().map(|p| p.rules.len()))
                    .unwrap_or(0);
                if count > 0 {
                    app.state.selected_rule_idx = (app.state.selected_rule_idx as i32 + delta)
                        .clamp(0, count as i32 - 1) as usize;
                }
            }
            Tab::Connections => {
                let len = app.state.filtered_sorted_connections().len();
                if len > 0 {
                    app.state.selected_conn_idx = (app.state.selected_conn_idx as i32 + delta)
                        .clamp(0, len as i32 - 1) as usize;
                }
            }
            Tab::Logs => {
                let max_scroll = app.state.logs.len().saturating_sub(1) as i32;
                app.state.log_scroll = (app.state.log_scroll as i32 + delta * 2)
                    .clamp(0, max_scroll.max(0)) as usize;
            }
            Tab::Settings => {
                app.state.settings_focus = (app.state.settings_focus as i32 + delta)
                    .clamp(0, 19) as usize;
            }
            Tab::Dashboard => {
                // Trigger site latency test on wheel scroll
                app.test_all_sites_latency();
            }
            _ => {}
        }
    }
}

// -----------------------------------------------------------------------------
// LEFT CLICK HANDLER
// -----------------------------------------------------------------------------

async fn handle_left_click(
    app: &mut App,
    col: u16,
    row: u16,
    is_double_click: bool,
    zones: &LayoutZones,
) -> anyhow::Result<()> {
    // 1. Click on Sidebar
    if rect_contains(zones.sidebar, col, row) {
        // Nav list starts at row 3 (after title & version)
        let inner_y = zones.sidebar.y + 3;
        let nav_height = (Tab::ALL.len() as u16).min(zones.sidebar.height.saturating_sub(7));
        if row >= inner_y && row < inner_y + nav_height {
            let tab_idx = (row - inner_y) as usize;
            if let Some(tab) = Tab::ALL.get(tab_idx) {
                app.state.active_tab = *tab;
                app.state.focus_zone = FocusZone::Sidebar;
            }
        }
        return Ok(());
    }

    // 2. Click on Top Header Bar
    if rect_contains(zones.header, col, row) {
        let inner_header = Rect::new(
            zones.header.x + 1,
            zones.header.y + 1,
            zones.header.width.saturating_sub(2),
            zones.header.height.saturating_sub(2),
        );
        let header_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),     // Control Pills
                Constraint::Length(30), // Traffic Readout
            ])
            .split(inner_header);

        if rect_contains(header_chunks[0], col, row) {
            let rel_x = col.saturating_sub(header_chunks[0].x);
            // Mode Pill: [m]
            if rel_x < 22 {
                app.cycle_mode();
            }
            // SysProxy Pill: [p]
            else if rel_x < 46 {
                let _ = app.action_tx.try_send(Action::ToggleSystemProxy);
            }
            // TUN Pill: [x]
            else if rel_x < 68 {
                let _ = app.action_tx.try_send(Action::ToggleTunMode);
            }
            // Core restart
            else {
                let _ = app.action_tx.try_send(Action::RestartCore);
            }
        } else if rect_contains(header_chunks[1], col, row) {
            // Click traffic readout triggers speed test or refresh
            let _ = app.action_tx.try_send(Action::FetchProxies);
        }
        return Ok(());
    }

    // 3. Click on Bottom Footer Bar
    if rect_contains(zones.footer, col, row) {
        let rel_x = col.saturating_sub(zones.footer.x);
        if rel_x < 10 {
            // Zone toggle
            app.state.focus_zone = match app.state.focus_zone {
                FocusZone::Sidebar => FocusZone::Workspace,
                FocusZone::Workspace => FocusZone::Sidebar,
            };
        } else if rel_x < 22 {
            // Next tab
            let current = app.state.active_tab as usize;
            app.state.active_tab = Tab::ALL[(current + 1) % Tab::ALL.len()];
        } else if rel_x < 48 {
            app.cycle_mode();
        } else if rel_x < 58 {
            let _ = app.action_tx.try_send(Action::ToggleSystemProxy);
        } else if rel_x < 68 {
            let _ = app.action_tx.try_send(Action::ToggleTunMode);
        } else if rel_x < 78 {
            app.state.show_help = true;
        } else {
            let _ = app.action_tx.try_send(Action::Quit);
        }
        return Ok(());
    }

    // 4. Click on Main Workspace Content
    if rect_contains(zones.content, col, row) {
        app.state.focus_zone = FocusZone::Workspace;

        match app.state.active_tab {
            Tab::Dashboard => {
                handle_dashboard_click(app, col, row, zones.content).await;
            }
            Tab::Proxies => {
                handle_proxies_click(app, col, row, is_double_click, zones.content).await;
            }
            Tab::Profiles => {
                handle_profiles_click(app, col, row, is_double_click, zones.content).await;
            }
            Tab::Rules => {
                handle_rules_click(app, col, row, zones.content);
            }
            Tab::Connections => {
                handle_connections_click(app, col, row, is_double_click, zones.content).await;
            }
            Tab::Logs => {
                handle_logs_click(app, col, row, zones.content);
            }
            Tab::Settings => {
                handle_settings_click(app, col, row, zones.content);
            }
            Tab::Privileges => {
                handle_privilege_click(app, col, row, zones.content);
            }
            _ => {}
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// RIGHT CLICK HANDLER
// -----------------------------------------------------------------------------

async fn handle_right_click(app: &mut App, col: u16, row: u16, zones: &LayoutZones) -> anyhow::Result<()> {
    if rect_contains(zones.content, col, row) {
        app.state.focus_zone = FocusZone::Workspace;

        match app.state.active_tab {
            Tab::Proxies => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(28), Constraint::Percentage(72)])
                    .split(zones.content);

                // Right click on node list triggers single node latency test!
                if rect_contains(chunks[1], col, row) {
                    let right_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            if app.state.is_searching { Constraint::Length(3) } else { Constraint::Length(0) },
                            Constraint::Min(0),
                        ])
                        .split(chunks[1]);

                    let list_rect = right_chunks[1];
                    let inner_y = list_rect.y + 1;
                    if row >= inner_y && row < list_rect.y + list_rect.height.saturating_sub(1) {
                        let offset = app.state.proxies_nodes_state.offset();
                        let clicked_idx = offset + (row - inner_y) as usize;
                        let nodes = app.state.display_group_nodes();
                        if clicked_idx < nodes.len() {
                            app.state.selected_node_idx = clicked_idx;
                            app.state.proxy_sub_focus = ProxySubFocus::Nodes;
                            app.test_single_node_latency();
                        }
                    }
                }
            }
            Tab::Profiles => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)])
                    .split(zones.content);

                if rect_contains(chunks[1], col, row) {
                    let inner_y = chunks[1].y + 1;
                    if row >= inner_y && row < chunks[1].y + chunks[1].height.saturating_sub(1) {
                        let offset = app.state.profiles_state.offset();
                        let clicked_idx = offset + (row - inner_y) as usize;
                        if let Some(p) = app.state.profiles.get(clicked_idx) {
                            app.state.selected_profile_idx = clicked_idx;
                            if let Some(url) = p.url.clone() {
                                let name = p.name.clone();
                                let _ = app.action_tx.try_send(Action::AddProfile { name, url });
                            }
                        }
                    }
                }
            }
            Tab::Connections => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(5)])
                    .split(zones.content);

                if rect_contains(chunks[0], col, row) {
                    let table_rect = chunks[0];
                    let inner_y = table_rect.y + 2; // Border + table header
                    if row >= inner_y && row < table_rect.y + table_rect.height.saturating_sub(1) {
                        let offset = app.state.connections_state.offset();
                        let clicked_idx = offset + (row - inner_y) as usize;
                        let conns = app.state.filtered_sorted_connections();
                        if clicked_idx < conns.len() {
                            app.state.selected_conn_idx = clicked_idx;
                            app.close_selected_connection().await;
                        }
                    }
                }
            }
            Tab::Logs => {
                // Right click in logs view clears logs
                app.state.logs.clear();
                app.state.push_toast("Logs cleared".to_string());
            }
            _ => {}
        }
    } else {
        // Right click on sidebar switches focus zone
        app.state.focus_zone = FocusZone::Sidebar;
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// TAB SPECIFIC CLICK IMPLEMENTATIONS
// -----------------------------------------------------------------------------

async fn handle_dashboard_click(app: &mut App, _col: u16, row: u16, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // 1. Diagnostics & System Info
            Constraint::Length(5), // 2. Site Matrix
            Constraint::Min(6),    // 3. Sparklines
        ])
        .split(area);

    if rect_contains(main_chunks[1], _col, row) {
        // Site Connectivity Matrix clicked -> test all sites latency!
        app.test_all_sites_latency();
    } else if rect_contains(main_chunks[0], _col, row) {
        // Diagnostics card clicked -> cycle mode
        app.cycle_mode();
    }
}

async fn handle_proxies_click(
    app: &mut App,
    col: u16,
    row: u16,
    is_double_click: bool,
    area: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(28), Constraint::Percentage(72)])
        .split(area);

    // 1. Left Pane: Proxy Groups List
    if rect_contains(chunks[0], col, row) {
        app.state.proxy_sub_focus = ProxySubFocus::Groups;
        let inner_y = chunks[0].y + 1;
        if row >= inner_y && row < chunks[0].y + chunks[0].height.saturating_sub(1) {
            let offset = app.state.proxies_groups_state.offset();
            let clicked_idx = offset + (row - inner_y) as usize;
            if clicked_idx < app.state.proxy_groups.len() {
                app.state.selected_group_idx = clicked_idx;
                app.state.selected_node_idx = 0;
            }
        }
    }
    // 2. Right Pane: Nodes List
    else if rect_contains(chunks[1], col, row) {
        app.state.proxy_sub_focus = ProxySubFocus::Nodes;

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                if app.state.is_searching { Constraint::Length(3) } else { Constraint::Length(0) },
                Constraint::Min(0),
            ])
            .split(chunks[1]);

        let list_rect = right_chunks[1];
        let inner_y = list_rect.y + 1;
        if row >= inner_y && row < list_rect.y + list_rect.height.saturating_sub(1) {
            let offset = app.state.proxies_nodes_state.offset();
            let clicked_idx = offset + (row - inner_y) as usize;
            let nodes = app.state.display_group_nodes();

            if clicked_idx < nodes.len() {
                if app.state.selected_node_idx == clicked_idx || is_double_click {
                    app.state.selected_node_idx = clicked_idx;
                    app.confirm_selection().await;
                } else {
                    app.state.selected_node_idx = clicked_idx;
                }
            }
        }
    }
}

async fn handle_profiles_click(
    app: &mut App,
    col: u16,
    row: u16,
    is_double_click: bool,
    area: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Action buttons bar
    if rect_contains(chunks[0], col, row) {
        let rel_x = col.saturating_sub(chunks[0].x);
        // Add button [a: Add]
        if rel_x < 18 {
            app.state.profile_name_input.clear();
            app.state.profile_url_input.clear();
            app.state.profile_input_focus = 0;
            app.state.show_profile_input = true;
        }
        // Update button [u: Update]
        else if rel_x < 36 {
            if let Some(p) = app.state.profiles.get(app.state.selected_profile_idx)
                && let Some(url) = p.url.clone() {
                    let name = p.name.clone();
                    let _ = app.action_tx.try_send(Action::AddProfile { name, url });
                }
        }
        // Delete button [d: Delete]
        else if rel_x < 54 {
            if let Some(p) = app.state.profiles.get(app.state.selected_profile_idx) {
                let name = p.name.clone();
                let _ = app.action_tx.try_send(Action::DeleteProfile(name));
            }
        }
    }
    // Profiles list
    else if rect_contains(chunks[1], col, row) {
        let inner_y = chunks[1].y + 1;
        if row >= inner_y && row < chunks[1].y + chunks[1].height.saturating_sub(1) {
            let offset = app.state.profiles_state.offset();
            let clicked_idx = offset + (row - inner_y) as usize;
            if clicked_idx < app.state.profiles.len() {
                if app.state.selected_profile_idx == clicked_idx || is_double_click {
                    app.state.selected_profile_idx = clicked_idx;
                    app.confirm_selection().await;
                } else {
                    app.state.selected_profile_idx = clicked_idx;
                }
            }
        }
    }
}

fn handle_rules_click(app: &mut App, _col: u16, row: u16, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            if app.state.is_rules_searching { Constraint::Length(3) } else { Constraint::Length(0) },
            Constraint::Min(0),
        ])
        .split(area);

    let table_rect = chunks[1];
    let inner_y = table_rect.y + 2; // Border + table header
    if row >= inner_y && row < table_rect.y + table_rect.height.saturating_sub(1) {
        let offset = app.state.rules_state.offset();
        let clicked_idx = offset + (row - inner_y) as usize;
        let count = app.state.filtered_rules().len();
        if clicked_idx < count {
            app.state.selected_rule_idx = clicked_idx;
        }
    }
}

async fn handle_connections_click(
    app: &mut App,
    _col: u16,
    row: u16,
    is_double_click: bool,
    area: Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(area);

    let table_rect = chunks[0];
    let header_row = table_rect.y + 1;

    // Click on table header toggles sort
    if row == header_row {
        app.state.sort_connections_by_traffic = !app.state.sort_connections_by_traffic;
        let status = if app.state.sort_connections_by_traffic {
            "已开启连接流量降序排序"
        } else {
            "已恢复默认连接顺序"
        };
        app.state.push_toast(status.to_string());
        return;
    }

    let inner_y = table_rect.y + 2;
    if row >= inner_y && row < table_rect.y + table_rect.height.saturating_sub(1) {
        let offset = app.state.connections_state.offset();
        let clicked_idx = offset + (row - inner_y) as usize;
        let conns = app.state.filtered_sorted_connections();

        if clicked_idx < conns.len() {
            app.state.selected_conn_idx = clicked_idx;
            if is_double_click {
                app.close_selected_connection().await;
            }
        }
    }
}

fn handle_logs_click(app: &mut App, col: u16, row: u16, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Filter pills bar & action buttons
    if rect_contains(chunks[0], col, row) {
        let rel_x = col.saturating_sub(chunks[0].x);
        if rel_x < 18 {
            app.state.log_filter = "all".into();
        } else if rel_x < 26 {
            app.state.log_filter = "info".into();
        } else if rel_x < 34 {
            app.state.log_filter = "warn".into();
        } else if rel_x < 42 {
            app.state.log_filter = "error".into();
        } else if rel_x < 50 {
            app.state.log_filter = "debug".into();
        } else if rel_x < 75 {
            app.state.logs_auto_scroll = !app.state.logs_auto_scroll;
            let status = if app.state.logs_auto_scroll {
                "已开启日志自动滚动"
            } else {
                "已暂停日志自动滚动"
            };
            app.state.push_toast(status.to_string());
        } else {
            app.state.logs.clear();
            app.state.push_toast("Logs cleared".to_string());
        }
    }
}

fn handle_settings_click(app: &mut App, col: u16, row: u16, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Banner
            Constraint::Min(0),    // Grid
            Constraint::Length(3), // Save Button Bar (Item 19)
        ])
        .split(area);

    // 1. Bottom Save Bar (Item 19)
    if rect_contains(main_chunks[2], col, row) {
        app.state.settings_focus = 19;
        let _ = app.action_tx.try_send(Action::SaveSettings);
        return;
    }

    // 2. Middle 4 Cards Grid
    if rect_contains(main_chunks[1], col, row) {
        let grid_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[1]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(52), // Card 1: items 0..5
                Constraint::Percentage(48), // Card 2: items 6..10
            ])
            .split(grid_chunks[0]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(52), // Card 3: items 11..15
                Constraint::Percentage(48), // Card 4: items 16..18
            ])
            .split(grid_chunks[1]);

        // Card 1: Network & Ports (items 0..5)
        if rect_contains(left_chunks[0], col, row) {
            let inner_y = left_chunks[0].y + 1;
            if row >= inner_y && row < inner_y + 6 {
                let idx = (row - inner_y) as usize;
                app.state.settings_focus = idx;
            }
        }
        // Card 2: DNS & Automation (items 6..10)
        else if rect_contains(left_chunks[1], col, row) {
            let inner_y = left_chunks[1].y + 1;
            if row >= inner_y && row < inner_y + 5 {
                let idx = 6 + (row - inner_y) as usize;
                app.state.settings_focus = idx;
                match idx {
                    6 => {
                        app.state.settings_dns_mode = if app.state.settings_dns_mode == "fake-ip" {
                            "redir-host".into()
                        } else {
                            "fake-ip".into()
                        };
                    }
                    7 => app.state.settings_sniffing = !app.state.settings_sniffing,
                    8 => app.state.settings_tcp_concurrent = !app.state.settings_tcp_concurrent,
                    9 => app.state.settings_auto_sysproxy = !app.state.settings_auto_sysproxy,
                    10 => {
                        app.state.settings_sub_update_hours = match app.state.settings_sub_update_hours {
                            0 => 6,
                            6 => 12,
                            12 => 24,
                            _ => 0,
                        };
                    }
                    _ => {}
                }
            }
        }
        // Card 3: Core & System Controls (items 11..15)
        else if rect_contains(right_chunks[0], col, row) {
            let inner_y = right_chunks[0].y + 1;
            if row >= inner_y && row < inner_y + 5 {
                let idx = 11 + (row - inner_y) as usize;
                app.state.settings_focus = idx;
                match idx {
                    11 => {
                        app.state.settings_tun_stack = match app.state.settings_tun_stack.as_str() {
                            "system" => "gvisor".into(),
                            "gvisor" => "lwip".into(),
                            _ => "system".into(),
                        };
                    }
                    12 => {
                        app.state.settings_log_level = match app.state.settings_log_level.as_str() {
                            "info" => "warning".into(),
                            "warning" => "error".into(),
                            "error" => "debug".into(),
                            "debug" => "silent".into(),
                            _ => "info".into(),
                        };
                    }
                    13 => app.state.settings_allow_lan = !app.state.settings_allow_lan,
                    14 => app.state.settings_ipv6 = !app.state.settings_ipv6,
                    15 => {
                        let _ = app.action_tx.try_send(Action::DownloadCore);
                    }
                    _ => {}
                }
            }
        }
        // Card 4: Display & Preferences (items 16..18)
        else if rect_contains(right_chunks[1], col, row) {
            let inner_y = right_chunks[1].y + 1;
            if row >= inner_y && row < inner_y + 3 {
                let idx = 16 + (row - inner_y) as usize;
                app.state.settings_focus = idx;
                match idx {
                    16 => {
                        app.state.settings_lang = if app.state.settings_lang == "zh" {
                            "en".into()
                        } else {
                            "zh".into()
                        };
                    }
                    17 => {
                        app.state.settings_ui_theme = match app.state.settings_ui_theme.as_str() {
                            "Catppuccin" => "Nord".into(),
                            "Nord" => "TokyoNight".into(),
                            "TokyoNight" => "Gruvbox".into(),
                            _ => "Catppuccin".into(),
                        };
                    }
                    18 => {
                        app.state.settings_refresh_ms = match app.state.settings_refresh_ms {
                            500 => 1000,
                            1000 => 2000,
                            _ => 500,
                        };
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_privilege_click(app: &mut App, col: u16, row: u16, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9), // TUN card
            Constraint::Length(5), // GSettings card
            Constraint::Min(0),    // Instructions
        ])
        .split(area);

    // TUN card buttons
    if rect_contains(chunks[0], col, row) {
        let rel_x = col.saturating_sub(chunks[0].x);
        let btn_row = chunks[0].y + 5;
        if row >= btn_row && row <= btn_row + 1 {
            // [P: 授权 Root]
            if rel_x < 22 {
                app.state.show_tun_modal = true;
            }
            // [R: 撤销权限]
            else if rel_x < 42 {
                let _ = app.action_tx.try_send(Action::RevokeTunPrivilege);
            }
            // [X: TUN 开关]
            else if rel_x < 62 {
                let _ = app.action_tx.try_send(Action::ToggleTunMode);
            }
        }
    }
    // GSettings card
    else if rect_contains(chunks[1], col, row) {
        let _ = app.action_tx.try_send(Action::ToggleSystemProxy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_contains() {
        let r = Rect::new(10, 5, 20, 15);
        assert!(rect_contains(r, 10, 5));
        assert!(rect_contains(r, 29, 19));
        assert!(!rect_contains(r, 9, 5));
        assert!(!rect_contains(r, 30, 5));
        assert!(!rect_contains(r, 15, 4));
        assert!(!rect_contains(r, 15, 20));
    }

    #[test]
    fn test_centered_rect() {
        let area = Rect::new(0, 0, 100, 50);
        let popup = centered_rect(60, 40, area);
        assert_eq!(popup.width, 60);
        assert_eq!(popup.height, 20);
        assert_eq!(popup.x, 20);
        assert_eq!(popup.y, 15);
    }

    #[test]
    fn test_compute_layout_zones() {
        let area = Rect::new(0, 0, 100, 30);
        let zones = compute_layout_zones(area);

        assert_eq!(zones.sidebar.width, 18);
        assert_eq!(zones.sidebar.x, 0);
        assert_eq!(zones.sidebar.height, 30);

        assert_eq!(zones.header.x, 18);
        assert_eq!(zones.header.y, 0);
        assert_eq!(zones.header.height, 3);
        assert_eq!(zones.header.width, 82);

        assert_eq!(zones.footer.y, 29);
        assert_eq!(zones.footer.height, 1);
        assert_eq!(zones.footer.width, 82);

        assert_eq!(zones.content.y, 3);
        assert_eq!(zones.content.height, 26);
        assert_eq!(zones.content.width, 82);
    }
}
