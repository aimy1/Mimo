# ⚡ Mimo (Minimal Mihomo Manager)

> **现代化、键盘驱动、Catppuccin 莫兰迪美学的 Mihomo / Clash Meta 终端 TUI 管理中心**
>
> 目标：在 Linux 终端中获得**完全对标 Clash Verge Rev GUI 核心功能**的极致 TUI 体验。

---

## 🌟 核心特性 (Features)

- **⚡ Linux 原生 & TUI 优先**：零 GUI 依赖，纯 Rust + Ratatui + Crossterm 打造，超低内存与 CPU 占用。
- **🎨 Catppuccin Mocha 莫兰迪调色盘**：电光紫与圆角边框 (`BorderType::Rounded`)，颜值媲美现代 GUI 应用。
- **📊 8 大全功能工作区**：
  1. **仪表盘 (Dashboard)**：实时硬件 CPU / RAM Gauges、连接负载、核心状态与 Btop 双极峰速报 Sparklines。
  2. **代理节点 (Proxies)**：全协议 Badge 标注 (`SS`/`Vmess`/`Vless`/`Trojan`/`Hysteria2`)、实心 Pill 延迟胶囊、实时 `s` 节点搜索。
  3. **订阅配置 (Profiles)**：支持在线订阅 URL 导入、自动解压、多配置无缝切换与 `PUT /configs?force=true` 热加载。
  4. **规则列表 (Rules)**：查看 Mihomo 规则匹配引擎 (`DOMAIN-SUFFIX`, `GEOIP`, `MATCH`) 与策略组映射。
  5. **链接管理 (Connections)**：TCP/UDP 活跃连接视图，支持下选 Connection Inspector 抓包详情与一键切断连接。
  6. **流量监控 (Traffic)**：高精上下行速率仪表与历史平滑 Sparkline 图表。
  7. **运行日志 (Logs)**：实时 WebSocket 日志流，支持 `[ALL]` `[INFO]` `[WARN]` `[ERR]` `[DBG]` 级别过滤与一键清空。
  8. **系统设置 (Settings)**：中/英文多语言切换、REST API 鉴权、端口设定与刷新率配置。
- **🛡️ Mihomo 内核 & 系统代理**：支持 `systemctl` 核心进程管理、Linux GNOME GSettings 系统代理开关、TUN 虚拟网卡与 `cap_net_admin` 权限检测。
- **🖱️ 全界面鼠标与 Vim 键盘双模支持**：支持 Vim (`j/k/h/l`)、方向键、`Tab` 切换与鼠标点击 / 滚轮直接操控。

---

## 🚀 快速安装 (Installation)

### 源码编译安装 (Cargo Install)

```bash
# 克隆仓库
git clone https://github.com/mimo-tui/mimo.git
cd mimo

# 编译并安装至 PATH (~/.cargo/bin)
cargo install --path .
```

安装完成后，在终端直接输入 `mimo` 或 `Mimo` 唤出 TUI：

```bash
mimo
```

---

## ⌨️ 快捷键指南 (Keybindings)

| 快捷键 | 功能说明 |
| :--- | :--- |
| **`1` - `8`** | 直达 仪表盘 / 节点 / 订阅 / 规则 / 连接 / 流量 / 日志 / 设置 |
| **`Tab` / `Shift+Tab`** | 顺序切换 Tab 页面 (在设置页面平滑切换表单输入焦点) |
| **`j` / `k`** 或 **`↓` / `↑`** | 上下移动列表焦点 / 滚动表格 |
| **`h` / `l`** 或 **`←` / `→`** | 在节点选择视图中切换 代理组 / 节点列表 分栏焦点 |
| **`Enter`** | 确认选中节点 / 激活订阅 / 保存设置 |
| **`t`** | 测试当前分组所有节点延迟 |
| **`m`** | 循环切换模式 (`Rule` 规则 -> `Global` 全局 -> `Direct` 直连) |
| **`p`** | 开关 Linux GNOME 系统代理 (System Proxy) |
| **`x`** | 开关 TUN 虚拟网卡模式 |
| **`r`** | 重启 Mihomo Core 内核 |
| **`a` / `u` / `d`** | 在订阅页面 Add 添加 / Update 更新 / Delete 删除配置 |
| **`c`** | 在日志页面清空 Log 缓冲区 |
| **`Shift + D`** | 在连接页面切断全部 TCP/UDP 连接 |
| **`s`** 或 **`/`** | 调出/关闭节点实时 Searching 搜索框 |
| **`?`** | 调出/关闭全局帮助对话框 |
| **`q`** 或 **`Ctrl + C`** | 退出 Mimo |

---

## 🛠️ CLI 命令行模式 (Command Line Interface)

Mimo 支持无需打开 TUI 的终端单条指令操控：

```bash
# 查看 Mihomo 核心运行状态
mimo status

# 查看及切换代理节点
mimo proxy list
mimo proxy use --group GLOBAL --node "香港 01"

# 节点延迟测试
mimo latency --group GLOBAL

# 查看及添加订阅
mimo profile list
mimo profile add --name "MySub" --url "https://example.com/sub.yaml"
mimo profile use "MySub"

# 系统代理与 TUN 管理
mimo sysproxy --enable
mimo tun --status
mimo core --restart
```

---

## 📄 开源协议 (License)

MIT License © 2026 Mimo Developers.
