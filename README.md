# ⚡ Mimo (Minimalist Mihomo Manager)

<div align="center">

![Author](https://img.shields.io/badge/Author-aisaniya-purple.svg?style=for-the-badge)
![Rust](https://img.shields.io/badge/Language-Rust--2024-orange.svg?style=for-the-badge&logo=rust)
![Ratatui](https://img.shields.io/badge/UI-Ratatui%20v0.29-blue.svg?style=for-the-badge)
![Core](https://img.shields.io/badge/Core-Mihomo%20%2F%20Clash%20Meta-red.svg?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)

<p align="center">
  <b>现代化、极轻量、极速响应的 Linux 终端 TUI & CLI 代理控制中心</b><br>
  <b>A Modern, Ultra-Lightweight Linux Terminal TUI & CLI Proxy Management Center</b><br>
  <b>Author: aisaniya</b> | 对标桌面级 <b>Clash Verge Rev</b> 核心能力 | <b>Catppuccin Morandi</b> 极简美学 | <b>100% 完整双语</b>
</p>

[ 🇨🇳 简体中文文档](#-mimo-minimalist-mihomo-manager--cn) | [ 🇺🇸 English Documentation](#-mimo-minimalist-mihomo-manager--en)

</div>

---

<a id="-mimo-minimalist-mihomo-manager--cn"></a>
## 🇨🇳 简体中文文档

### 🌟 为什么选择 Mimo？ (Why Mimo?)

普通的代理客户端要么依赖庞大的 Electron GUI 框架（占用数个 G 的内存），要么缺乏直观的交互。**Mimo** 专为 Linux 用户与终端极客打造：

- **⚡ 极致轻量与秒开**：基于 Rust 2021 + Ratatui 纯原生打造，**启动延迟 <10ms，常驻内存仅 ~5MB**，极低 CPU 占用。
- **🎨 Catppuccin 莫兰迪极简美学**：采用圆角卡片边框、电光紫与猫咪暗黑配色，提供媲美现代 GUI 应用的视觉体验。
- **🛡️ Linux 原生全系统接管**：支持 **TUN 虚拟网卡模式**（透明接管 100% 系统的 TCP/UDP/DNS 流量），具备 Polkit 图形化 / Sudo 免密提权、实时 MTU & 流量监控与协议栈切换（`system`/`gvisor`/`lwip`）。
- **⌨️ 纯净顺滑的 Vim 4 向键盘流**：支持 `j/k/h/l` 与 `↑/↓/←/→` 4 向阵列导航、侧边栏 ↔ 工作区双域交替、`1-0` 页面直达与全局 `/` 模糊搜索。
- **🌐 常用网站与服务连通性测试**：仪表盘内置 6 大常用网站（Google, GitHub, YouTube, OpenAI, Bilibili, Baidu）专属品牌配色卡片，支持按 `'t'` 键或鼠标点击并发刷新延迟。
- **⚙️ 4 大分组 16 项系统设置卡片**：包含网络与代理端口、DNS 增强解析模式（`fake-ip`/`redir-host`）、自启系统代理、订阅自动更新周期、TUN 协议栈、日志等级、局域网共享与 UI 主题预设（`Catppuccin`, `Nord`, `Tokyo Night`, `Gruvbox`），实时保存至 `~/.config/mimo/config.toml`。
- **🌐 100% 深度双语国际化 (i18n)**：所有 10 大页面、帮助模态框、侧边栏与按键引导支持中文与英文无缝切换。

---

### 📊 10 大全功能工作区 (Workspace Tabs)

1. **🌐 仪表盘 (Dashboard)**：显示 Mihomo 核心状态、**公网出口 IP 诊断**、主机与 Linux 内核信息、常用网站连通性测试卡片、与上下行实时速率 Sparklines。
2. **⚡ 节点选择 (Proxies)**：全协议 Badge 标注 (`SS`/`Vmess`/`Vless`/`Trojan`/`Hysteria2`/`Tuic`)、**一键并发测速 (`t`)**、**单节点独立测速 (`d`)**、**延迟升序重排 (`o`)** 与实时节点模糊搜索 (`/`)。
3. **📁 订阅配置 (Profiles)**：支持在线订阅 URL 导入（Base64 与标准 YAML 智能解析）、配置自动下载解压、多订阅一键切换与配置热重载。
4. **🛣️ 规则路由 (Rules)**：实时查看 Mihomo 规则匹配引擎（如 `DOMAIN-SUFFIX`, `GEOIP`, `IP-CIDR`, `MATCH`）与策略组映射关系。
5. **🔌 链接管理 (Connections)**：活动 TCP/UDP 连接监控，支持实时查看目标 IP、应用进程名与链路，提供**流量降序重排 (`s`)**、**连接实时搜索 (`/`)** 与一键切断单条/全量连接 (`Shift+D`)。
6. **📈 流量监控 (Traffic)**：高精上下行速率仪表盘与历史平滑趋势 Sparkline 图表。
7. **📝 运行日志 (Logs)**：基于 WebSocket 的实时日志流，支持 `[ALL]` `[INFO]` `[WARN]` `[ERR]` `[DBG]` 分级过滤与一键清空 (`c`)。
8. **⚙️ 系统设置 (Settings)**：4 大莫兰迪分组栅格卡片，支持微调 API 端口、Secret Token、代理端口、测速 URL、DNS 模式、自启代理、订阅更新周期、TUN 协议栈、日志等级、局域网共享、IPv6 与 UI 配色主题。
9. **🛡️ 权限管理 (Privileges)**：查看 CAP_NET_ADMIN 与 GSettings 提权状态，支持免密 Sudo 与桌面 Polkit 窗口完成 Root 授权。
10. **ℹ️ 关于 Mimo (About)**：软件版本信息、ASCII 艺术标语、构建架构元数据与开源致谢列表。

---

### 🚀 快速开始 (Getting Started)

#### 1. 源码编译安装 (Cargo Install)

确保本地已安装 Rust / Cargo 环境（版本 1.80+）：

```bash
# 克隆 Mimo 仓库
git clone https://github.com/aimy1/Mimo.git
cd Mimo

# 编译并安装 release 二进制文件至系统路径 (~/.cargo/bin/mimo)
cargo install --path .
```

#### 2. 启动 Mimo

安装完成后，在终端直接输入 `mimo` 即可调出 TUI 界面：

```bash
mimo
```

---

### ⌨️ 键盘交互与快捷键指南 (Keybindings)

| 快捷键 | 功能说明 |
| :--- | :--- |
| **`1` ~ `0`** | 直达 仪表盘 / 节点 / 订阅 / 规则 / 连接 / 流量 / 日志 / 设置 / 权限 / 关于 |
| **`Tab`** | 循环切换页面（在设置页面与弹窗框平滑切换表单输入焦点） |
| **`j` / `k`** 或 **`↓` / `↑`** | **统一列表滚动**：上下移动当前页面列表焦点 / 滚动表格 |
| **`h` / `l`** 或 **`←` / `→`** | 在侧边栏 ↔ 工作区、代理分组 ↔ 节点列表、设置卡片列之间 4 向切换焦点 |
| **`g` / `G`** | 跳转到当前列表的 **顶部 (Top)** / **底部 (Bottom)** |
| **`Enter`** | 确认选中节点 / 激活订阅 / 保存系统设置 / 退出搜索 |
| **`Esc`** | **一键复位**：退出搜索模式、关闭弹窗对话框、切回侧边栏焦点 |
| **`m`** | 循环切换 Mihomo 运行模式 (`Rule` 规则 -> `Global` 全局 -> `Direct` 直连) |
| **`p`** | 开关 Linux GNOME 桌面系统代理 (SysProxy) |
| **`x`** | 开关 **TUN 虚拟网卡模式** (全局透明接管所有流量) |
| **`r`** | 重启 Mihomo Core 内核 |
| **`t`** | (Dashboard 页) 刷新常用网站连通性 / (Proxies 页) 一键对当前组节点并发测速 |
| **`d`** | (Proxies 页) 单节点测速 / (Profiles 页) 删除订阅 / (Connections 页) 断开选中连接 |
| **`o`** | (Proxies 页) 切换节点列表**按延迟升序重新排序 (ON/OFF)** |
| **`s`** | (Connections 页) 切换连接列表**按流量降序重新排序 (ON/OFF)** |
| **`/`** | (Proxies / Connections 页) 调出实时 Searching 模糊搜索框 |
| **`a` / `u`** | (Profiles 页) **`a`** 添加新订阅 / **`u`** 更新选中订阅 |
| **`c`** | (Logs 页) 清空当前日志缓冲区 |
| **`Shift + D`** | (Connections 页) 强制切断当前所有活动 TCP/UDP 连接 |
| **`?`** | 调出/关闭全局快捷键帮助对话框 |
| **`q` / `Ctrl+C`** | 退出 Mimo |

---

### 🛠️ CLI 命令行模式 (Command Line Interface)

```bash
# 查看 Mimo 与 Mihomo 核心运行状态
mimo status

# 管理 TUN 虚拟网卡模式及提权
mimo tun status                         # 查看 TUN 网卡与提权状态
mimo tun on                             # 开启 TUN 模式
mimo tun off                            # 关闭 TUN 模式
mimo tun grant                          # 进行系统的 cap_net_admin 授权
mimo tun revoke                         # 撤销提权 (setcap -r)
mimo tun system|gvisor|lwip            # 切换 TUN 网络协议栈

# 管理桌面系统代理 (GSettings)
mimo sysproxy on
mimo sysproxy off

# 管理机场订阅
mimo profile list
mimo profile del --name "MySub"

# 管理核心进程
mimo core start
mimo core stop
mimo core restart
```

---

<a id="-mimo-minimalist-mihomo-manager--en"></a>
## 🇺🇸 English Documentation

### 🌟 Why Choose Mimo?

Traditional proxy clients rely either on heavy Electron GUI frameworks (consuming gigabytes of RAM) or lack intuitive terminal controls. **Mimo** is built specifically for Linux users and terminal power users:

- **⚡ Ultra-Lightweight & Instant Launch**: Powered natively by Rust 2021 + Ratatui, **launch latency is <10ms, RAM footprint is ~5MB**, with minimal CPU overhead.
- **🎨 Catppuccin Morandi Aesthetics**: Styled with rounded card borders, mauve highlights, and Morandi dark palette for a premium aesthetic.
- **🛡️ Native Linux System Interception**: Supports **TUN Virtual Adapter Mode** (transparently proxying 100% system TCP/UDP/DNS traffic), with Polkit GUI / Sudo authorization, realtime MTU & traffic monitoring, and protocol stack switching (`system`/`gvisor`/`lwip`).
- **⌨️ Seamless 4-Directional Keybindings**: Supports `j/k/h/l` and `↑/↓/←/→` matrix navigation, sidebar ↔ workspace focus switching, `1-0` tab shortcuts, and fuzzy searching (`/`).
- **🌐 Website & Service Connectivity Grid**: Dashboard features brand-colored cards for 6 popular sites (Google, GitHub, YouTube, OpenAI, Bilibili, Baidu) with single-key (`'t'`) or mouse click latency testing.
- **⚙️ 4 Grouped Cards Settings (16 Options)**: Configure REST API URL, secret tokens, proxy ports, test URLs, DNS mode (`fake-ip`/`redir-host`), auto-launch proxy, sub update interval, TUN stack, log level, LAN sharing, and UI themes (`Catppuccin`, `Nord`, `Tokyo Night`, `Gruvbox`), instantly saved to `~/.config/mimo/config.toml`.
- **🌐 100% Complete Dual-Language i18n**: All 10 workspace tabs, help dialogs, sidebar, and status guides feature 100% seamless Chinese & English translation switching.

---

### 📊 10 Workspace Tabs

1. **🌐 Dashboard**: Displays Mihomo core status, **outbound IP diagnostics**, system info, site connectivity grid, and realtime traffic sparklines.
2. **⚡ Proxies**: Protocol badges (`SS`/`Vmess`/`Vless`/`Trojan`/`Hysteria2`/`Tuic`), **concurrent group latency test (`t`)**, **single node test (`d`)**, **latency sorting (`o`)**, and fuzzy node search (`/`).
3. **📁 Profiles**: Online subscription URL import (Base64 & standard YAML parsing), auto fetch, profile switching, and hot reload.
4. **🛣️ Rules**: Inspect Mihomo routing rules (`DOMAIN-SUFFIX`, `GEOIP`, `IP-CIDR`, `MATCH`) and proxy target mappings.
5. **🔌 Connections**: Active TCP/UDP connection monitoring, showing destination IP, process names, and chains, with **traffic sorting (`s`)**, **fuzzy search (`/`)**, and one-key disconnection (`Shift+D`).
6. **📈 Traffic**: Realtime upload/download gauges and historical sparklines.
7. **📝 Logs**: WebSocket realtime log stream with `[ALL]` `[INFO]` `[WARN]` `[ERR]` `[DBG]` filtering and clear button (`c`).
8. **⚙️ Settings**: 4 grouped cards to adjust API URL, secret token, proxy ports, test URL, DNS mode, auto-sysproxy, sub-update interval, TUN stack, log level, LAN sharing, and UI themes.
9. **🛡️ Privileges**: Inspect CAP_NET_ADMIN and GSettings status, with password Sudo & Polkit GUI authorization.
10. **ℹ️ About**: Version metadata, ASCII banner, build targets, and acknowledgments.

---

### 🚀 Getting Started

#### Build and Install via Cargo

Ensure Rust & Cargo (1.80+) are installed:

```bash
git clone https://github.com/aimy1/Mimo.git
cd Mimo

cargo install --path .
```

#### Launch Mimo

Run the binary directly in any terminal:

```bash
mimo
```

---

### ⌨️ Keybindings Guide

| Keybinding | Function |
| :--- | :--- |
| **`1` ~ `0`** | Jump directly to Dashboard / Proxies / Profiles / Rules / Connections / Traffic / Logs / Settings / Privileges / About |
| **`Tab`** | Cycle tabs or switch input focus in settings and modals |
| **`j` / `k`** or **`↓` / `↑`** | Scroll item lists or table rows up / down |
| **`h` / `l`** or **`←` / `→`** | Switch focus between Sidebar ↔ Workspace / Groups ↔ Nodes / Setting columns |
| **`g` / `G`** | Jump to top (`g`) or bottom (`G`) of focused list |
| **`Enter`** | Confirm selection / Activate profile / Save settings / Submit input |
| **`Esc`** | Cancel search mode / Close modal / Return focus to Sidebar |
| **`m`** | Cycle operation mode (`Rule` -> `Global` -> `Direct`) |
| **`p`** | Toggle Linux GNOME Desktop System Proxy |
| **`x`** | Toggle **TUN Virtual Adapter Mode** |
| **`r`** | Restart Mihomo Core process |
| **`t`** | Refresh site connectivity (Dashboard) / Run concurrent latency test (Proxies) |
| **`d`** | Single node test (Proxies) / Delete profile (Profiles) / Disconnect connection (Connections) |
| **`o`** | Toggle sorting node list by latency (Proxies) |
| **`s`** | Toggle sorting connections by traffic usage (Connections) |
| **`/`** | Open fuzzy search bar (Proxies / Connections) |
| **`a` / `u`** | Add new profile (`a`) / Update active profile (`u`) |
| **`c`** | Clear log stream buffer (Logs) |
| **`Shift + D`** | Force close all active connections (Connections) |
| **`?`** | Toggle global help dialog |
| **`q` / `Ctrl+C`** | Quit Mimo |

---

### 🛠️ CLI Mode

```bash
mimo status                             # Check status
mimo tun status                         # Check TUN status
mimo tun on                             # Enable TUN mode
mimo tun off                            # Disable TUN mode
mimo tun grant                          # Authorize cap_net_admin
mimo sysproxy on                        # Enable System Proxy
mimo profile list                       # List profiles
mimo core restart                       # Restart core process
```

---

### 🏗️ Architecture & Tech Stack

- **Language**: Rust 2021
- **UI Engine**: [Ratatui v0.29](https://github.com/ratatui/ratatui) + [Crossterm](https://github.com/crossterm-rs/crossterm)
- **Async Runtime**: [Tokio](https://tokio.rs/) + [Reqwest](https://github.com/seanmonstar/reqwest) + WebSocket (`tokio-tungstenite`)
- **Core Compatibility**: [Mihomo Core (Clash Meta)](https://github.com/MetaCubeX/mihomo) REST API & WebSocket

---

### 📄 License

This project is licensed under the [MIT License](LICENSE). Copyright © 2026 aisaniya. Contributions, Issues, and PRs are welcome!
