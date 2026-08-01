pub mod api;
pub mod app;
pub mod commands;
pub mod config;
pub mod core;
pub mod models;
pub mod profile;
pub mod ui;

use api::MihomoClient;
use app::App;
use clap::{Parser, Subcommand};
use config::Config;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[derive(Parser)]
#[command(name = "mimo")]
#[command(author = "Mimo Developers")]
#[command(version = "0.1.0")]
#[command(about = "Minimal Mihomo Manager - Modern, keyboard-driven Linux TUI for Mihomo/Clash Meta", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Override Mihomo API URL (e.g. http://127.0.0.1:9090)
    #[arg(short, long)]
    api: Option<String>,

    /// Override Mihomo API secret token
    #[arg(short, long)]
    secret: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show Mihomo core status & version
    Status,

    /// Mihomo core service management (start/stop/restart)
    Core {
        /// Action to perform: start, stop, restart
        action: String,
    },

    /// Toggle Linux System Proxy
    Sysproxy {
        /// State: on or off
        state: String,
        /// HTTP proxy port (default 7890)
        #[arg(long, default_value_t = 7890)]
        http_port: u16,
        /// SOCKS proxy port (default 7891)
        #[arg(long, default_value_t = 7891)]
        socks_port: u16,
    },

    /// Manage TUN Mode and privileges (on, off, status, grant, revoke)
    Tun {
        /// Action or state: on, off, status, grant, revoke
        #[arg(default_value = "status")]
        action: String,
    },

    /// Manage profiles and subscription URLs
    Profile {
        #[command(subcommand)]
        action: ProfileSubCommands,
    },

    /// Manage proxy groups and nodes
    Proxy {
        #[command(subcommand)]
        action: ProxyCommands,
    },

    /// Test node latency
    Latency {
        /// Optional specific node name to test
        node: Option<String>,
    },

    /// List active Mihomo rules
    Rules,
}

#[derive(Subcommand)]
enum ProfileSubCommands {
    /// List local profiles
    List,
    /// Add or update a subscription URL
    Add {
        /// Profile name
        name: String,
        /// Subscription URL
        url: String,
    },
    /// Switch active profile
    Use {
        /// Profile name to activate
        name: String,
    },
    /// Delete a local profile
    Del {
        /// Profile name to delete
        name: String,
    },
}

#[derive(Subcommand)]
enum ProxyCommands {
    /// List all proxies or filter by group
    List {
        /// Optional group name to filter
        group: Option<String>,
    },
    /// Switch active node in a proxy group
    Use {
        /// Proxy group name (e.g. GLOBAL or ProxyGroup)
        group: String,
        /// Target node name to select
        node: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    if let Some(api) = cli.api {
        config.api_url = api;
    }
    if let Some(secret) = cli.secret {
        config.secret = Some(secret);
    }

    let client = MihomoClient::new(&config.api_url, config.secret.clone())?;

    // Handle One-shot CLI Mode
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Status => {
                commands::handle_status(&client).await?;
            }
            Commands::Core { action } => {
                commands::handle_core_action(&action).await?;
            }
            Commands::Sysproxy { state, http_port, socks_port } => {
                let enable = state.eq_ignore_ascii_case("on") || state.eq_ignore_ascii_case("true") || state == "1";
                commands::handle_sysproxy_toggle(enable, http_port, socks_port).await?;
            }
            Commands::Tun { action } => {
                commands::handle_tun_action(&client, &action).await?;
            }
            Commands::Profile { action } => match action {
                ProfileSubCommands::List => {
                    commands::handle_profile_list().await?;
                }
                ProfileSubCommands::Add { name, url } => {
                    commands::handle_profile_add(&name, &url).await?;
                }
                ProfileSubCommands::Use { name } => {
                    commands::handle_profile_use(&client, &name).await?;
                }
                ProfileSubCommands::Del { name } => {
                    commands::handle_profile_del(&name).await?;
                }
            },
            Commands::Proxy { action } => match action {
                ProxyCommands::List { group } => {
                    commands::handle_proxy_list(&client, group.as_deref()).await?;
                }
                ProxyCommands::Use { group, node } => {
                    commands::handle_proxy_use(&client, &group, &node).await?;
                }
            },
            Commands::Latency { node } => {
                commands::handle_latency(&client, node.as_deref()).await?;
            }
            Commands::Rules => {
                commands::handle_rules_list(&client).await?;
            }
        }
        return Ok(());
    }

    // Launch Ratatui TUI Mode
    run_tui(config, client).await
}

async fn run_tui(_config: Config, _client: MihomoClient) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
    let res = app.run(&mut terminal).await;

    // Clean terminal state on exit
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    println!("Mimo closed.");
    res
}
