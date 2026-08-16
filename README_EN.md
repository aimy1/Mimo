# ⚡ Mimo (Minimalist Mihomo Manager)

<div align="center">

![Author](https://img.shields.io/badge/Author-aisaniya-purple.svg?style=for-the-badge)
![Rust](https://img.shields.io/badge/Language-Rust--2024-orange.svg?style=for-the-badge&logo=rust)
![Ratatui](https://img.shields.io/badge/UI-Ratatui%20v0.29-blue.svg?style=for-the-badge)
![Core](https://img.shields.io/badge/Core-Mihomo%20%2F%20Clash%20Meta-red.svg?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)

<p align="center">
  <b>A Modern, Ultra-Lightweight Linux Terminal TUI & CLI Proxy Management Center</b><br>
  <b>Catppuccin Mocha Dark Aesthetics | Comparable to GUI Clash Clients | 100% Bilingual i18n</b>
</p>

[ 🇨🇳 简体中文文档 ](README.md) | [ 🇺🇸 English Documentation (Current) ](README_EN.md)

</div>

---

## 🌟 Key Features

- **⚡ Ultra-Lightweight & Instant Launch**: Powered natively by Rust + Ratatui, **cold start latency is <10ms, memory footprint is only ~5MB**, with near-zero CPU usage.
- **🎨 Catppuccin Mocha Aesthetics**: Re-architected with cohesive unified sidebar containers and status header pills. Zero visual clutter, no nested box-in-box borders, and smooth color hierarchies.
- **🛡️ Native Linux System Interception (TUN Mode)**: Transparently routes 100% of system TCP/UDP/DNS traffic with Polkit GUI / Sudo elevation, live interface state, and network stack switching (`system`/`gvisor`/`lwip`).
- **⌨️ Fluid Vim / Lazygit 4-Way Navigation**: Seamless `j/k/h/l` and arrow key navigation, sidebar ↔ workspace focus alternating, `1-0` tab shortcuts, and instant `/` fuzzy searching.
- **🌐 Website & Service Latency Matrix**: Built-in latency matrix for popular endpoints (Google, GitHub, YouTube, OpenAI, etc.) with concurrent one-key testing.
- **📁 Robust Subscription Management**: Supports HTTP/HTTPS subscription fetching (smart parsing of Base64 and standard YAML), instant profile activation, and hot reloading.
- **⚙️ Comprehensive Settings Dashboard**: Configure ports, enhanced DNS mode (`fake-ip`/`redir-host`), auto-launch sysproxy, update intervals, TUN stack, and UI theme presets.
- **🌐 Complete Bilingual Internationalization (i18n)**: Seamless instant switching between English and Simplified Chinese across all 10 tabs and modals.

---

## 📊 10 Workspace Tabs

| # | Tab | Description |
| :--- | :--- | :--- |
| **1** | **Dashboard** | Core process status, outbound public IP diagnostics, website latency matrix, and realtime traffic sparklines |
| **2** | **Proxies** | Protocol badges (`SS`/`Vmess`/`Vless`/`Trojan`/`Hysteria2`/`Tuic`), single/batch latency tests (`d`/`t`), latency sorting (`o`), and search (`/`) |
| **3** | **Profiles** | Subscription management (Add/Update/Delete), Base64 & YAML parsing, profile switching and hot reload |
| **4** | **Rules** | Active routing rules inspection (`DOMAIN-SUFFIX`, `GEOIP`, `IP-CIDR`, etc.) and proxy group target mappings |
| **5** | **Connections** | Live TCP/UDP connections monitor, sort by traffic (`s`), close connection (`d`), and kill all active sockets (`Shift+D`) |
| **6** | **Traffic** | High-precision realtime upload/download rate gauges and smoothed historical trend sparklines |
| **7** | **Logs** | Realtime WebSocket core logs stream with level filtering (`ALL`/`INFO`/`WARN`/`ERR`/`DBG`) and clear button (`c`) |
| **8** | **Settings** | Adjust REST API, secret tokens, proxy ports, DNS enhanced mode, auto-sysproxy, and visual themes |
| **9** | **Privileges** | Inspect Linux `CAP_NET_ADMIN` capability and Desktop GSettings proxy permissions with Sudo/Polkit authorization |
| **10** | **About** | Version info, architecture metadata, technology stack, and open source acknowledgments |

---

## 🚀 Installation & Getting Started

### 1. Build and Install via Cargo (Recommended)

Ensure Rust & Cargo (1.80+) are installed:

```bash
# Clone the repository
git clone https://github.com/aimy1/Mimo.git
cd Mimo

# Compile and install release binary to Cargo path (~/.cargo/bin/mimo)
cargo install --path .
```

### 2. Launch Mimo

Run the binary directly in your terminal:

```bash
mimo
```

---

## ⌨️ Keybindings Cheat Sheet

| Key | Action |
| :--- | :--- |
| **`1` ~ `0`** | Direct jump to any of the 10 Workspace Tabs |
| **`Tab`** | Cycle tabs / Switch input focus in settings and dialogs |
| **`j` / `k`** or **`↓` / `↑`** | Scroll items / table rows down and up |
| **`h` / `l`** or **`←` / `→`** | Switch focus between Sidebar ↔ Workspace / Groups ↔ Nodes |
| **`g` / `G`** | Jump to list Top (`g`) / Bottom (`G`) |
| **`Enter`** | Select proxy node / Activate subscription profile / Save settings |
| **`Esc`** | Exit search, close modals, or return focus to sidebar |
| **`m`** | Cycle routing mode (`Rule` -> `Global` -> `Direct`) |
| **`p`** | Toggle desktop system proxy (SysProxy) |
| **`x`** | Toggle **TUN Virtual Adapter Mode** (Global Transparent Proxy) |
| **`r`** | Restart Mihomo Core service |
| **`t`** | (Dashboard) Refresh website latency / (Proxies) Concurrent latency test for group |
| **`d`** | (Proxies) Test single node / (Profiles) Delete profile / (Connections) Close socket |
| **`o`** | (Proxies) Toggle sort nodes by latency ascending (ON/OFF) |
| **`s`** | (Connections) Toggle sort connections by traffic descending (ON/OFF) |
| **`/`** | Open instant fuzzy search input filter |
| **`a` / `u`** | (Profiles) **`a`** Add new profile / **`u`** Update selected profile |
| **`c`** | (Logs) Clear log buffer |
| **`Shift + D`** | (Connections) Force close all active network connections |
| **`?`** | Toggle keybindings help dialog |
| **`q` / `Ctrl+C`** | Quit Mimo |

---

## 🛠️ CLI Command Line Interface

Mimo can also be operated non-interactively in headless or script environments:

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

# Manage desktop system proxy (GSettings)
mimo sysproxy on
mimo sysproxy off

# Manage subscription profiles
mimo profile list
mimo profile del --name "MySub"

# Manage core process
mimo core start
mimo core stop
mimo core restart
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
