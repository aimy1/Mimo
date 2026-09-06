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

# ⚡ Mimo · 极简现代 Linux 终端代理控制中心

[![Release](https://img.shields.io/github/v/release/aimy1/Mimo?style=flat-square&color=cba6f7&label=Release)](https://github.com/aimy1/Mimo/releases)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-fab387?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui%20v0.29-89b4fa?style=flat-square)](https://ratatui.rs/)
[![Core](https://img.shields.io/badge/Core-Mihomo-a6e3a1?style=flat-square)](https://github.com/MetaCubeX/mihomo)
[![License](https://img.shields.io/badge/License-MIT-94e2d5?style=flat-square)](LICENSE)

<p align="center">
  <b>极轻量、极速响应的 Linux 终端 TUI & CLI 代理控制中心</b><br>
  Catppuccin Mocha 现代暗色调 · 键盘 Vim / 鼠标双驱动 · Linux 全局 TUN 透明代理 · 智能内核热更新
</p>

[ 🇨🇳 简体中文 (Current) ](README.md) &nbsp;|&nbsp; [ 🇺🇸 English Documentation ](README_EN.md)

</div>

---

## 🖥️ 界面预览

```text
╭─ ⚡ MIMO 1.4.4 ─╮╭─ [模式: RULE] ── [系统代理: OFF] ── [TUN: ON (gvisor)] ── [↑ 12.4 KB/s  ↓ 1.2 MB/s] ──╮
│ 1 仪表盘       ││ 代理分组 (3)             │ 节点列表 · PROXY (28 节点) · 延迟升序 [Enter:切换 | t:测速]   │
│>2 节点选择     ││ ╭──────────────────────╮ │ ╭─────────────────────────────────────────────────────────╮ │
│ 3 订阅配置     ││ │ ● PROXY              │ │ │ ● [Hysteria2] 🇭🇰 香港 01 - 专线 10x          128 ms │ │
│ 4 规则路由     ││ │   AUTO-SELECT        │ │ │   [Vmess]     🇯🇵 日本 02 - 优质 BGP           164 ms │ │
│ 5 连接管理     ││ │   DIRECT             │ │ │   [Vless]     🇸🇬 新加坡 01 - 超高速           192 ms │ │
│ 6 流量监控     ││ │                      │ │ │   [Trojan]    🇺🇸 美国 05 - 洛杉矶             235 ms │ │
│ 7 运行日志     ││ │                      │ │ │   [SS]        🇩🇪 德国 01 - 备份节点           310 ms │ │
│ 8 系统设置     ││ ╰──────────────────────╯ │ ╰─────────────────────────────────────────────────────────╯ │
│ 9 权限管理     │╰────────────────────────────────────────────────────────────────────────────────────────╯
│ 0 关于信息     │╭────────────────────────────────────────────────────────────────────────────────────────╮
╰────────────────╯│ [Tab:切页] [j/k:移动] [h/l:窗格] [m:模式] [p:代理] [x:TUN] [o:排序] [/:搜索] [?:帮助] [q:退出] │
                  ╰────────────────────────────────────────────────────────────────────────────────────────╯
```

---

## ⚡ 核心特性亮点

- 🚀 **极致轻量 & 原生性能**：基于 Rust 2024 + Ratatui + Tokio 原生打造，告别臃肿的 Electron 消耗，冷启动 **<10ms**，常驻内存仅 **~5MB**。
- 🎨 **Catppuccin Mocha 美学设计**：优雅暗色系调色板，高对比度状态徽章，告别层叠边框视觉干扰。
- 🕹️ **Vim 键盘流 + 全维度鼠标交互**：完美支持 `j/k/h/l`、`g/G`、`/` 搜索以及原生鼠标操作（滚轮智能滚动、单击选中、双击激活代理、右键一键测速或断连）。
- 🌐 **全系统 TUN 虚拟网卡透明代理**：内置 `system` / `gvisor` / `lwip` 网络栈，支持 Polkit GUI 提权与免 root `CAP_NET_ADMIN` 授权，无缝接管终端与全局应用流量。
- 🖥️ **主流 Linux 桌面环境代理联动**：自动原生适配 **KDE Plasma 5/6** (`kwriteconfig` / D-Bus) 与 **GNOME / Cinnamon** (`gsettings`)，同时支持全局终端环境变量联动。
- 🔄 **智能内核管理与自动热更新**：动态探测 GitHub 最新 Release，多镜像源（GitHub、ghproxy.net、gh-proxy.com、gh.ddlc.top）智能竞速与免代理直连回退；临时文件原子替换避免 Linux `ETXTBSY` 文件占用冲突，更新后自动重载服务。
- 📑 **全功能订阅与节点管理**：支持 Clash/Mihomo 标准 YAML、Base64 订阅及 SS/Vmess/Vless/Trojan 协议解析；支持默认 / 延迟升序 / 字典序三模排序与实时关键词过滤。
- 🎛️ **双模运行 (TUI + 强大 CLI)**：既能享受沉浸式终端界面，也可在命令行脚本中直接单次调用，兼顾桌面日常与服务器运维。

---

## 🗺️ 10 大功能面板一览

| 序号 | 页面名称 | 快捷键 | 功能说明 |
| :---: | :--- | :---: | :--- |
| **1** | **仪表盘** (Dashboard) | `1` | 核心运行状态、出口公网 IP、实时吞吐速率、主流站点连通延迟测试 |
| **2** | **节点选择** (Proxies) | `2` | 代理分组切换、节点列表浏览、三模排序轮转 (`o`)、全量/单节点测速 (`t`/`d`)、实时过滤 (`/`) |
| **3** | **订阅配置** (Profiles) | `3` | 订阅链接解析导入 (`a`)、在线一键更新、活动配置切换 (`Enter`)、配置删除 (`d`) |
| **4** | **规则路由** (Rules) | `4` | 实时查看 Mihomo 路由规则集、生效行为标注、全文模糊搜索过滤 (`/`) |
| **5** | **连接管理** (Connections) | `5` | 查看当前所有网络连接、来源进程与目标地址、上行下行速率、选中连接一键断开 (`d`) |
| **6** | **流量监控** (Traffic) | `6` | 动态终端流量历史图表、累计吞吐计量与瞬时传输统计 |
| **7** | **运行日志** (Logs) | `7` | Mihomo 核心实时运行日志捕获、级别过滤、自动滚动锁定/暂停 (`a`) |
| **8** | **系统设置** (Settings) | `8` | 核心 API 地址/Secret、代理端口、DNS 模式、嗅探开关、**内核自动升级 (Enter)**、主题预设与多语言切换 |
| **9** | **权限管理** (Privileges) | `9` | TUN 虚拟网卡权限诊断、一键配置 Polkit / `CAP_NET_ADMIN` 免密运行能力 |
| **0** | **关于信息** (About) | `0` | 项目版本、系统软硬件环境诊断、快捷键概览与项目许可 |

---

## 🚀 安装指南

### 方式 1: 官方一键安装 (推荐)

一键自动检测 Linux 发行版与 CPU 架构（x86_64 / aarch64 / armv7），自动下载最新版本并配置可执行权限：

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### 方式 2: Arch Linux (AUR)

Arch Linux / Manjaro / EndeavourOS 用户可通过 AUR 助手直接安装：

```bash
yay -S mimo-bin
# 或
paru -S mimo-bin
```

### 方式 3: Cargo 源码编译

```bash
# 确保已安装 Rust 工具链 (edition 2024)
git clone https://github.com/aimy1/Mimo.git && cd Mimo
cargo build --release
sudo install -m 755 target/release/mimo /usr/local/bin/mimo
```

---

## ⌨️ 常用操控速查

### 全局导航与核心开关

| 按键 / 操作 | 功能说明 |
| :--- | :--- |
| **`1` ~ `0`** | 数字键直达 10 大主功能页面 |
| **`Tab` / `]`** | 顺时针快速切页 / 切换表单输入焦点 |
| **`h` / `l`** 或 **`←` / `→`** | 侧边栏 ↔ 工作区焦点切换 / 代理分组 ↔ 节点列表窗格切换 |
| **`j` / `k`** 或 **`↓` / `↑`** | 光标上下移动或列表滚动 |
| **`m`** | 循环切换核心代理模式 (`Rule` 规则 ➔ `Global` 全局 ➔ `Direct` 直连) |
| **`p`** | 一键切换 Linux 桌面系统代理 (SysProxy) |
| **`x`** | 一键切换 **TUN 虚拟网卡** 透明代理模式 |
| **`r`** | 重启 Mihomo 核心服务 |
| **`?`** | 弹出 / 关闭键盘快捷键帮助窗口 |
| **`q` / `Esc`** | 退出当前弹窗或退出 Mimo 程序 |

### 页面专属快捷键

* **节点选择页**：
  * **`Enter`**：选中并激活代理节点
  * **`t`**：测速当前选中分组的所有节点
  * **`d`**：对当前光标下的单节点进行延迟测试
  * **`o`**：循环切换节点排序模式（`默认顺序` ➔ `延迟升序` ➔ `名称 A-Z`）
  * **`/` 或 `s`**：开启节点名称实时搜索过滤框
* **订阅管理页**：
  * **`a`**：弹出添加订阅配置窗口 (输入名称与 URL)
  * **`Enter`**：激活选中配置文件并重载核心
  * **`u`**：重新从远端下载更新选中订阅
  * **`d`**：删除选中的订阅配置
* **运行日志页**：
  * **`a`**：开启 / 暂停日志末尾自动跟踪滚动
  * **`c`**：清空当前已捕获的日志缓冲区
* **系统设置页**：
  * **`Space` (空格)**：快速轮转枚举项（DNS 模式、嗅探开关、日志级别、语言、主题等）
  * **`Enter`**：在第 15 项触发**内核自动检查更新并热重启**；在底栏保存全部设置

### 🖱️ 鼠标交互支持

* **左键单击**：选中节点、切换菜单、激活窗格或点击状态按钮。
* **左键双击**：(节点列表) 直接切换代理节点；(订阅列表) 直接激活订阅。
* **右键单击**：(节点列表) 单节点延迟测速；(连接列表) 立即切断该网络连接。
* **滚轮滚动**：在任何列表、日志或侧边栏区域均可丝滑平移。

---

## 🛠️ 常用 CLI 命令行速查

无需启动 TUI，也可在终端或自动化脚本中无缝调用 Mimo CLI 控制代理核心：

```bash
# 查看核心运行状态、出口公网 IP 与监听端口
mimo status

# 一键开关 Linux 桌面系统代理 (GNOME / KDE / 环境变量)
mimo sysproxy on
mimo sysproxy off --http-port 7890

# 管理 TUN 虚拟网卡与免密权限
mimo tun on                     # 开启 TUN 模式
mimo tun off                    # 关闭 TUN 模式
mimo tun grant                  # 赋予 CAP_NET_ADMIN (免 root 运行 TUN)
mimo tun status                 # 查看网卡状态与权限诊断

# 核心服务控制与自动更新
mimo core update                # 自动检测官方最新版本、下载并热重启生效
mimo core restart               # 重启 Mihomo 服务
mimo core stop                  # 停止内核

# 订阅与配置管理
mimo profile list               # 查看本地订阅与激活状态
mimo profile add "MySub" <URL>  # 下载并添加订阅
mimo profile use "MySub"        # 激活指定订阅

# 节点与规则管理
mimo proxy list                 # 列出所有代理节点与测速结果
mimo proxy use "PROXY" "节点名"  # 切换指定分组的活跃节点
mimo latency                    # 全量测速节点延迟
mimo rules                      # 输出当前生效的路由规则清单
```

---

## 📂 存储与配置路径

* **主配置文件**：`~/.config/mimo/config.toml`
* **自动下载内核**：`~/.config/mimo/bin/mihomo`
* **订阅存储目录**：`~/.config/mimo/profiles/`
* **GeoIP / GeoSite 数据库**：`~/.config/mimo/geoip.metadb`

---

## 📄 开源许可

本项目采用 [MIT License](LICENSE) 开源协议。欢迎提交 Issue 与 Pull Request 共同打造最极致的 Linux 代理控制体验！
