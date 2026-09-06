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
  Catppuccin Mocha Modern Dark Theme · Dual Vim & Mouse Control · Full-System TUN Mode · Smart Core Hot-Update
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

## ⚡ Highlights

- **Ultra-Lightweight**: Crafted natively in pure Rust + Ratatui. Cold boot in **<10ms** with a memory footprint of just **~5MB**.
- **Modern Aesthetics**: Catppuccin Mocha color palette with crisp status badges, free from visual clutter.
- **Dual-Mode Control**: Full **Vim keyboard flow** (`j/k/h/l`, `g/G`, `/`) and **full mouse interaction** (smooth scrolling, single-click selection, double-click switching, right-click actions).
- **Full-System TUN**: Native transparent proxying for all system traffic with `system`, `gvisor`, and `lwip` network stacks and Polkit privilege management.
- **Desktop Integration**: Automatic detection and control of **KDE Plasma 5/6** and **GNOME / Cinnamon** system proxy settings.
- **Smart Core Hot-Update**: Dynamic probing of official GitHub releases, multi-mirror acceleration with direct fallback, atomic rename against `ETXTBSY`, and instant restart.

---

## 🚀 Installation & Quick Start

### Method 1: Official One-Line Installer (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### Method 2: Arch Linux (AUR)

```bash
yay -S mimo-bin
```

### Method 3: Build from Source with Cargo

```bash
git clone https://github.com/aimy1/Mimo.git && cd Mimo
cargo build --release
install -m 755 target/release/mimo ~/.local/bin/mimo
```

### Run Mimo

```bash
mimo
```

---

## ⌨️ Common Controls & Shortcuts

| Key / Action | Description |
| :--- | :--- |
| **`1` ~ `0`** | Jump directly to any of the 10 workspace tabs |
| **`Tab` / `]`** | Cycle clockwise through tabs / focus fields |
| **`j` / `k`** or **`↓` / `↑`** | Move cursor up / down or scroll list |
| **`h` / `l`** or **`←` / `→`** | Switch focus between Sidebar ↔ Workspace / Groups ↔ Nodes |
| **`Enter`** | Select node / Activate profile / Confirm action |
| **`m`** | Cycle routing mode (`Rule` ➔ `Global` ➔ `Direct`) |
| **`p`** | Toggle desktop system proxy (SysProxy) |
| **`x`** | Toggle **TUN adapter** transparent proxy mode |
| **`o`** | (Proxies) Cycle sorting (Default ➔ Latency ➔ Name A-Z) |
| **`t`** | (Proxies) Test group latencies / (Dashboard) Ping websites |
| **`d`** | (Proxies) Test node / (Connections) Close socket / (Profiles) Delete |
| **`/` or `s`** | Open fuzzy search / filter box |
| **`a`** | (Logs) Toggle auto-scroll / (Profiles) Add subscription |
| **`?`** | Open / close help modal |
| **`q`** | Quit application |
| **🖱️ Mouse** | Click to select, double-click to switch proxy, right-click to test/disconnect, wheel to scroll |

---

## 🛠️ Common CLI Commands

Control the proxy core directly from your terminal without launching the TUI:

```bash
mimo status               # Check core runtime status and public IP
mimo core update          # Auto-detect latest release, download, and hot-restart
mimo core restart         # Restart core service
mimo sysproxy on|off      # Toggle desktop system proxy
mimo tun on|off           # Toggle TUN transparent mode
mimo tun grant            # Grant CAP_NET_ADMIN (run TUN without root)
mimo profile list         # List all subscription profiles
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
