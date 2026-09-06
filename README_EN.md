<div align="center">

```text
 ╭────────────────────────────────────────────────────────────────────────╮
 │   ███╗   ███╗██╗███╗   ███╗ ██████╗       ⚡ MIMO TUI                  │
 │   ████╗ ████║██║████╗ ████║██╔═══██╗      Minimalist Mihomo Manager    │
 │   ██╔████╔██║██║██╔████╔██║██║   ██║      Rust 2024 · Ratatui v0.29    │
 │   ██║╚██╔╝██║██║██║╚██╔╝██║██║   ██║      Cold Start <10ms · ~5MB RAM │
 │   ██║ ╚═╝ ██║██║██║ ╚═╝ ██║╚██████╔╝                                   │
 │   ╚═╝     ╚═╝╚═╝╚═╝     ╚═╝ ╚═════╝       v1.4.4 · Modern Linux Proxy  │
 ╰────────────────────────────────────────────────────────────────────────╯
```

# ⚡ Mimo · Minimalist Modern Linux Terminal Proxy Manager

[![Release](https://img.shields.io/github/v/release/aimy1/Mimo?style=flat-square&color=cba6f7&label=Release)](https://github.com/aimy1/Mimo/releases)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-fab387?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui%20v0.29-89b4fa?style=flat-square)](https://ratatui.rs/)
[![Core](https://img.shields.io/badge/Core-Mihomo%20%2F%20Clash.Meta-a6e3a1?style=flat-square)](https://github.com/MetaCubeX/mihomo)
[![License](https://img.shields.io/badge/License-MIT-94e2d5?style=flat-square)](LICENSE)

<p align="center">
  <b>An Ultra-Lightweight, Lightning-Fast Linux Terminal TUI & CLI Proxy Control Center</b><br>
  <b>Catppuccin Mocha Modern Dark Theme · Dual Vim & Mouse Control · Full-System TUN Mode · Smart Core Hot-Update</b>
</p>

[ 🇨🇳 简体中文文档 ](README.md) &nbsp;|&nbsp; [ 🇺🇸 English Documentation (Current) ](README_EN.md)

</div>

---

## 🖥️ TUI Aesthetics Preview

```text
╭─ ⚡ MIMO 1.4.4 ─╮╭─ [Mode: RULE] ─── [SysProxy: OFF] ── [TUN: ON (gvisor)] ─── [↑ 12.4 KB/s  ↓ 1.2 MB/s] ──╮
│ 1 Dashboard     ││ Proxy Groups (3)         │ Nodes · PROXY (28 nodes) · Lowest Latency [Enter:Select | t:Test] │
│>2 Proxies       ││ ╭──────────────────────╮ │ ╭─────────────────────────────────────────────────────────────╮ │
│ 3 Profiles      ││ │ ● PROXY              │ │ │ ● [Hysteria2] 🇭🇰 Hong Kong 01 - Premium 10x       128 ms │ │
│ 4 Rules         ││ │   AUTO-SELECT        │ │ │   [Vmess]     🇯🇵 Japan 02 - BGP Direct            164 ms │ │
│ 5 Connections   ││ │   DIRECT             │ │ │   [Vless]     🇸🇬 Singapore 01 - Ultra-Fast        192 ms │ │
│ 6 Traffic       ││ │                      │ │ │   [Trojan]    🇺🇸 US 05 - Los Angeles              235 ms │ │
│ 7 Logs          ││ │                      │ │ │   [SS]        🇩🇪 Germany 01 - Backup              310 ms │ │
│ 8 Settings      ││ ╰──────────────────────╯ │ ╰─────────────────────────────────────────────────────────────╯ │
│ 9 Privileges    │╰────────────────────────────────────────────────────────────────────────────────────────────╯
│ 0 About         │╭────────────────────────────────────────────────────────────────────────────────────────────╮
╰─────────────────╯│ [Tab:Cycle] [j/k:Move] [h/l:Panes] [m:Mode] [p:Proxy] [x:TUN] [o:Sort] [/:Search] [q:Quit]  │
                   ╰────────────────────────────────────────────────────────────────────────────────────────────╯
```

---

## 🌟 Core Design Philosophy

| Dimension | Architectural Highlights |
| :--- | :--- |
| ⚡ **Extreme Performance** | Built natively in pure Rust + Tokio + Ratatui. No Electron bloat: **cold start <10ms, memory footprint ~5MB** |
| 🎨 **Catppuccin Aesthetics** | Includes 4 dark presets: **Catppuccin Mocha**, Nord, TokyoNight, and Gruvbox; no nested box clutter, high-contrast badges |
| 🕹️ **Dual Vim + Mouse Engine** | Full **Vim navigation** (`j/k/h/l`, `g/G`, `/`) + **Mouse support** (smooth wheel scrolling, double-click switch, right-click actions) |
| 🛡️ **Full Transparent TUN** | Seamlessly captures system TCP/UDP/DNS traffic with `system`/`gvisor`/`lwip` stacks & Polkit non-root elevation |
| 🐧 **Deep Linux Desktop Hooks** | Native automatic system proxy hooks for **KDE Plasma 5/6** (`kioslaverc` + KIO D-Bus) and **GNOME / Cinnamon** (`gsettings`) |
| 🔄 **Smart Core Hot-Update** | Dynamically probes GitHub latest release, multi-mirror fast download with direct fallback, atomic rename against `ETXTBSY` |
| 📑 **Multi-Format Subscriptions** | Auto-parses Clash/Mihomo YAML, Base64, and raw SS/Vmess/Vless/Trojan URIs; supports tri-mode sort & fuzzy search |
| 🎛️ **Dual Form Factor (TUI + CLI)** | Full-screen interactive TUI or headless single-line terminal execution for automation scripts and servers alike |

---

## 📊 10 Workspace Panes

```text
┌── [1] Dashboard ──── Core runtime health · Outbound public IP · Popular site latencies (Google/GitHub etc.) · Speeds
├── [2] Proxies ────── Protocol badges · Group & single latency tests · Tri-mode sorting (Default/Latency/A-Z) · Fuzzy search
├── [3] Profiles ───── Add subscription URL (`a`) · Online one-click update (`u`) · Switch active profile (`Enter`) · Persistent store
├── [4] Rules ──────── Comprehensive rule table (`DOMAIN-SUFFIX`, `IP-CIDR`, `GEOIP`, etc.) · Instant keyword search filter
├── [5] Connections ── Active TCP/UDP sockets · Process name & destination host · Speeds & transferred bytes · Close socket (`d`)
├── [6] Traffic ────── Visual real-time throughput meters · Historical bandwidth trend graphs · Peak traffic diagnostics
├── [7] Logs ───────── Real-time core log stream via WebSocket · Severity filter (`INFO`/`WARN`/`ERR` etc.) · Auto-scroll lock (`a`)
├── [8] Settings ───── API URL & Secret · Proxy ports · DNS modes (`fake-ip`/`redir-host`) · Auto-update core (`Enter`) · Themes
├── [9] Privileges ─── Linux `CAP_NET_ADMIN` diagnostics · Polkit elevation & privilege revocation · Permission auditing
└── [0] About ──────── Version info & build metadata · Host system & CPU architecture diagnostics · Keybindings guide · License
```

---

## 🚀 Quick Start

### Option 1: Official One-Line Installer (Recommended)

Automatically detects CPU architecture (`x86_64` / `aarch64` / `armv7`) and distribution, pulls the latest binary, and sets executable permissions:

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### Option 2: Distribution Packages

| Distribution | Installation Method | Architectures | Format |
| :--- | :--- | :--- | :--- |
| **Arch Linux / Manjaro** | `yay -S mimo-bin` or `paru -S mimo-bin` | `x86_64`, `aarch64` | `AUR (PKGBUILD)` |
| **Debian / Ubuntu / Mint** | `sudo dpkg -i mimo_1.4.4_amd64.deb` | `amd64`, `arm64`, `armhf` | `DEB` |
| **Fedora / RHEL / Rocky** | `sudo dnf install mimo-1.4.4-1.x86_64.rpm` | `x86_64`, `aarch64`, `armhfp` | `RPM` |
| **Universal Linux Binary** | Extract `mimo-1.4.4-x86_64.tar.gz` to `~/.local/bin/` | `x86_64`, `aarch64`, `armv7` | `Tarball` |

### Option 3: Build from Source with Cargo

```bash
# Requires Rust 2024 toolchain
git clone https://github.com/aimy1/Mimo.git && cd Mimo
cargo build --release
sudo install -m 755 target/release/mimo /usr/local/bin/mimo
```

### Launch Mimo

```bash
mimo
```

---

## 🎮 Controls & Navigation

### 🖱️ Mouse Engine Mapping

- **Sidebar Menu**: Single-click to jump to any page; scroll wheel on sidebar smoothly navigates through all 10 tabs.
- **Top Status Badges**: Click or scroll mouse wheel over the mode badge to cycle proxy modes (`Rule` / `Global` / `Direct`).
- **Proxies Panel**:
  - **Single Click**: Select a proxy group or individual node.
  - **Double Click**: Instantly switch and activate the selected proxy node.
  - **Right Click**: Perform an isolated latency benchmark test on the hovered node.
- **Profiles Panel**: Click top action buttons (Add/Update/Delete); double-click to activate; right-click to update.
- **Connections Panel**: Click header columns to sort; single-click to inspect socket; double-click or right-click to drop connection.
- **Logs Panel**: Click log severity badges to filter; press `c` or right-click to clear logs; press `a` to toggle auto-scroll lock.
- **Settings Panel**: Click any item to toggle or cycle values; click the bottom bar to persist changes immediately.
- **Modal Windows**: Click inputs to focus; press `Esc` or click outside the modal overlay to dismiss.

---

### ⌨️ Keyboard Shortcuts Cheat Sheet

```text
╭── Navigation & Basic Controls ───────╮╭── Proxy & Core Controls ─────────────╮
│ 1 ~ 0       Direct jump to 10 Tabs   ││ m / M       Cycle routing mode (Rule/Global/Direct)
│ Tab / ]     Cycle clockwise / focus  ││ p / P       Toggle Desktop System Proxy (SysProxy)│
│ [           Cycle counter-clockwise  ││ x / X       Toggle TUN Transparent Adapter │
│ j / k / ↓ ↑ Move cursor / scroll list││ r / R       Restart Mihomo Core Service    │
│ h / l / ← → Switch Sidebar ↔ Pane    ││ t / T       (Proxies) Group test / (Dash) Ping
│ g / G       Jump to Top / Bottom     ││ o / O       (Proxies) Cycle node sorting mode
│ Enter       Select / Activate / Save ││ d / D       (Proxies) Test node / (Conn) Drop
│ / or s      Open fuzzy search filter ││ a / u       (Profiles) Add / Update sub    │
│ Esc         Exit search / Close modal││ a / c       (Logs) Toggle scroll / Clear   │
│ ?           Open / close help modal  ││ q / Ctrl+C  Safely quit Mimo application   │
╰──────────────────────────────────────╯╰──────────────────────────────────────╯
```

---

## 🛠️ CLI Command Reference

Execute proxy controls directly in terminal or automation scripts without opening the TUI:

```bash
# Check core runtime status, outbound public IP, and active ports
mimo status

# Manage TUN transparent adapter & non-root permissions
mimo tun status                         # Diagnostic report on TUN interface & privileges
mimo tun on                             # Enable TUN transparent proxy
mimo tun off                            # Disable TUN mode
mimo tun grant                          # Grant CAP_NET_ADMIN privilege (run TUN without sudo)
mimo tun revoke                         # Revoke capability (setcap -r)

# Control desktop system proxy (GNOME / KDE / Cinnamon)
mimo sysproxy on                        # Enable desktop system proxy
mimo sysproxy off                       # Disable desktop system proxy
mimo sysproxy on --http-port 7890       # Specify port

# Core daemon management & automatic hot-upgrade
mimo core update                        # Dynamically probe latest version, install & hot-restart
mimo core restart                       # Restart Mihomo daemon
mimo core stop                          # Stop core daemon
mimo core start                         # Start core daemon

# Subscription & configuration management
mimo profile list                       # List local subscription profiles
mimo profile add "MySub" "https://..."  # Download and register a subscription
mimo profile use "MySub"                # Activate profile
mimo profile del "MySub"                # Delete profile

# Proxy groups, nodes & routing rules
mimo proxy list                         # List all nodes and ping latencies
mimo proxy use "PROXY" "NodeName"       # Switch active node in group
mimo latency                            # Benchmark latencies across nodes
mimo rules                              # Output active routing rules table
```

---

## 📂 Architecture & Directory Layout

### Source Code Hierarchy

```text
src/
├── main.rs                 # Program entry point · CLI argument parsing (Clap) & TUI init
├── api/                    # Mihomo RESTful API client & WebSocket streaming
├── app/
│   ├── mod.rs              # Main event loop · Keyboard dispatch & async action loop
│   ├── mouse.rs            # Dynamic hit-testing engine · Wheel tracking & click handling
│   ├── action.rs           # Unified user/system interaction action definitions
│   └── state.rs            # Reactive application state machine & configuration models
├── config/                 # Configuration file management (~/.config/mimo/config.toml)
├── core/                   # Core daemon · Desktop proxies (GSettings/KIO) · TUN · Downloader
├── profile/                # Profile manager · Multi-protocol Base64 & YAML parsers
├── ui/                     # Ratatui rendering engine · Catppuccin themes · i18n
│   ├── components/         # Header, footer, sidebar, and interactive modal dialogs
│   └── views/              # 10 Workspace view rendering implementations
└── commands/               # CLI subcommand execution handlers
```

### Local Storage Layout

* **Main Configuration**: `~/.config/mimo/config.toml`
* **Managed Core Binary**: `~/.config/mimo/bin/mihomo`
* **Profiles Directory**: `~/.config/mimo/profiles/`
* **GeoIP / GeoSite Database**: `~/.config/mimo/geoip.metadb`

---

## 📄 License

This project is licensed under the [MIT License](LICENSE). Star 🌟, issue reports, and pull requests are welcome!

