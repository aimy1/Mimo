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
    OutboundIpFetched(Result<String, String>),
    TrafficReceived(TrafficMessage),
    LogReceived(LogMessage),
    LatencyResult { node: String, result: Result<u16, String> },
    // Core Overview Actions
    FetchVersion,
    FetchConfig,
    FetchProxies,
    FetchOutboundIp,
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
    GrantTunPrivilegeWithPassword(String),
    GrantTunPrivilegePkexec,
    TunPrivilegeResult(Result<(), String>),
    RevokeTunPrivilege,
    RevokeTunPrivilegeResult(Result<(), String>),
    StartCore,
    StopCore,
    RestartCore,
    DownloadCore,
    DownloadCoreResult(Result<String, String>),
    SaveSettings,
    // Navigation / Focus
    FocusNextPane,
    FocusPrevPane,
    MoveUp,
    MoveDown,
    // Operations
    ConfirmSelectNode,
    TestSelectedGroupLatency,
    BatchTestGroupLatency,
    TestSiteLatencies,
    SiteLatencyResult { site: String, result: Result<u16, String> },
    ToggleNodeSort,
    ChangeMode(String),
    CloseSelectedConnection,
    CloseAllConnections,
    ClearLogs,
    SetLogFilter(String),
    ToggleHelp,
    ClearScreen,
}
