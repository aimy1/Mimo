<div align="center">

```text
 ╭────────────────────────────────────────────────────────╮
 │   ███╗   ███╗██╗███╗   ███╗ ██████╗       ⚡ MIMO TUI  │
 │   ████╗ ████║██║████╗ ████║██╔═══██╗      v1.4.3       │
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
  Catppuccin Mocha Dark Theme · Dual Vim & Mouse Control · Linux Transparent TUN Mode
</p>

[ 🇨🇳 简体中文文档 ](README.md) &nbsp;|&nbsp; [ 🇺🇸 English Documentation (Current) ](README_EN.md)

</div>

---

## 🖥️ Interface Preview

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

## ⚡ Highlights

- **Ultra-Lightweight**: Built with native Rust + Ratatui. Cold startup **<10ms**, memory footprint **~5MB**.
- **Modern Aesthetics**: Catppuccin Mocha palette with crisp badges and zero visual clutter.
- **Dual Input Engine**: Smooth **Vim navigation** (`j/k/h/l`, `g/G`, `/`) + **Full Mouse Support** (wheel scrolling, double-click activation, right-click actions).
- **System Transparent TUN**: Transparently routes all TCP/UDP/DNS traffic with `system`/`gvisor`/`lwip` stacks & Polkit elevation.
- **Multi-Desktop Integration**: Automatic desktop proxy support for **KDE Plasma 5/6** and **GNOME / Cinnamon**.

---

## 🚀 Installation & Quick Start

### Option 1: One-Line Installer (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### Option 2: Build from Source via Cargo

```bash
git clone https://github.com/aimy1/Mimo.git && cd Mimo
cargo build --release
install -m 755 target/release/mimo ~/.local/bin/mimo
```

### Launch Mimo

```bash
mimo
```

---

## ⌨️ Controls & Keybindings Cheat Sheet

| Key / Action | Description |
| :--- | :--- |
| **`1` ~ `0`** | Direct jump to any of the 10 Workspace Tabs |
| **`Tab` / `]`** | Clockwise tab cycle / switch form input focus |
| **`j` / `k`** or **`↓` / `↑`** | Move cursor / scroll list and tables |
| **`h` / `l`** or **`←` / `→`** | Switch focus between Sidebar ↔ Workspace / Groups ↔ Nodes |
| **`Enter`** | Select node / Activate profile / Confirm action |
| **`m`** | Cycle routing mode (`Rule` ➔ `Global` ➔ `Direct`) |
| **`p`** | Toggle desktop system proxy (SysProxy) |
| **`x`** | Toggle **TUN Virtual Adapter** transparent proxy mode |
| **`o`** | (Proxies) Cycle sort mode (Default ➔ Lowest Latency ➔ Name A-Z) |
| **`t`** | (Proxies) Test group latency / (Dashboard) Test popular sites |
| **`d`** | (Proxies) Test single node / (Connections) Close socket / (Profiles) Delete profile |
| **`/` or `s`** | Open instant fuzzy search input filter |
| **`a`** | (Logs) Toggle auto-scroll lock / (Profiles) Add profile |
| **`?`** | Open / close keybindings help modal |
| **`q`** | Safe quit application |
| **🖱️ Mouse** | Click to select, double-click to switch proxy, right-click to test/close, wheel to scroll |

---

## 🛠️ Essential CLI Commands

Control your proxy core directly from terminal or scripts:

```bash
mimo status               # Check core status and outbound public IP
mimo sysproxy on|off      # Toggle desktop system proxy
mimo tun on|off           # Toggle TUN transparent proxy
mimo tun grant            # Grant CAP_NET_ADMIN (run TUN without sudo)
mimo profile list         # List all subscription profiles
mimo core restart         # Restart Mihomo core daemon
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
