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

# ⚡ Mimo · 极简现代 Linux 终端代理控制中心

[![Release](https://img.shields.io/github/v/release/aimy1/Mimo?style=flat-square&color=cba6f7&label=Release)](https://github.com/aimy1/Mimo/releases)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-fab387?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui%20v0.29-89b4fa?style=flat-square)](https://ratatui.rs/)
[![Core](https://img.shields.io/badge/Core-Mihomo%20%2F%20Clash.Meta-a6e3a1?style=flat-square)](https://github.com/MetaCubeX/mihomo)
[![License](https://img.shields.io/badge/License-MIT-94e2d5?style=flat-square)](LICENSE)

<p align="center">
  <b>极轻量、极速响应的 Linux 终端 TUI & CLI 代理控制中心</b><br>
  <b>Catppuccin Mocha 现代暗色调设计 · 键盘 Vim / 鼠标全维度双模驱动 · 全局 TUN 透明接管 · 智能内核热更新</b>
</p>

[ 🇨🇳 简体中文 (Current) ](README.md) &nbsp;|&nbsp; [ 🇺🇸 English Documentation ](README_EN.md)

</div>

---

## 🖥️ TUI 界面美学预览

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

## 🌟 核心设计理念

| 维度 | 特性亮点 |
| :--- | :--- |
| ⚡ **极致轻量与瞬时冷启动** | 纯 Rust + Tokio + Ratatui 原生打造，告别 Electron 笨重负担，**冷启动 <10ms，常驻内存仅 ~5MB** |
| 🎨 **Catppuccin 现代暗色调** | 预置 **Catppuccin Mocha**、Nord、TokyoNight、Gruvbox 4 大暗色主题，告别嵌套边框，高对比度状态徽章 |
| 🕹️ **Vim 键盘流 + 鼠标双驱** | **Vim 流** (`j/k/h/l`, `g/G`, `/`) 与**全维度鼠标**（滚轮顺滑平移、单击切换焦点、双击改切代理、右键快速测速） |
| 🛡️ **全局 TUN 虚拟网卡代理** | 原生透明接管系统全部 TCP/UDP/DNS 流量，支持 `system` / `gvisor` / `lwip` 协议栈及 Polkit 免密提权 |
| 🐧 **Linux 主流桌面深度联动** | 自动无缝适配 **KDE Plasma 5/6** (`kioslaverc` + KIO D-Bus) 与 **GNOME / Cinnamon** (`gsettings`) |
| 🔄 **智能内核管理与自动热更** | 动态探测 GitHub 官方最新 Release，多镜像调度与免代理直连回退；临时文件原子替换杜绝 `ETXTBSY` 占用，平滑重启生效 |
| 📑 **全格式订阅与智能解析** | 原生解析 Clash/Mihomo YAML、Base64 与 SS/Vmess/Vless/Trojan 直连协议；提供三模排序与模糊搜索 |
| 🎛️ **双模形态 (TUI + 强大 CLI)** | 既能享受终端全屏沉浸式 TUI，也可在 Shell 脚本中直接一行命令控制代理，桌面运维两相宜 |

---

## 📊 10 大功能工作区全景

```text
┌── [1] 仪表盘 (Dashboard) ─── 核心运行健康度 · 出口公网 IP 探测 · 常用站点 (Google/GitHub 等) 延迟 · 瞬时吞吐
├── [2] 节点选择 (Proxies) ── 协议色彩标签 · 单点/整组真连接测速 · 三模式排序 (默认/延迟升序/名称 A-Z) · 实时搜索过滤
├── [3] 订阅配置 (Profiles) ── 订阅链接导入 (`a`) · 在线一键更新 (`u`) · 活跃配置切换 (`Enter`) · 自动持久化存储
├── [4] 路由规则 (Rules) ──── 规则集全量明细 (`DOMAIN-SUFFIX`, `IP-CIDR`, `GEOIP` 等) · 全文关键词实时搜索
├── [5] 连接管理 (Connections) 活跃 TCP/UDP 套接字监控 · 来源进程与目标域名 · 瞬时与累计流量 · 单连接切断 (`d`)
├── [6] 流量监控 (Traffic) ──── 高精实时上下行速率仪表盘 · 平滑历史趋势图表 · 累计带宽流量统计
├── [7] 运行日志 (Logs) ────── WebSocket 核心实时日志流 · 严重级别过滤 (`INFO`/`WARN`/`ERR` 等) · 自动跟随滚动 (`a`)
├── [8] 系统设置 (Settings) ── API 地址与密钥 · 代理端口 · DNS 模式 (`fake-ip`/`redir-host`) · 自动升级内核 (`Enter`) · 主题切换
├── [9] 权限管理 (Privileges) ─ Linux `CAP_NET_ADMIN` 能力诊断 · Polkit GUI 提权与权限撤销 · 系统安全审计
└── [0] 关于信息 (About) ────── 软件版本与构建元数据 · 操作系统与 CPU 硬件诊断 · 快捷键导航手册 · 开源协议
```

---

## 🚀 快速开始

### 方式 1: 官方一键极速安装 (推荐)

一键自动检测 Linux 发行版与 CPU 架构（`x86_64` / `aarch64` / `armv7`），自动下载最新版本并配置可执行权限：

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### 方式 2: 各 Linux 发行版安装包

| 发行版体系 | 安装方式 | 架构支持 | 产物格式 |
| :--- | :--- | :--- | :--- |
| **Arch Linux / Manjaro** | `yay -S mimo-bin` 或 `paru -S mimo-bin` | `x86_64`, `aarch64` | `AUR (PKGBUILD)` |
| **Debian / Ubuntu / Mint** | `sudo dpkg -i mimo_1.4.4_amd64.deb` | `amd64`, `arm64`, `armhf` | `DEB` |
| **Fedora / RHEL / Rocky** | `sudo dnf install mimo-1.4.4-1.x86_64.rpm` | `x86_64`, `aarch64`, `armhfp` | `RPM` |
| **通用 Linux 二进制** | 解压 `mimo-1.4.4-x86_64.tar.gz` 复制到 `~/.local/bin/` | `x86_64`, `aarch64`, `armv7` | `Tarball` |

### 方式 3: Cargo 源码编译

```bash
# 需已配置 Rust 2024 工具链
git clone https://github.com/aimy1/Mimo.git && cd Mimo
cargo build --release
sudo install -m 755 target/release/mimo /usr/local/bin/mimo
```

### 启动运行

```bash
mimo
```

---

## 🎮 操控指南 (Controls)

### 🖱️ 鼠标交互地图 (Mouse Engine)

- **左侧导航栏**：单击直接切换页面；鼠标停留在侧边栏时滚动滚轮快速轮巡 10 大页面。
- **顶部状态胶囊**：单击或滚轮上下滚动直接切换运行模式（`Rule` / `Global` / `Direct`）。
- **代理节点面板**：
  - **单击**：选中节点 / 代理分组。
  - **双击**：立即切换并激活该代理节点。
  - **右键单击**：对鼠标所指单个节点执行独立延迟测速。
- **订阅管理面板**：点击顶部按钮（添加/更新/删除）；双击激活选中订阅；右键直接拉取更新。
- **连接管理面板**：点击表头切换排序；单击查看连接详情；双击或右键快速切断指定连接。
- **运行日志面板**：点击日志级别标签即时过滤；按 `c` 或右键清空日志；按 `a` 切换自动滚动锁定。
- **系统设置面板**：点击任意配置行即时切换开关或循环选项；在底栏点击保存条立即持久化。
- **弹窗交互体验**：点击输入框自动对焦，按 `Esc` 或点击弹窗外部半透明蒙层即可快速关闭。

---

### ⌨️ 键盘热键双列速查表

```text
╭── 导航与基础操控 ────────────────────╮╭── 代理与核心控制 ────────────────────╮
│ 1 ~ 0       快速直达 10 大主功能页面 ││ m / M       循环切换模式 (Rule/Global/Direct)│
│ Tab / ]     顺时针切页 / 切换表单焦点││ p / P       开关桌面系统代理 (SysProxy)      │
│ [           逆时针切页               ││ x / X       开关 TUN 虚拟网卡透明代理        │
│ j / k / ↓ ↑ 光标上下移动或滚动列表   ││ r / R       重启 Mihomo 核心服务             │
│ h / l / ← → 侧边栏 ↔ 工作区焦点切换  ││ t / T       (Proxies) 组测速 / (Dash) 网站测速│
│ g / G       快速跳至列表 顶部 / 底部 ││ o / O       (Proxies) 循环切换节点排序模式   │
│ Enter       确认选择 / 激活 / 保存   ││ d / D       (Proxies) 单测 / (Conn) 断开连接 │
│ / 或 s      开启实时搜索过滤输入框   ││ a / u       (Profiles) 添加 / 更新订阅       │
│ Esc         退出搜索 / 关闭当前弹窗  ││ a / c       (Logs) 锁定滚动 / 清空日志流     │
│ ?           打开 / 关闭快捷键帮助窗口││ q / Ctrl+C  安全退出 Mimo 程序               │
╰──────────────────────────────────────╯╰──────────────────────────────────────╯
```

---

## 🛠️ CLI 命令行模式速查

无需启动 TUI 界面，Mimo 提供全套极简终端子命令，非常适合编写自动切换脚本或在 SSH 无头服务器运行：

```bash
# 查看核心运行状态、出口公网 IP 与监听端口
mimo status

# 管理 TUN 虚拟网卡透明代理与系统提权
mimo tun status                         # 查看 TUN 状态与权限诊断
mimo tun on                             # 开启 TUN 透明代理模式
mimo tun off                            # 关闭 TUN 模式
mimo tun grant                          # 执行系统的 CAP_NET_ADMIN 授权 (免 root)
mimo tun revoke                         # 撤销提权能力 (setcap -r)

# 一键控制桌面系统代理 (自动识别 GNOME / KDE / Cinnamon)
mimo sysproxy on                        # 开启桌面系统代理
mimo sysproxy off                       # 关闭桌面系统代理
mimo sysproxy on --http-port 7890       # 指定端口开启

# 核心守护进程控制与智能升级
mimo core update                        # 动态探测官方最新版本、下载并热重启生效
mimo core restart                       # 重启 Mihomo 守护进程
mimo core stop                          # 停止内核服务
mimo core start                         # 启动内核服务

# 订阅配置管理
mimo profile list                       # 列出本地所有订阅配置
mimo profile add "MySub" "https://..."  # 下载并注册新订阅
mimo profile use "MySub"                # 激活指定配置
mimo profile del "MySub"                # 删除指定配置

# 节点与路由规则
mimo proxy list                         # 列出所有代理节点及延迟
mimo proxy use "PROXY" "节点名"         # 切换指定分组的活跃节点
mimo latency                            # 全量刷新节点延迟
mimo rules                              # 输出当前生效的路由规则清单
```

---

## 📂 项目架构与目录规范

### 源码模块架构

```text
src/
├── main.rs                 # 程序入口 · 命令行 CLI 解析 (Clap) 与 TUI 初始化
├── api/                    # Mihomo RESTful API 客户端 & WebSocket 实时通信
├── app/
│   ├── mod.rs              # 主事件循环 · 键盘事件调度与异步 Action 处理
│   ├── mouse.rs            # 动态坐标命中测试引擎 · 滚轮跟随与手势处理
│   ├── action.rs           # 统一应用交互 Action 定义
│   └── state.rs            # 全局响应式状态机 · 视图状态与配置模型
├── config/                 # 本地配置文件 (~/.config/mimo/config.toml) 与读写
├── core/                   # 核心管理 · 多桌面代理 (GSettings/KIO) · TUN 提权 · 动态多源下载器
├── profile/                # 订阅管理器 · Base64 & YAML 多协议智能解析器
├── ui/                     # Ratatui 终端渲染引擎 · Catppuccin 主题 · 国际化多语言
│   ├── components/         # 头部栏、底栏、侧边栏、帮助弹窗、输入弹窗、TUN 提权弹窗
│   └── views/              # 10 大工作区视图组件实现
└── commands/               # CLI 命令行子命令处理实现
```

### 本地持久化路径

* **用户配置文件**：`~/.config/mimo/config.toml`
* **托管内核二进制**：`~/.config/mimo/bin/mihomo`
* **订阅存储目录**：`~/.config/mimo/profiles/`
* **GeoIP / 规则元数据**：`~/.config/mimo/geoip.metadb`

---

## 📄 开源许可

本项目采用 [MIT License](LICENSE) 开源协议。欢迎 Star 🌟、提交 Issue 与 Pull Request 共同打造 Linux 终端最极致的代理控制体验！

