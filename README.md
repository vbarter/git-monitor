# Git Monitor

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" alt="Rust Version">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg" alt="Platform">
</p>

<p align="center">
  A beautiful, real-time Git file change monitoring terminal UI tool with animations.
</p>

<p align="center">
  <strong>English</strong> | <a href="README_CN.md">中文</a>
</p>

---

## Features

- **Real-time Monitoring** - Automatically detects file changes in your Git repository with 200ms debouncing
- **Beautiful TUI** - Modern terminal interface built with [Ratatui](https://github.com/ratatui-org/ratatui) and Catppuccin color theme
- **File Type Icons** - Colorful file icons powered by [devicons](https://github.com/alexpasmantier/rust-devicons) (requires Nerd Font)
- **Animated Feedback** - Pulse animation effect when files change, with changed files automatically sorted to the top
- **Diff Preview** - Side-by-side diff view with syntax highlighting for additions and deletions
- **Mouse Support** - Scroll and click support for both file list and diff view
- **Keyboard Navigation** - Vim-style keybindings for efficient navigation
- **Stage/Unstage** - Quick staging and unstaging of files with a single key

## Screenshots

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

## Installation

### Prerequisites

- **Rust 1.70+** - Install via [rustup](https://rustup.rs/)
- **Nerd Font** - Required for file type icons ([Download](https://www.nerdfonts.com/))

### Install Nerd Font (Recommended)

```bash
# macOS (using Homebrew)
brew install --cask font-jetbrains-mono-nerd-font

# Or download manually from https://www.nerdfonts.com/font-downloads
```

After installation, configure your terminal to use the Nerd Font.

### Build from Source

```bash
# Clone the repository
git clone https://github.com/vbarter/git-monitor.git
cd git-monitor

# Build release version
cargo build --release

# The binary will be at ./target/release/git-monitor
```

### Install via Cargo

```bash
cargo install --path .
```

## Usage

```bash
# Monitor current directory
git-monitor

# Monitor a specific repository
git-monitor /path/to/your/repo
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `Tab` | Switch between panels |
| `Enter` | Stage / Unstage file |
| `r` | Refresh status |
| `PageDown` | Scroll diff down (10 lines) |
| `PageUp` | Scroll diff up (10 lines) |
| `Home` | Go to first file |
| `End` | Go to last file |
| `q` / `Esc` | Quit |
| `Ctrl+C` | Force quit |

## Mouse Support

| Action | Effect |
|--------|--------|
| Scroll in file list | Navigate files up/down |
| Scroll in diff view | Scroll diff content |
| Click on file | Select that file |
| Click on diff area | Activate diff panel |

## Status Indicators

| Symbol | Color | Meaning |
|--------|-------|---------|
| `M` | Yellow | Modified (unstaged) |
| `M` | Green | Modified (staged) |
| `A` | Blue | Added |
| `D` | Red | Deleted |
| `R` | Purple | Renamed |
| `?` | Gray | Untracked |
| `!` | Pink | Conflicted |

## Animation Effects

When a file is modified:
- The file icon and name pulse with a warm glow
- The background briefly highlights
- The file automatically moves to the top of the list
- Animation lasts approximately 800ms

## Configuration

Git Monitor uses sensible defaults and requires no configuration. It automatically:
- Detects the Git repository root
- Watches for file changes recursively
- Ignores `.git/objects`, `.git/logs`, `target/`, and temporary files

## Architecture

```
git-monitor/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # Application state
│   ├── terminal.rs          # Terminal setup/cleanup
│   ├── event/
│   │   ├── mod.rs           # Event system
│   │   └── handler.rs       # Keyboard/mouse handlers
│   ├── git/
│   │   ├── mod.rs
│   │   ├── repository.rs    # Git operations (git2)
│   │   └── watcher.rs       # File system watcher (notify-rs)
│   └── ui/
│       ├── mod.rs
│       ├── layout.rs        # UI layout
│       ├── theme.rs         # Catppuccin colors
│       ├── icons.rs         # File type icons
│       ├── components/
│       │   ├── file_list.rs # File list component
│       │   ├── diff_view.rs # Diff preview component
│       │   └── status_bar.rs# Header & status bar
│       └── effects/
│           └── manager.rs   # Animation effects
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [ratatui](https://github.com/ratatui-org/ratatui) | Terminal UI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Cross-platform terminal manipulation |
| [git2](https://github.com/rust-lang/git2-rs) | Git operations (libgit2 bindings) |
| [notify](https://github.com/notify-rs/notify) | File system watching |
| [devicons](https://github.com/alexpasmantier/rust-devicons) | File type icons |
| [tokio](https://github.com/tokio-rs/tokio) | Async runtime |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Ratatui](https://github.com/ratatui-org/ratatui) - For the amazing TUI framework
- [Catppuccin](https://github.com/catppuccin/catppuccin) - For the beautiful color palette
- [Nerd Fonts](https://www.nerdfonts.com/) - For the file type icons
- [rust-devicons](https://github.com/alexpasmantier/rust-devicons) - For the icon mapping

---

<p align="center">
  Made with ❤️ in Rust
</p>
