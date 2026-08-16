# ⚡ Mimo (Minimalist Mihomo Manager)

<div align="center">

![Author](https://img.shields.io/badge/Author-aisaniya-purple.svg?style=for-the-badge)
![Rust](https://img.shields.io/badge/Language-Rust--2024-orange.svg?style=for-the-badge&logo=rust)
![Ratatui](https://img.shields.io/badge/UI-Ratatui%20v0.29-blue.svg?style=for-the-badge)
![Core](https://img.shields.io/badge/Core-Mihomo%20%2F%20Clash%20Meta-red.svg?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)

<p align="center">
  <b>现代化、极轻量、极速响应的 Linux 终端 TUI & CLI 代理控制中心</b><br>
  <b>Catppuccin Mocha 现代暗色调设计 | 对标桌面级 Clash Verge Rev 核心能力 | 100% 完整双语</b>
</p>

[ 🇨🇳 简体中文 (Current) ](README.md) | [ 🇺🇸 English Documentation ](README_EN.md)

</div>

---

## 🌟 核心特性

- **⚡ 极致轻量与秒开**：基于 Rust + Ratatui 纯原生打造，**冷启动耗时 <10ms，常驻内存仅 ~5MB**，极低 CPU 资源占用。
- **🎨 Catppuccin Mocha 现代美学**：采用全新重构的单体流线型侧边栏与状态胶囊，去繁化消除嵌套多层边框，色调温和不刺眼，专注信息层级与视觉舒适度。
- **🛡️ Linux 原生全系统接管 (TUN 模式)**：透明接管系统全部 TCP/UDP/DNS 流量，支持 Polkit 窗口 / Sudo 快捷提权、实时网卡状态与协议栈切换（`system`/`gvisor`/`lwip`）。
- **⌨️ 纯净顺滑的 Vim / Lazygit 键盘流**：支持 `j/k/h/l` 与 `↑/↓/←/→` 四向无缝导航，侧边栏 ↔ 工作区双域交替、`1-0` 页面直达与全局 `/` 快速搜索。
- **🌐 常用网站连通性与测速矩阵**：仪表盘集成 Google、GitHub、YouTube、OpenAI 等主流服务连通性诊断，支持一键并发测速。
- **📁 强大的订阅管理**：支持 HTTP/HTTPS 在线订阅解析（Base64 与标准 YAML 智能识别）、多配置一键激活与热重载。
- **⚙️ 完备的系统设置卡片**：涵盖网络端口、DNS 增强解析模式（`fake-ip`/`redir-host`）、自启系统代理、TUN 协议栈与 UI 主题配置。
- **🌐 100% 深度双语国际化 (i18n)**：所有 10 大工作区、弹窗与操作提示均支持中文与英文无缝切换。

---

## 📊 10 大功能工作区

| 序号 | 工作区 | 功能描述 |
| :--- | :--- | :--- |
| **1** | **仪表盘 (Dashboard)** | 核心运行状态、公网出口 IP 诊断、常用网站连通性测试、上下行实时速率图表 |
| **2** | **节点选择 (Proxies)** | 协议徽章展示 (`SS`/`Vmess`/`Vless`/`Trojan`/`Hysteria2`/`Tuic`)、节点测速 (`d`/`t`)、延迟排序 (`o`) 与实时搜索 (`/`) |
| **3** | **订阅配置 (Profiles)** | 在线订阅添加/更新/删除、Base64 & YAML 智能解析、配置热加载与切换 |
| **4** | **规则路由 (Rules)** | Mihomo 规则匹配引擎（`DOMAIN-SUFFIX`, `GEOIP`, `IP-CIDR` 等）与策略组映射明细 |
| **5** | **连接管理 (Connections)** | 活动 TCP/UDP 连接监控、按流量降序排序 (`s`)、单连接断开 (`d`) 与全量切断 (`Shift+D`) |
| **6** | **流量监控 (Traffic)** | 高精上下行实时速率仪表盘与平滑历史趋势 Sparkline 视图 |
| **7** | **运行日志 (Logs)** | 实时 WebSocket 核心日志流，支持分级过滤 (`ALL`/`INFO`/`WARN`/`ERR`/`DBG`) 与一键清空 (`c`) |
| **8** | **系统设置 (Settings)** | 端口设置、DNS 模式、开机自启系统代理、自动更新周期与主题偏好调节 |
| **9** | **权限管理 (Privileges)** | Linux `CAP_NET_ADMIN` 与桌面 GSettings 代理权限监控与提权授权 |
| **10** | **关于 (About)** | 软件版本、架构元数据、依赖技术栈与开源致谢 |

---

## 🚀 安装与使用

### 1. 源码编译安装 (推荐)

确保本地已安装 Rust / Cargo 环境（Rust 1.80+）：

```bash
# 克隆仓库
git clone https://github.com/aimy1/Mimo.git
cd Mimo

# 编译并安装至系统 Cargo 路径 (~/.cargo/bin/mimo)
cargo install --path .
```

### 2. 启动 Mimo

安装完成后，在终端直接运行：

```bash
mimo
```

---

## ⌨️ 常用快捷键速查

| 快捷键 | 功能说明 |
| :--- | :--- |
| **`1` ~ `0`** | 快速直达 10 大主功能页面 |
| **`Tab`** | 循环切换页面 / 弹窗中切换表单输入焦点 |
| **`j` / `k`** 或 **`↓` / `↑`** | 上下滚动列表或表格行 |
| **`h` / `l`** 或 **`←` / `→`** | 在侧边栏 ↔ 工作区、代理分组 ↔ 节点列表间切换焦点 |
| **`g` / `G`** | 快速跳转至列表 顶部 (Top) / 底部 (Bottom) |
| **`Enter`** | 确认选中节点 / 激活订阅 / 保存设置 |
| **`Esc`** | 退出搜索模式、关闭弹窗、切回侧边栏焦点 |
| **`m`** | 循环切换运行模式 (`Rule` 规则 -> `Global` 全局 -> `Direct` 直连) |
| **`p`** | 开关桌面系统代理 (SysProxy) |
| **`x`** | 开关 **TUN 虚拟网卡模式** (透明接管全局网络) |
| **`r`** | 重启 Mihomo 核心服务 |
| **`t`** | (仪表盘) 刷新网站连通性 / (节点页) 当前组节点并发批量测速 |
| **`d`** | (节点页) 单节点测速 / (订阅页) 删除订阅 / (连接页) 断开选中连接 |
| **`o`** | (节点页) 切换节点按延迟升序排序 (ON/OFF) |
| **`s`** | (连接页) 切换连接按流量降序排序 (ON/OFF) |
| **`/`** | 调出实时搜索过滤输入框 |
| **`a` / `u`** | (订阅页) **`a`** 添加新订阅 / **`u`** 更新选中订阅 |
| **`c`** | (日志页) 清空当前日志记录 |
| **`Shift + D`** | (连接页) 强制断开所有活动网络连接 |
| **`?`** | 调出/关闭快捷键帮助菜单 |
| **`q` / `Ctrl+C`** | 退出程序 |

---

## 🛠️ CLI 命令行模式

Mimo 支持在非 TUI 环境下直接通过命令行进行脚本化控制：

```bash
# 查看核心运行状态
mimo status

# 管理 TUN 虚拟网卡模式及提权
mimo tun status                         # 查看 TUN 状态与提权状态
mimo tun on                             # 开启 TUN 模式
mimo tun off                            # 关闭 TUN 模式
mimo tun grant                          # 执行系统的 cap_net_admin 授权
mimo tun revoke                         # 撤销提权 (setcap -r)
mimo tun system|gvisor|lwip            # 切换 TUN 网络协议栈

# 管理桌面系统代理 (GSettings)
mimo sysproxy on
mimo sysproxy off

# 管理订阅
mimo profile list
mimo profile del --name "MySub"

# 管理核心进程
mimo core start
mimo core stop
mimo core restart
```

---

## 📄 开源许可

本项目采用 [MIT License](LICENSE) 开源协议。
