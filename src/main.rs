mod app;
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

    // Get repository path from args or use current directory
    let repo_path = std::env::args()
        .nth(1)
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
