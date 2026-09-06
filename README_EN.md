<div align="center">

```text
 ╭────────────────────────────────────────────────────────╮
 │   ███╗   ███╗██╗███╗   ███╗ ██████╗       ⚡ MIMO TUI  │
 │   ████╗ ████║██║████╗ ████║██╔═══██╗      v1.4.4       │
 │   ██╔████╔██║██║██╔████╔██║██║   ██║      Rust/Ratatui │
 │   ██║╚██╔╝██║██║██║╚██╔╝██║██║   ██║      <10ms · ~5MB │
 │   ██║ ╚═╝ ██║██║██║ ╚═╝ ██║╚██████╔╝                   │
 │   ╚═╝     ╚═╝╚═╝╚═╝     ╚═╝ ╚═════╝       Linux Proxy  │
 ╰────────────────────────────────────────────────────────╯
```

# ⚡ Mimo · Minimalist Modern Linux Terminal Proxy Manager

[![Release](https://img.shields.io/github/v/release/aimy1/Mimo?style=flat-square&color=cba6f7&label=Release)](https://github.com/aimy1/Mimo/releases)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-fab387?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui%20v0.29-89b4fa?style=flat-square)](https://ratatui.rs/)
[![Core](https://img.shields.io/badge/Core-Mihomo-a6e3a1?style=flat-square)](https://github.com/MetaCubeX/mihomo)
[![License](https://img.shields.io/badge/License-MIT-94e2d5?style=flat-square)](LICENSE)

<p align="center">
  <b>An Ultra-Lightweight, Lightning-Fast Linux Terminal TUI & CLI Proxy Control Center</b><br>
  Catppuccin Mocha Dark Theme · Dual Vim & Mouse Control · Linux Transparent TUN Mode · Smart Core Hot-Update
</p>

[ 🇨🇳 简体中文文档 ](README.md) &nbsp;|&nbsp; [ 🇺🇸 English Documentation (Current) ](README_EN.md)

</div>

---

## 🖥️ Interface Preview

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

## ⚡ Highlights & Key Features

- 🚀 **Ultra-Lightweight & Native Performance**: Built natively with Rust 2024 + Ratatui + Tokio. No Electron bloat: cold start **<10ms**, resident memory **~5MB**.
- 🎨 **Catppuccin Mocha Aesthetics**: Elegant dark palette, high-contrast badges, with clean layout and zero visual clutter.
- 🕹️ **Dual Input Engine (Vim Flow + Full Mouse Interaction)**: Full support for `j/k/h/l`, `g/G`, `/` search alongside smooth mouse controls (wheel scrolling, single-click select, double-click activate, right-click test/disconnect).
- 🌐 **Full-System TUN Virtual Interface Mode**: Built-in `system` / `gvisor` / `lwip` network stacks, supporting Polkit GUI privilege escalation and non-root `CAP_NET_ADMIN` operation.
- 🖥️ **Seamless Linux Desktop Integration**: Native automatic desktop proxy configuration for **KDE Plasma 5/6** (`kwriteconfig` / D-Bus) and **GNOME / Cinnamon** (`gsettings`), plus environment variable propagation.
- 🔄 **Smart Core Management & Hot-Update**: Dynamically detects the latest official GitHub release; smart multi-mirror scheduling (GitHub, ghproxy.net, gh-proxy.com, gh.ddlc.top) with direct fallback; atomic file replacement to eliminate Linux `ETXTBSY` locks, followed by automated service reboot.
- 📑 **Comprehensive Profiles & Node Handling**: Parses Clash/Mihomo YAML, Base64, and raw SS/Vmess/Vless/Trojan URIs; supports tri-mode sorting (Default / Lowest Latency / Name A-Z) and real-time fuzzy filtering.
- 🎛️ **Dual Operational Modes (Interactive TUI + Powerful CLI)**: Enjoy immersive full-terminal control or invoke one-shot commands from scripts and server workflows.

---

## 🗺️ 10 Workspace Views Guide

| Tab | View Name | Key | Description |
| :---: | :--- | :---: | :--- |
| **1** | **Dashboard** | `1` | Core runtime overview, outbound public IP, real-time speed, popular site ping latency |
| **2** | **Proxies** | `2` | Proxy groups, node browser, tri-mode sort (`o`), group & single latency tests (`t`/`d`), search (`/`) |
| **3** | **Profiles** | `3` | Subscription URL import (`a`), online auto-update (`u`), switch active profile (`Enter`), delete (`d`) |
| **4** | **Rules** | `4` | View active Mihomo routing rules, target behaviors, and full-text fuzzy filter (`/`) |
| **5** | **Connections** | `5` | Monitor active connections, process info, source & destination, instantaneous speed, drop connection (`d`) |
| **6** | **Traffic** | `6` | Real-time visual throughput graphs, upload/download historical charts, bandwidth usage metrics |
| **7** | **Logs** | `7` | Live Mihomo core log stream, severity filtering, auto-scroll lock/pause (`a`), clear buffer (`c`) |
| **8** | **Settings** | `8` | API Base URL/Secret, proxy ports, DNS modes, TLS sniffer, **Auto-update Core (Enter)**, theme presets |
| **9** | **Privileges** | `9` | TUN network privilege diagnostics, Polkit elevation, `CAP_NET_ADMIN` grant & revoke tools |
| **0** | **About** | `0` | Version information, system & kernel environment diagnostics, keybindings cheat sheet, license |

---

## 🚀 Installation Guide

### Option 1: Official One-Line Installer (Recommended)

Automatically detects your Linux distribution and architecture (`x86_64` / `aarch64` / `armv7`), downloads the latest release, and sets permissions:

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### Option 2: Arch Linux (AUR)

For Arch Linux, Manjaro, or EndeavourOS users:

```bash
yay -S mimo-bin
# or
paru -S mimo-bin
```

### Option 3: Build from Source with Cargo

```bash
# Requires Rust 2024 toolchain
git clone https://github.com/aimy1/Mimo.git && cd Mimo
cargo build --release
sudo install -m 755 target/release/mimo /usr/local/bin/mimo
```

---

## ⌨️ Controls & Keybindings Cheat Sheet

### Global Navigation & Controls

| Key / Action | Description |
| :--- | :--- |
| **`1` ~ `0`** | Direct jump to any of the 10 Workspace Tabs |
| **`Tab` / `]`** | Clockwise tab cycle / switch form input focus |
| **`h` / `l`** or **`←` / `→`** | Switch focus between Sidebar ↔ Workspace / Groups ↔ Nodes |
| **`j` / `k`** or **`↓` / `↑`** | Move cursor / scroll lists and tables |
| **`m`** | Cycle routing mode (`Rule` ➔ `Global` ➔ `Direct`) |
| **`p`** | One-key toggle Linux desktop system proxy (SysProxy) |
| **`x`** | One-key toggle **TUN Virtual Adapter** transparent proxy mode |
| **`r`** | Restart Mihomo core daemon service |
| **`?`** | Open / close keybindings help modal |
| **`q` / `Esc`** | Close current modal or quit Mimo |

### Context-Specific Keybindings

* **Proxies View**:
  * **`Enter`**: Select and activate node
  * **`t`**: Test latency for all nodes in the selected group
  * **`d`**: Test latency for the single selected node
  * **`o`**: Cycle sort modes (`Default` ➔ `Lowest Latency` ➔ `Name A-Z`)
  * **`/` or `s`**: Open real-time search filter input
* **Profiles View**:
  * **`a`**: Open "Add Subscription" input modal (Name & URL)
  * **`Enter`**: Activate selected profile and reload core
  * **`u`**: Re-download and update selected profile
  * **`d`**: Delete selected profile
* **Logs View**:
  * **`a`**: Toggle auto-scroll lock / follow tail
  * **`c`**: Clear currently buffered log entries
* **Settings View**:
  * **`Space`**: Cycle enum options (DNS Mode, Sniffer, Log Level, Language, Theme)
  * **`Enter`**: On item 15, triggers **Core check for updates and hot restart**; on bottom bar, saves settings

### 🖱️ Mouse Interaction

* **Left Click**: Select node, change active tab, click status buttons, activate inputs.
* **Double Click**: (Nodes) Immediately switch active proxy node; (Profiles) Activate subscription.
* **Right Click**: (Nodes) Instant latency test; (Connections) Instantly kill selected connection.
* **Scroll Wheel**: Smoothly scroll any list, table, log stream, or sidebar area.

---

## 🛠️ Essential CLI Command Reference

Control your proxy core seamlessly without entering the TUI:

```bash
# Inspect core health, outbound public IP, and listening ports
mimo status

# Toggle Linux Desktop system proxy (GNOME / KDE / env)
mimo sysproxy on
mimo sysproxy off --http-port 7890

# Manage TUN virtual adapter & non-root privileges
mimo tun on                     # Enable TUN transparent proxy
mimo tun off                    # Disable TUN mode
mimo tun grant                  # Grant CAP_NET_ADMIN (run TUN without sudo)
mimo tun status                 # Diagnostic report on TUN interface & privileges

# Core service management & hot-updating
mimo core update                # Dynamically fetch latest version, install & hot-restart
mimo core restart               # Restart Mihomo service
mimo core stop                  # Stop core service

# Subscription & configuration profiles
mimo profile list               # List local profiles & active status
mimo profile add "MySub" <URL>  # Download and register a subscription
mimo profile use "MySub"        # Activate profile

# Proxy groups, nodes & routing rules
mimo proxy list                 # List all proxy nodes & latencies
mimo proxy use "PROXY" "NodeName" # Switch active node in group
mimo latency                    # Benchmark node latencies
mimo rules                      # Output active routing rules table
```

---

## 📂 Configuration Paths

* **Main Configuration**: `~/.config/mimo/config.toml`
* **Managed Core Binary**: `~/.config/mimo/bin/mihomo`
* **Profiles Directory**: `~/.config/mimo/profiles/`
* **GeoIP / GeoSite Database**: `~/.config/mimo/geoip.metadb`

---

## 📄 License

This project is licensed under the [MIT License](LICENSE). Contributions, issues, and feature requests are welcome!
