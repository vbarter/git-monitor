use crate::app::App;
use crate::ui::components::{render_diff_view, render_file_list, render_header, render_status_bar};
use ratatui::prelude::*;

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Create main layout: header, content, status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Content
            Constraint::Length(3),  // Status bar
        ])
        .split(area);

    // Render header
    render_header(frame, app, chunks[0]);

    // Create content layout: file list (left) and diff preview (right)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),  // File list (wider for full filenames)
            Constraint::Percentage(50),  // Diff preview
        ])
        .margin(1)
        .split(chunks[1]);

    // Store areas for mouse events
    let file_list_area = content_chunks[0];
    app.file_list_area = Some((
        file_list_area.x,
        file_list_area.y,
        file_list_area.width,
        file_list_area.height,
    ));

    let diff_area = content_chunks[1];
    app.diff_view_area = Some((
        diff_area.x,
        diff_area.y,
        diff_area.width,
        diff_area.height,
    ));

    // Render file list
    render_file_list(frame, app, content_chunks[0]);

    // Render diff preview
    render_diff_view(frame, app, content_chunks[1]);

    // Render status bar
    render_status_bar(frame, app, chunks[2]);
}
