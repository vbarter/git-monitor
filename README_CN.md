<p align="center">
  <img src="assets/logo.svg" alt="git-monitor" width="600">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" alt="Rust Version">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg" alt="Platform">
  <img src="https://img.shields.io/github/stars/vbarter/git-monitor?style=social" alt="GitHub Stars">
</p>

<p align="center">
  <strong>一个美观的、实时 Git 文件变更监控终端 UI 工具，带有动画效果。</strong>
</p>

<p align="center">
  <a href="README.md">English</a> | <strong>中文</strong>
</p>

---

## 功能特性

- **实时监控** - 自动检测 Git 仓库中的文件变更，200ms 防抖处理
- **精美界面** - 基于 [Ratatui](https://github.com/ratatui-org/ratatui) 构建的现代终端界面，采用 Catppuccin 配色主题
- **文件图标** - 彩色文件类型图标，由 [devicons](https://github.com/alexpasmantier/rust-devicons) 提供（需要 Nerd Font 字体）
- **动画反馈** - 文件变更时的脉冲动画效果，变更的文件自动排序到列表顶部
- **Diff 预览** - 并排显示 Diff 内容，语法高亮显示增删行
- **鼠标支持** - 支持在文件列表和 Diff 视图中滚动和点击
- **键盘导航** - Vim 风格快捷键，高效导航
- **暂存管理** - 一键暂存/取消暂存文件

## 界面预览

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Git Monitor - [branch: main] ● watching                          [v0.1.0] │
├─────────────────────────────────────────────────────────────────────────────┤
│ ┌─ Changed Files (5) ──────────────┐ ┌─ Diff: src/main.rs ────────────────┐│
│ │  main.rs src/               M  │ │ @@ -10,7 +10,8 @@                  ││
│ │  lib.rs src/                M  │ │  fn main() {                       ││
│ │  utils.py scripts/          A  │ │ -    old_code();                   ││
│ │  config.json                ?  │ │ +    new_code();                   ││
│ │  README.md                  M  │ │ +    additional_line();            ││
│ └──────────────────────────────────┘ │  }                                ││
│                                       └────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────────────────────┤
│  Staged: 2 | Modified: 2 | Untracked: 1 | Last update: 0.5s ago            │
└─────────────────────────────────────────────────────────────────────────────┘
```

## 安装

### 前置要求

- **Rust 1.70+** - 通过 [rustup](https://rustup.rs/) 安装
- **Nerd Font 字体** - 用于显示文件类型图标（[下载](https://www.nerdfonts.com/)）

### 安装 Nerd Font（推荐）

```bash
# macOS（使用 Homebrew）
brew install --cask font-jetbrains-mono-nerd-font

# 或者从 https://www.nerdfonts.com/font-downloads 手动下载
```

安装后，请配置终端使用 Nerd Font 字体。

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/vbarter/git-monitor.git
cd git-monitor

# 构建发布版本
cargo build --release

# 二进制文件位于 ./target/release/git-monitor
```

### 通过 Cargo 安装

```bash
cargo install --path .
```

## 使用方法

```bash
# 监控当前目录
git-monitor

# 监控指定仓库
git-monitor /path/to/your/repo
```

## 键盘快捷键

| 按键 | 功能 |
|-----|------|
| `j` / `↓` | 向下移动 |
| `k` / `↑` | 向上移动 |
| `Tab` | 切换面板 |
| `Enter` | 暂存 / 取消暂存文件 |
| `r` | 刷新状态 |
| `PageDown` | 向下滚动 Diff（10 行）|
| `PageUp` | 向上滚动 Diff（10 行）|
| `Home` | 跳到第一个文件 |
| `End` | 跳到最后一个文件 |
| `q` / `Esc` | 退出 |
| `Ctrl+C` | 强制退出 |

## 鼠标操作

| 操作 | 效果 |
|-----|------|
| 在文件列表滚动 | 上下选择文件 |
| 在 Diff 视图滚动 | 滚动 Diff 内容 |
| 点击文件 | 选中该文件 |
| 点击 Diff 区域 | 激活 Diff 面板 |

## 状态标识

| 符号 | 颜色 | 含义 |
|-----|------|------|
| `M` | 黄色 | 已修改（未暂存）|
| `M` | 绿色 | 已修改（已暂存）|
| `A` | 蓝色 | 新增 |
| `D` | 红色 | 已删除 |
| `R` | 紫色 | 重命名 |
| `?` | 灰色 | 未跟踪 |
| `!` | 粉色 | 冲突 |

## 动画效果

当文件被修改时：
- 文件图标和名称会闪烁暖色光芒
- 背景短暂高亮
- 文件自动移动到列表顶部
- 动画持续约 800ms

## 配置

Git Monitor 使用合理的默认配置，无需额外配置。它会自动：
- 检测 Git 仓库根目录
- 递归监控文件变更
- 忽略 `.git/objects`、`.git/logs`、`target/` 和临时文件

## 项目结构

```
git-monitor/
├── src/
│   ├── main.rs              # 入口点
│   ├── app.rs               # 应用状态
│   ├── terminal.rs          # 终端设置/清理
│   ├── event/
│   │   ├── mod.rs           # 事件系统
│   │   └── handler.rs       # 键盘/鼠标处理
│   ├── git/
│   │   ├── mod.rs
│   │   ├── repository.rs    # Git 操作 (git2)
│   │   └── watcher.rs       # 文件监控 (notify-rs)
│   └── ui/
│       ├── mod.rs
│       ├── layout.rs        # UI 布局
│       ├── theme.rs         # Catppuccin 配色
│       ├── icons.rs         # 文件类型图标
│       ├── components/
│       │   ├── file_list.rs # 文件列表组件
│       │   ├── diff_view.rs # Diff 预览组件
│       │   └── status_bar.rs# 头部和状态栏
│       └── effects/
│           └── manager.rs   # 动画效果
```

## 依赖库

| 库 | 用途 |
|---|------|
| [ratatui](https://github.com/ratatui-org/ratatui) | 终端 UI 框架 |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 跨平台终端操作 |
| [git2](https://github.com/rust-lang/git2-rs) | Git 操作（libgit2 绑定）|
| [notify](https://github.com/notify-rs/notify) | 文件系统监控 |
| [devicons](https://github.com/alexpasmantier/rust-devicons) | 文件类型图标 |
| [tokio](https://github.com/tokio-rs/tokio) | 异步运行时 |

## 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m '添加某个很棒的功能'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- [Ratatui](https://github.com/ratatui-org/ratatui) - 优秀的 TUI 框架
- [Catppuccin](https://github.com/catppuccin/catppuccin) - 漂亮的配色方案
- [Nerd Fonts](https://www.nerdfonts.com/) - 文件类型图标
- [rust-devicons](https://github.com/alexpasmantier/rust-devicons) - 图标映射

---

<p align="center">
  用 ❤️ 和 Rust 构建
</p>
