<div align="center">

```text
 ╭────────────────────────────────────────────────────────────────────────╮
 │   ███╗   ███╗██╗███╗   ███╗ ██████╗       ⚡ MINIMALIST MIHOMO MANAGER │
 │   ████╗ ████║██║████╗ ████║██╔═══██╗      Linux Terminal TUI & CLI    │
 │   ██╔████╔██║██║██╔████╔██║██║   ██║      Catppuccin Mocha · Rust     │
 │   ██║╚██╔╝██║██║██║╚██╔╝██║██║   ██║      Cold Start <10ms · ~5MB RAM │
 │   ██║ ╚═╝ ██║██║██║ ╚═╝ ██║╚██████╔╝                                  │
 │   ╚═╝     ╚═╝╚═╝╚═╝     ╚═╝ ╚═════╝       v1.4.3 · Modern Linux Proxy  │
 ╰────────────────────────────────────────────────────────────────────────╯
```

# ⚡ Mimo · Minimalist Modern Linux Terminal Proxy Manager

[![Release](https://img.shields.io/github/v/release/aimy1/Mimo?style=flat-square&color=cba6f7&label=Release)](https://github.com/aimy1/Mimo/releases)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-fab387?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui%20v0.29-89b4fa?style=flat-square)](https://ratatui.rs/)
[![Core](https://img.shields.io/badge/Core-Mihomo%20%2F%20Clash.Meta-a6e3a1?style=flat-square)](https://github.com/MetaCubeX/mihomo)
[![License](https://img.shields.io/badge/License-MIT-94e2d5?style=flat-square)](LICENSE)

<p align="center">
  <b>An Ultra-Lightweight, Lightning-Fast Linux Terminal TUI & CLI Proxy Management Center</b><br>
  <b>Catppuccin Mocha Dark Aesthetics · Dual Vim Keyboard & Full Mouse Engine · 100% Bilingual i18n</b>
</p>

[ 🇨🇳 简体中文文档 ](README.md) &nbsp;|&nbsp; [ 🇺🇸 English Documentation (Current) ](README_EN.md)

</div>

---

## 🖥️ TUI Aesthetic Preview

```text
╭─ ⚡ MIMO 1.4.3 ─╮╭─ [Mode: RULE] ─── [SysProxy: OFF] ── [TUN: ON (gvisor)] ─── [↑ 12.4 KB/s  ↓ 1.2 MB/s] ──╮
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

## 🌟 Design Philosophy & Highlights

| Pillar | Highlights |
| :--- | :--- |
| ⚡ **Instant & Ultra-Lightweight** | Built with native Rust + Ratatui. **Cold start latency <10ms, memory footprint ~5MB**, near-zero CPU idle. |
| 🎨 **Catppuccin Aesthetics** | Includes Catppuccin Mocha, Nord, TokyoNight, and Gruvbox presets. Border clutter eliminated with clean status badges. |
| 🖱️ **Dual Input Engine** | **Vim Workflow** (`j/k/h/l`, `g/G`, `/`) + **Complete Mouse Interactions** (dynamic hit-testing, context-aware scrolling, double-click activation, right-click actions). |
| 🛡️ **Transparent System TUN** | Transparently intercepts 100% TCP/UDP/DNS traffic with `system`, `gvisor`, and `lwip` network stacks & Polkit / Sudo privilege workflows. |
| 🐧 **Multi-Desktop Integration** | Seamless automatic integration with **KDE Plasma 5 / 6** (`kioslaverc` + KIO signals) & **GNOME / Cinnamon** (`gsettings`). |
| 🌐 **100% Bilingual i18n** | Full instant hot-switching between English and Simplified Chinese across all 10 tabs, dialogs, and metrics. |

---

## 📊 10 Feature Workspaces

```text
┌── [1] Dashboard ───── Core status · Outbound IP diagnosis · Multi-site latency matrix · Realtime traffic graphs
├── [2] Proxies ─────── Protocol color badges · Single/group delay tests · Tri-mode sorting (Default/Latency/Name) · Protocol search
├── [3] Profiles ────── Online subscription management · Smart Base64 & YAML parser · Hot reload & activation
├── [4] Rules ───────── Detailed routing rules inspection (`DOMAIN-SUFFIX`, `GEOIP`, `IP-CIDR`, etc.) · Live filter
├── [5] Connections ─── Live TCP/UDP socket monitor · Sort by traffic · Single socket close & kill all connections
├── [6] Traffic ─────── High-precision realtime speed gauges · Smoothed historical sparklines · Peak stats
├── [7] Logs ────────── Realtime WebSocket core logs · Multi-level filtering (`ALL`/`INFO`/`WARN`/`ERR`/`DBG`) · Auto-scroll
├── [8] Settings ────── REST API & secret tokens · Port config · DNS enhanced mode (`fake-ip`/`redir-host`) · Themes
├── [9] Privileges ──── Linux `CAP_NET_ADMIN` capability · Desktop GSettings & KIO permissions · One-click elevation
└── [0] About ───────── Software version · Kernel architecture metadata · Technology stack & acknowledgments
```

---

## 🚀 Quick Start

### Option 1: Official One-Line Installer (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### Option 2: Build from Source via Cargo

```bash
# Clone the repository
git clone https://github.com/aimy1/Mimo.git
cd Mimo

# Compile and install to system path (~/.local/bin/mimo)
cargo build --release
install -m 755 target/release/mimo ~/.local/bin/mimo
```

### Option 3: Linux Distribution Packages

| Distro Family | Installation Method | Architecture |
| :--- | :--- | :--- |
| **Arch Linux / Manjaro** | `yay -S mimo-bin` or `makepkg -si` (in `packaging/aur`) | `x86_64`, `aarch64` |
| **Debian / Ubuntu** | `sudo dpkg -i mimo_1.4.3_amd64.deb` | `amd64`, `arm64`, `armhf` |
| **Fedora / RHEL** | `sudo dnf install mimo-1.4.3-1.x86_64.rpm` | `x86_64`, `aarch64` |
| **Universal Binary** | Extract `mimo-1.4.3-x86_64.tar.gz` and run | `x86_64`, `aarch64`, `armv7` |

---

## 🎮 Controls & Interactions

### 🖱️ Mouse Engine

- **Sidebar Navigation**: Click to jump directly to any tab; hover and scroll mouse wheel to cycle through all 10 tabs.
- **Header Status Pills**: Click or scroll mouse wheel to cycle routing modes (`Rule` / `Global` / `Direct`).
- **Proxies Pane**:
  - **Click**: Select node or group.
  - **Double Click / Re-click**: Immediately activate/switch proxy node.
  - **Right Click**: Run single node latency test.
- **Profiles Pane**: Click top buttons (Add/Update/Delete); double click to activate profile; right click to update subscription.
- **Connections Pane**: Click table header to toggle traffic sorting; click row to inspect; double click or right click to close connection.
- **Logs Pane**: Click level filter tags to filter; click/right click to clear logs; click auto-scroll pill to toggle lock.
- **Settings Pane**: Click any setting row to toggle/cycle values instantly; click bottom bar to save config.
- **Modals**: Click input field to focus, click outside translucent backdrop to dismiss.

---

### ⌨️ Keybindings Cheat Sheet

```text
╭── Navigation & Basics ───────────────╮╭── Core & Proxy Controls ────────────╮
│ 1 ~ 0       Direct jump to tabs      ││ m / M       Cycle routing mode (Rule/Global/Direct)
│ Tab / ]     Clockwise cycle / focus  ││ p / P       Toggle system proxy (SysProxy)│
│ [           Counter-clockwise cycle  ││ x / X       Toggle TUN transparent proxy │
│ j / k / ↓ ↑ Move list/table cursor   ││ r / R       Restart Mihomo Core service  │
│ h / l / ← → Switch Sidebar ↔ Pane    ││ t / T       (Proxies) Group test / (Dash) Site test
│ g / G       Jump to Top / Bottom     ││ o / O       (Proxies) Cycle node sort mode
│ Enter       Confirm / Select / Save  ││ d / D       (Proxies) Test node / (Conn) Close socket
│ / or s      Open live search bar     ││ Shift + D   (Connections) Kill all active sockets
│ a / u       (Profiles) Add / Update  ││ a           (Logs) Toggle auto-scroll lock
│ Esc         Exit search / Dismiss    ││ c           (Logs) Clear log stream      │
│ ?           Toggle keymap help modal ││ q / Ctrl+C  Safe quit application        │
╰──────────────────────────────────────╯╰─────────────────────────────────────╯
```

---

## 🛠️ Headless CLI Interface

Mimo can be scripted and operated in headless environments without TUI:

```bash
# Check status
mimo status

# Manage TUN virtual adapter & capabilities
mimo tun status                         # View TUN status and capability
mimo tun on                             # Enable TUN mode
mimo tun off                            # Disable TUN mode
mimo tun grant                          # Grant cap_net_admin capability
mimo tun revoke                         # Revoke capability (setcap -r)
mimo tun system|gvisor|lwip            # Switch TUN network protocol stack

# Manage desktop system proxy (GNOME / KDE)
mimo sysproxy on
mimo sysproxy off

# Manage subscription profiles
mimo profile list
mimo profile add "MySub" "https://example.com/sub"
mimo profile del --name "MySub"

# Manage core process
mimo core start
mimo core stop
mimo core restart
```

---

## 📂 Project Architecture

```text
src/
├── main.rs                 # Entry point · CLI parsing and TUI runtime bootstrap
├── api/                    # Mihomo RESTful API client & WebSocket real-time engine
├── app/
│   ├── mod.rs              # Main event loop · keybinding dispatcher & async actions
│   ├── mouse.rs            # Dynamic layout hit-testing engine · mouse & wheel handlers
│   └── state.rs            # Application global state & persistent view offsets
├── config/                 # Configuration (~/.config/mimo/config.toml) & theme palettes
├── core/                   # Core daemon lifecycle · multi-desktop proxy · TUN & mirrors
├── profile/                # Subscription manager · Base64 & YAML multi-protocol parser
└── ui/                     # Ratatui rendering engine · modular components · i18n
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE). Contributions, issues, and PRs are welcome!
