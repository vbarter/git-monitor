use crate::app::{ActivePanel, App};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

/// Handle keyboard events
pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),

        // Navigation
        KeyCode::Char('j') | KeyCode::Down => {
            match app.active_panel {
                ActivePanel::FileList => app.select_next(),
                ActivePanel::DiffView => app.scroll_diff_down(),
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            match app.active_panel {
                ActivePanel::FileList => app.select_previous(),
                ActivePanel::DiffView => app.scroll_diff_up(),
            }
        }

        // Panel switching
        KeyCode::Tab => app.toggle_panel(),

        // Stage/unstage
        KeyCode::Enter => {
            if let Err(e) = app.toggle_stage() {
                // Log error but don't crash
                eprintln!("Failed to toggle stage: {}", e);
            }
        }

        // Refresh
        KeyCode::Char('r') => {
            if let Err(e) = app.refresh_status() {
                eprintln!("Failed to refresh: {}", e);
            }
        }

        // Page navigation for diff view
        KeyCode::PageDown => {
            for _ in 0..10 {
                app.scroll_diff_down();
            }
        }
        KeyCode::PageUp => {
            for _ in 0..10 {
                app.scroll_diff_up();
            }
        }

        // Home/End for file list
        KeyCode::Home => {
            app.selected_index = 0;
            app.diff_scroll = 0;
        }
        KeyCode::End => {
            if !app.files.is_empty() {
                app.selected_index = app.files.len() - 1;
                app.diff_scroll = 0;
            }
        }

        _ => {}
    }
}

/// Handle mouse events
pub fn handle_mouse_event(
    app: &mut App,
    mouse: MouseEvent,
    file_list_area: Option<(u16, u16, u16, u16)>,
    diff_view_area: Option<(u16, u16, u16, u16)>,
) {
    let in_file_list = is_in_area(mouse.column, mouse.row, file_list_area);
    let in_diff_view = is_in_area(mouse.column, mouse.row, diff_view_area);

    match mouse.kind {
        MouseEventKind::ScrollDown => {
            if in_file_list {
                app.select_next();
            } else if in_diff_view {
                // Scroll diff view (3 lines at a time for smoother scrolling)
                for _ in 0..3 {
                    app.scroll_diff_down();
                }
            }
        }
        MouseEventKind::ScrollUp => {
            if in_file_list {
                app.select_previous();
            } else if in_diff_view {
                for _ in 0..3 {
                    app.scroll_diff_up();
                }
            }
        }
        MouseEventKind::Down(_) => {
            if in_file_list {
                if let Some((_, y, _, _)) = file_list_area {
                    // Calculate which file was clicked (accounting for border)
                    let relative_row = mouse.row.saturating_sub(y + 1); // +1 for border
                    let index = relative_row as usize;
                    if index < app.files.len() {
                        app.selected_index = index;
                        app.diff_scroll = 0;
                    }
                }
                app.active_panel = ActivePanel::FileList;
            } else if in_diff_view {
                app.active_panel = ActivePanel::DiffView;
            }
        }
        _ => {}
    }
}

/// Check if coordinates are within the given area
fn is_in_area(x: u16, y: u16, area: Option<(u16, u16, u16, u16)>) -> bool {
    if let Some((ax, ay, width, height)) = area {
        x >= ax && x < ax + width && y >= ay && y < ay + height
    } else {
        false
    }
}
