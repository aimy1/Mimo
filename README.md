# ⚡ Mimo (Minimal Mihomo Manager)

<div align="center">

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust)
![Ratatui](https://img.shields.io/badge/UI-Ratatui-blue.svg?style=for-the-badge)
![Core](https://img.shields.io/badge/Core-Mihomo%20%2F%20Clash%20Meta-red.svg?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)

<p align="center">
  <b>现代化、极轻量、极速响应的 Linux 终端 TUI & CLI 代理控制中心</b><br>
  完全对标桌面级 <b>Clash Verge Rev</b> 核心能力，拥有极致的 <b>Catppuccin Morandi</b> 暗黑极简美学
</p>

</div>

---

## 🌟 为什么选择 Mimo？ (Why Mimo?)

普通的代理客户端要么依赖庞大的 GUI 框架（占用数个 G 的内存），要么缺乏直观的交互。**Mimo** 专为 Linux 用户与终端极客打造：

- **⚡ 极致轻量与秒开**：基于 Rust + Ratatui 纯原生打造，**启动延迟 <10ms，常驻内存仅 ~5MB**，极低 CPU 占用。
- **🎨 Catppuccin 莫兰迪极简美学**：采用圆角卡片边框、电光紫与猫咪暗黑配色，提供媲美现代 GUI 应用的视觉体验。
- **🛡️ Linux 原生全系统接管**：支持 **TUN 虚拟网卡模式**（透明接管 100% 系统的 TCP/UDP/DNS 流量），具备 Polkit 图形化 / Sudo 免密提权、实时 MTU & 流量监控与协议栈切换（`system`/`gvisor`/`lwip`）。
- **⌨️ 纯净顺滑的 Vim 键盘流**：统一的 `j/k` 列表滚动、`h/l` 分栏切换、`1-8` 页面直达与全局 `/` 模糊搜索。
- **🛠️ TUI & CLI 双栖模式**：既有直观的高帧率终端图形界面，也有完整的单条命令行（CLI）快捷指令。

---

## 📊 功能模块全览 (Workspace Views)

Mimo 拥有 **8 大全功能工作区**：

1. **🌐 仪表盘 (Dashboard)**：显示 Mihomo 核心状态、**公网出口 IP 诊断**、模式/节点信息、硬件 CPU / RAM Gauges 与上下行实时 Btop 速率 Sparklines。
2. **⚡ 代理节点 (Proxies)**：全协议 Badge 标注 (`SS`/`Vmess`/`Vless`/`Trojan`/`Hysteria2`/`Tuic`)、**一键全量并发测速 (`t`)**、**单个节点独立测速 (`d`)**、**延迟升序重排 (`o`)** 与实时节点搜索 (`/`)。
3. **📁 订阅管理 (Profiles)**：支持在线订阅 URL 导入（Base64 与标准 YAML 智能解析）、配置自动下载解压、多订阅一键切换与配置热重载。
4. **🛣️ 规则路由 (Rules)**：实时查看 Mihomo 规则匹配引擎（如 `DOMAIN-SUFFIX`, `GEOIP`, `IP-CIDR`, `MATCH`）与策略组映射关系。
5. **🔌 连接监控 (Connections)**：活动 TCP/UDP 连接监控，支持实时查看目标 IP、应用进程名与链路，提供**流量降序重排 (`s`)**、**连接实时搜索 (`/`)** 与一键切断单条/全量连接。
6. **📈 流量趋势 (Traffic)**：高精上下行速率仪表盘与历史平滑趋势 Sparkline 图表。
7. **📝 运行日志 (Logs)**：基于 WebSocket 的实时日志流，支持 `[ALL]` `[INFO]` `[WARN]` `[ERR]` `[DBG]` 分级过滤与一键清空 (`c`)。
8. **⚙️ 综合设置 (Settings)**：莫兰迪分组双列卡片布局，支持配置 REST API 端口、鉴权 Token、代理端口、节点测速链接、界面刷新率、TUN 网络协议栈、日志等级、Allow LAN（局域网共享）与 IPv6 开关。

---

## 🚀 快速开始 (Getting Started)

### 1. 源码编译安装 (Cargo Install)

确保本地已安装 Rust / Cargo 环境（版本 1.80+）：

```bash
# 克隆 Mimo 仓库
git clone https://github.com/aimy1/Mimo.git
cd Mimo

# 编译并安装 release 二进制文件至系统路径 (~/.cargo/bin/mimo)
cargo install --path .
```

### 2. 启动 Mimo

安装完成后，在终端直接输入 `mimo` 即可调出 TUI 界面：

```bash
mimo
```

---

## ⌨️ 键盘交互与快捷键指南 (Keybindings)

### 📌 视图导航与跳转

| 快捷键 | 功能说明 |
| :--- | :--- |
| **`1` ~ `8`** | 直达 仪表盘 / 节点 / 订阅 / 规则 / 连接 / 流量 / 日志 / 设置 |
| **`Tab` / `Shift+Tab`** | 顺序循环切换 Tab 页面（在设置页面平滑切换表单输入焦点） |
| **`j` / `k`** 或 **`↓` / `↑`** | **统一列表滚动**：上下移动当前页面列表焦点 / 滚动表格 |
| **`h` / `l`** 或 **`←` / `→`** | 在节点选择视图中切换 **代理分组 (Left)** / **节点列表 (Right)** 分栏 |
| **`g` / `G`** | 跳转到当前列表的 **顶部 (Top)** / **底部 (Bottom)** |
| **`Enter`** | 确认选中节点 / 激活订阅 / 保存系统设置 / 退出搜索 |
| **`Esc`** | **一键复位**：退出搜索模式、关闭弹窗对话框并清空关键字 |

### ⚡ 代理与功能控制

| 快捷键 | 功能说明 |
| :--- | :--- |
| **`m`** | 循环切换 Mihomo 运行模式 (`Rule` 规则 -> `Global` 全局 -> `Direct` 直连) |
| **`p`** | 开关 Linux GNOME 桌面系统代理 (SysProxy) |
| **`x`** | 开关 **TUN 虚拟网卡模式** (全局透明接管所有流量) |
| **`r`** | 重启 Mihomo Core 内核 |
| **`t`** | (Proxies 页) 一键对当前组内所有节点发起并发延迟测试 |
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

## 🛠️ CLI 命令行模式 (Command Line Interface)

Mimo 支持无需启动 TUI 的单条终端命令行控制，适合自动化脚本或无头服务器环境：

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

## 🏗️ 架构与技术栈 (Architecture)

- **主语言**：Rust 2021
- **UI 引擎**：[Ratatui](https://github.com/ratatui/ratatui) + [Crossterm](https://github.com/crossterm-rs/crossterm)
- **网络与并发**：[Tokio](https://tokio.rs/) + [Reqwest](https://github.com/seanmonstar/reqwest) + WebSocket (`tokio-tungstenite`)
- **内核兼容**：[Mihomo Core (Clash Meta)](https://github.com/MetaCubeX/mihomo) REST API & WebSocket Protocol

---

## 📄 开源协议 (License)

本项目采用 [MIT License](LICENSE) 开源协议。欢迎提交 PR 和 Issue！
