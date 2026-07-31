use crate::models::*;
use crossterm::event::KeyEvent;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Action {
    Tick,
    Key(KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Quit,
    NextTab,
    PrevTab,
    SelectTab(usize),
    // State Updates
    VersionFetched(Result<VersionInfo, String>),
    ConfigFetched(Result<MihomoConfig, String>),
    ProxiesFetched(Result<ProxiesResponse, String>),
    ConnectionsFetched(Result<ConnectionsResponse, String>),
    TrafficReceived(TrafficMessage),
    LogReceived(LogMessage),
    LatencyResult { node: String, result: Result<u16, String> },
    // Core Overview Actions
    FetchVersion,
    FetchConfig,
    FetchProxies,
    // Connections Actions
    FetchConnections,
    // Rules Actions
    FetchRules,
    RulesFetched(Result<crate::models::RulesResponse, String>),
    // Profile Actions
    FetchProfiles,
    ProfilesFetched(Result<Vec<crate::profile::ProfileItem>, String>),
    AddProfile { name: String, url: String },
    UpdateProfile(String),
    ActivateProfile(String),
    DeleteProfile(String),
    ToggleProfileInput,
    // Core & System Controls
    ToggleSystemProxy,
    ToggleTunMode,
    ShowTunModal,
    HideTunModal,
    GrantTunPrivilege,
    TunPrivilegeResult(bool),
    StartCore,
    StopCore,
    RestartCore,
    SaveSettings,
    // Navigation / Focus
    FocusNextPane,
    FocusPrevPane,
    MoveUp,
    MoveDown,
    // Operations
    ConfirmSelectNode,
    TestSelectedGroupLatency,
    ChangeMode(String),
    CloseSelectedConnection,
    CloseAllConnections,
    ClearLogs,
    SetLogFilter(String),
    ToggleHelp,
}
