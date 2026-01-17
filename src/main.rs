mod app;
mod banner;
mod event;
mod git;
mod terminal;
mod ui;

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use std::path::PathBuf;
use terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Handle --help and --version
    if args.iter().any(|a| a == "--help" || a == "-h") {
        banner::print_banner();
        banner::print_version();
        print_help();
        return Ok(());
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        banner::print_banner();
        banner::print_version();
        return Ok(());
    }

    // Get repository path from args or use current directory
    let repo_path = args
        .get(1)
        .filter(|a| !a.starts_with('-'))
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    // Initialize terminal
    let mut terminal = Terminal::new()?;

    // Initialize application
    let mut app = App::new(repo_path)?;

    // Create event handler
    let mut events = EventHandler::new(200);

    // Start file watcher
    app.start_watcher(events.sender())?;

    // Main loop
    while app.is_running() {
        // Render UI
        terminal.draw(|frame| {
            ui::render(frame, &mut app);
        })?;

        // Handle events
        match events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => event::handle_key_event(&mut app, key_event),
            Event::Mouse(mouse_event) => {
                let file_area = app.file_list_area;
                let diff_area = app.diff_view_area;
                event::handle_mouse_event(&mut app, mouse_event, file_area, diff_area)
            }
            Event::GitChange(changed_paths) => app.refresh_status_with_paths(changed_paths)?,
            Event::Resize(_, _) => {}
        }
    }

    // Cleanup
    terminal.restore()?;

    Ok(())
}

fn print_help() {
    const GREEN: &str = "\x1b[38;2;166;227;161m";
    const YELLOW: &str = "\x1b[38;2;249;226;175m";
    const TEXT: &str = "\x1b[38;2;205;214;244m";
    const DIM: &str = "\x1b[38;2;108;112;134m";
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";

    eprintln!("{BOLD}USAGE:{RESET}");
    eprintln!("    {GREEN}git-monitor{RESET} {DIM}[PATH]{RESET}");
    eprintln!();
    eprintln!("{BOLD}ARGS:{RESET}");
    eprintln!("    {YELLOW}<PATH>{RESET}    Path to Git repository {DIM}(default: current directory){RESET}");
    eprintln!();
    eprintln!("{BOLD}OPTIONS:{RESET}");
    eprintln!("    {GREEN}-h{RESET}, {GREEN}--help{RESET}       Print help information");
    eprintln!("    {GREEN}-V{RESET}, {GREEN}--version{RESET}    Print version information");
    eprintln!();
    eprintln!("{BOLD}KEYBINDINGS:{RESET}");
    eprintln!("    {YELLOW}j/k{RESET} or {YELLOW}↑/↓{RESET}      Navigate files");
    eprintln!("    {YELLOW}Tab{RESET}             Switch panels");
    eprintln!("    {YELLOW}Enter{RESET}           Stage/Unstage file");
    eprintln!("    {YELLOW}r{RESET}               Refresh status");
    eprintln!("    {YELLOW}q{RESET} or {YELLOW}Esc{RESET}        Quit");
    eprintln!();
    eprintln!("{DIM}For more information, visit: https://github.com/vbarter/git-monitor{RESET}");
}
