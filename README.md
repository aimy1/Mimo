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

# ⚡ Mimo · 极简现代 Linux 终端代理控制中心

[![Release](https://img.shields.io/github/v/release/aimy1/Mimo?style=flat-square&color=cba6f7&label=Release)](https://github.com/aimy1/Mimo/releases)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-fab387?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui%20v0.29-89b4fa?style=flat-square)](https://ratatui.rs/)
[![Core](https://img.shields.io/badge/Core-Mihomo-a6e3a1?style=flat-square)](https://github.com/MetaCubeX/mihomo)
[![License](https://img.shields.io/badge/License-MIT-94e2d5?style=flat-square)](LICENSE)

<p align="center">
  <b>极轻量、极速响应的 Linux 终端 TUI & CLI 代理控制中心</b><br>
  Catppuccin Mocha 现代暗色调 · 键盘 Vim / 鼠标双驱 · Linux 全局 TUN 透明代理
</p>

[ 🇨🇳 简体中文 (Current) ](README.md) &nbsp;|&nbsp; [ 🇺🇸 English Documentation ](README_EN.md)

</div>

---

## 🖥️ 界面预览

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

## ⚡ 核心亮点

- **极致轻量**：纯 Rust + Ratatui 原生打造，冷启动 **<10ms**，常驻内存仅 **~5MB**。
- **现代美学**：Catppuccin Mocha 优雅配色，高对比度状态徽章，告别嵌套边框干扰。
- **双模操控**：支持 **Vim 键盘流**（`j/k/h/l`, `g/G`, `/`）与**全维度鼠标交互**（滚轮切页、单击选中、双击切换代理、右键测速）。
- **全系统 TUN**：原生透明接管系统全部流量，支持 `system` / `gvisor` / `lwip` 协议栈及 Polkit 提权。
- **多桌面适配**：自动适配 **KDE Plasma 5/6** 与 **GNOME / Cinnamon** 桌面系统代理设置。

---

## 🚀 安装与运行

### 方式 1: 官方一键安装 (推荐)

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

### 方式 2: Cargo 编译安装

```bash
git clone https://github.com/aimy1/Mimo.git && cd Mimo
cargo build --release
install -m 755 target/release/mimo ~/.local/bin/mimo
```

### 启动运行

```bash
mimo
```

---

## ⌨️ 常用操控速查

| 按键 / 操作 | 功能说明 |
| :--- | :--- |
| **`1` ~ `0`** | 快速直达 10 大主功能页面 |
| **`Tab` / `]`** | 顺时针切页 / 切换表单焦点 |
| **`j` / `k`** 或 **`↓` / `↑`** | 上下移动光标或滚动列表 |
| **`h` / `l`** 或 **`←` / `→`** | 侧边栏 ↔ 工作区 / 代理分组 ↔ 节点列表 焦点切换 |
| **`Enter`** | 切换选中节点 / 激活订阅 / 确认操作 |
| **`m`** | 循环切换模式 (`Rule` 规则 ➔ `Global` 全局 ➔ `Direct` 直连) |
| **`p`** | 开关桌面系统代理 (SysProxy) |
| **`x`** | 开关 **TUN 虚拟网卡** 透明代理模式 |
| **`o`** | (节点页) 循环切换排序 (默认 ➔ 延迟升序 ➔ 名称 A-Z) |
| **`t`** | (节点页) 组节点测速 / (仪表盘) 常用网站测速 |
| **`d`** | (节点页) 单节点测速 / (连接页) 断开连接 / (订阅页) 删除订阅 |
| **`/` 或 `s`** | 开启实时搜索过滤框 |
| **`a`** | (日志页) 开启/暂停自动滚动 / (订阅页) 添加订阅 |
| **`?`** | 打开/关闭快捷键帮助窗口 |
| **`q`** | 退出程序 |
| **🖱️ 鼠标操作** | 单击选中、双击切换代理、右键测速/断连、滚轮智能滚动 |

---

## 🛠️ 常用 CLI 命令

无需启动 TUI，也可在终端直接控制代理核心：

```bash
mimo status               # 查看核心运行状态与公网 IP
mimo sysproxy on|off      # 开关桌面系统代理
mimo tun on|off           # 开关 TUN 透明网卡模式
mimo tun grant            # 授权 CAP_NET_ADMIN (免 root 运行 TUN)
mimo profile list         # 列出所有订阅配置
mimo core restart         # 重启内核服务
```

---

## 📄 开源许可

本项目采用 [MIT License](LICENSE) 开源协议。
