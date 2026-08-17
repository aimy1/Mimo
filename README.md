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

# ⚡ Mimo · 极简现代 Linux 终端代理控制中心

[![Release](https://img.shields.io/github/v/release/aimy1/Mimo?style=flat-square&color=cba6f7&label=Release)](https://github.com/aimy1/Mimo/releases)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-fab387?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui%20v0.29-89b4fa?style=flat-square)](https://ratatui.rs/)
[![Core](https://img.shields.io/badge/Core-Mihomo%20%2F%20Clash.Meta-a6e3a1?style=flat-square)](https://github.com/MetaCubeX/mihomo)
[![License](https://img.shields.io/badge/License-MIT-94e2d5?style=flat-square)](LICENSE)

<p align="center">
  <b>极轻量、极速响应的 Linux 终端 TUI & CLI 代理控制中心</b><br>
  <b>Catppuccin Mocha 现代暗色调设计 · 键盘 Vim / 鼠标全维度双模驱动 · 100% 深度双语国际化</b>
</p>

[ 🇨🇳 简体中文 (Current) ](README.md) &nbsp;|&nbsp; [ 🇺🇸 English Documentation ](README_EN.md)

</div>

---

## 🖥️ TUI 界面美学预览

```text
╭─ ⚡ MIMO 1.4.3 ─╮╭─ [模式: RULE] ── [系统代理: OFF] ── [TUN: ON (gvisor)] ── [↑ 12.4 KB/s  ↓ 1.2 MB/s] ──╮
│ 1 仪表盘       ││ 代理分组 (3)             │ 节点列表 · PROXY (28 节点) · 延迟升序 [Enter:切换 | t:测速]   │
│>2 节点选择     ││ ╭──────────────────────╮ │ ╭─────────────────────────────────────────────────────────╮ │
│ 3 订阅管理     ││ │ ● PROXY              │ │ │ ● [Hysteria2] 🇭🇰 香港 01 - 专线 10x          128 ms │ │
│ 4 路由规则     ││ │   AUTO-SELECT        │ │ │   [Vmess]     🇯🇵 日本 02 - 优质 BGP           164 ms │ │
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

## 🌟 核心设计理念

| 维度 | 特性亮点 |
| :--- | :--- |
| ⚡ **极致轻量与秒开** | 基于 Rust + Ratatui 纯原生打造，**冷启动耗时 <10ms，常驻内存仅 ~5MB**，极低 CPU 占用 |
| 🎨 **Catppuccin 现代美学** | 内置 Catppuccin Mocha / Nord / TokyoNight / Gruvbox 4 大暗色调预设，消除嵌套边框，高对比度状态徽章 |
| 🖱️ **键盘 / 鼠标双驱引擎** | **Vim 流** (`j/k/h/l`, `g/G`, `/`) + **鼠标交互**（动态坐标计算、智能滚轮跟随、双击切换代理、右键快捷测速） |
| 🛡️ **全系统接管 (TUN)** | 原生透明接管系统 TCP/UDP/DNS 流量，支持 `system` / `gvisor` / `lwip` 三协议栈与 Polkit / Sudo 权限管理 |
| 🐧 **多桌面环境深度适配** | 自动支持 **KDE Plasma 5 / 6** (`kioslaverc` + KIO 信号) 与 **GNOME / Cinnamon** (`gsettings`) 系统代理 |
| 🌐 **100% 深度双语 (i18n)** | 所有 10 大工作区、弹窗、诊断指标与控制台支持中英文一键无缝热切换 |

---

## 📊 10 大功能工作区

```text
┌── [1] 仪表盘 (Dashboard) ─── 核心状态 · 出口 IP 诊断 · 主流网站并发测速 · 上下行实时流量图表
├── [2] 节点选择 (Proxies) ── 协议色彩徽章 · 单点/整组测速 · 三模式排序 (默认/延迟/名称) · 协议实时搜索
├── [3] 订阅配置 (Profiles) ── 在线订阅添加/更新/删除 · Base64 & YAML 智能解析 · 配置热重载
├── [4] 路由规则 (Rules) ──── 规则匹配明细 (`DOMAIN-SUFFIX`, `GEOIP`, `IP-CIDR` 等) · 实时搜索过滤
├── [5] 连接管理 (Connections) 活动 TCP/UDP 连接监控 · 流量降序排序 · 单连接切断与全量断开
├── [6] 流量监控 (Traffic) ──── 高精上下行速率仪表盘 · 平滑历史趋势 Sparkline 视图 · 峰值统计
├── [7] 运行日志 (Logs) ────── WebSocket 核心实时日志 · 分级过滤 (`ALL`/`INFO`/`WARN`/`ERR`/`DBG`) · 自动滚动
├── [8] 系统设置 (Settings) ── 端口配置 · DNS 增强解析 (`fake-ip`/`redir-host`) · 开机自启 · 主题与语言
├── [9] 权限管理 (Privileges) ─ Linux `CAP_NET_ADMIN` 权限状态 · GSettings 代理权限 · 快捷提权授权
└── [0] 关于信息 (About) ────── 软件版本 · 系统内核架构 · 依赖技术栈与开源致谢
```

---

## 🚀 快速开始

### 方式 1: 官方一键极速安装 (推荐)

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### 方式 2: Cargo 源码编译安装

```bash
# 克隆仓库
git clone https://github.com/aimy1/Mimo.git
cd Mimo

# 编译并安装至系统路径 (~/.local/bin/mimo)
cargo build --release
install -m 755 target/release/mimo ~/.local/bin/mimo
```

### 方式 3: 各 Linux 发行版安装包

| 发行版体系 | 安装方式 | 架构支持 |
| :--- | :--- | :--- |
| **Arch Linux / Manjaro** | `yay -S mimo-bin` 或 `makepkg -si` (见 `packaging/aur`) | `x86_64`, `aarch64` |
| **Debian / Ubuntu** | `sudo dpkg -i mimo_1.4.3_amd64.deb` | `amd64`, `arm64`, `armhf` |
| **Fedora / RHEL** | `sudo dnf install mimo-1.4.3-1.x86_64.rpm` | `x86_64`, `aarch64` |
| **通用 Linux 二进制** | 解压 `mimo-1.4.3-x86_64.tar.gz` 即可直接运行 | `x86_64`, `aarch64`, `armv7` |

---

## 🎮 操控指南 (Controls)

### 🖱️ 鼠标交互地图 (Mouse Engine)

- **左侧导航栏**：单击直接切换页面；鼠标悬停时滚动滚轮快速轮巡 10 大 Tab。
- **顶部状态胶囊**：单击或滚轮上下直接切换运行模式（`Rule` / `Global` / `Direct`）。
- **代理节点面板**：
  - **单击**：选中节点 / 分组。
  - **双击 / 再次点击**：立即切换并激活该代理节点。
  - **右键单击**：对鼠标所指单个节点执行独立延迟测速。
- **订阅面板**：点击顶部按钮（添加/更新/删除）；双击激活订阅；右键更新订阅。
- **连接面板**：点击表头切换流量排序；单击查看连接详情；双击或右键快速切断指定连接。
- **日志面板**：点击日志级别标签即时过滤；点击/右键清空日志；点击切换自动滚动锁定。
- **设置面板**：点击任意配置行即时切换开关或循环选项，点击底部保存条立即持久化。
- **弹窗交互**：点击输入框自动对焦，点击弹窗外部半透明区域快速关闭。

---

### ⌨️ 键盘热键速查表 (Keyboard Shortcuts)

```text
╭── 导航与基础 ───────────────────────╮╭── 代理与核心控制 ───────────────────╮
│ 1 ~ 0       快速直达对应功能 Tab    ││ m / M       循环切换代理模式 (Rule/Global/Direct)
│ Tab / ]     顺时针切页 / 聚焦切换   ││ p / P       开关桌面系统代理 (SysProxy)│
│ [           逆时针切页              ││ x / X       开关 TUN 虚拟网卡透明代理 │
│ j / k / ↓ ↑ 列表 / 表格光标上下移动 ││ r / R       重启 Mihomo 核心服务      │
│ h / l / ← → 侧边栏 ↔ 工作区焦点切换 ││ t / T       (Proxies) 组测速 / (Dash) 网站测速
│ g / G       跳至列表 顶部 / 底部    ││ o / O       (Proxies) 循环切换节点排序模式
│ Enter       确认选择 / 激活 / 保存  ││ d / D       (Proxies) 单测 / (Conn) 断开连接
│ / 或 s      开启实时搜索过滤框      ││ Shift + D   (Connections) 切断所有活动连接
│ a / u       (Profiles) 添加 / 更新  ││ a           (Logs) 开启 / 暂停日志自动滚动
│ Esc         退出搜索 / 关闭弹窗     ││ c           (Logs) 清空日志流         │
│ ?           调出键盘快捷键帮助菜单  ││ q / Ctrl+C  安全退出程序              │
╰─────────────────────────────────────╯╰─────────────────────────────────────╯
```

---

## 🛠️ CLI 命令行模式

Mimo 完美兼容终端脚本与无桌面服务器环境：

```bash
# 查看核心运行与版本状态
mimo status

# 管理 TUN 虚拟网卡模式及提权
mimo tun status                         # 查看 TUN 状态与提权状态
mimo tun on                             # 开启 TUN 透明代理
mimo tun off                            # 关闭 TUN 模式
mimo tun grant                          # 执行系统的 cap_net_admin 授权
mimo tun revoke                         # 撤销提权 (setcap -r)
mimo tun system|gvisor|lwip            # 切换 TUN 网络协议栈

# 管理桌面系统代理 (GNOME / KDE)
mimo sysproxy on
mimo sysproxy off

# 管理订阅配置
mimo profile list
mimo profile add "MySub" "https://example.com/sub"
mimo profile del --name "MySub"

# 管理核心守护进程
mimo core start
mimo core stop
mimo core restart
```

---

## 📂 项目结构

```text
src/
├── main.rs                 # 程序入口 · 命令行 CLI 解析与 TUI 初始化
├── api/                    # Mihomo RESTful API 客户端 & WebSocket 实时通信
├── app/
│   ├── mod.rs              # 主事件循环 · 键盘调度与异步 Action 处理
│   ├── mouse.rs            # 动态坐标命中引擎 · 滚轮跟随与手势处理
│   └── state.rs            # 应用全局状态 · 响应式数据流与视图状态持久化
├── config/                 # 本地配置文件 (~/.config/mimo/config.toml) 与主题
├── core/                   # 核心进程管理 · 多桌面系统代理 · TUN 提权 · 镜像下载
├── profile/                # 订阅管理器 · Base64 & YAML 多协议智能解析器
└── ui/                     # Ratatui TUI 渲染引擎 · 组件化视图 · 国际化多语言
```

---

## 📄 开源许可

本项目基于 [MIT License](LICENSE) 协议开源。欢迎提交 Issue 与 PR 共同完善！
