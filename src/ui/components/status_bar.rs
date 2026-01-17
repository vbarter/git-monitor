use crate::app::App;
use crate::ui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let watching_indicator = if app.files.is_empty() {
        "○ idle"
    } else {
        "● watching"
    };

    let title = format!(
        "  Git Monitor - [branch: {}] {}",
        app.branch_name, watching_indicator
    );

    let version = " [v0.1.0] ";

    let title_len = title.len();
    let padding_len = area.width
        .saturating_sub(title_len as u16 + version.len() as u16) as usize;

    let header_line = Line::from(vec![
        Span::styled(title, Style::default().fg(Theme::TEXT).bold()),
        Span::styled(" ".repeat(padding_len), Style::default()),
        Span::styled(version, Style::default().fg(Theme::SUBTEXT)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Theme::BORDER));

    let paragraph = Paragraph::new(header_line).block(block);

    frame.render_widget(paragraph, area);
}

pub fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let staged = app.staged_count();
    let modified = app.modified_count();
    let untracked = app.untracked_count();
    let last_update = app.seconds_since_update();

    let status_text = format!(
        " Staged: {} | Modified: {} | Untracked: {} | Last: {:.1}s ago ",
        staged, modified, untracked, last_update
    );

    let help_text = " q: quit | j/k: navigate | Tab: switch panel | Enter: stage/unstage | r: refresh ";

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Theme::BORDER));

    let available_width = area.width.saturating_sub(2) as usize;
    let help_display = if available_width > status_text.len() + help_text.len() {
        help_text.to_string()
    } else {
        " q: quit ".to_string()
    };

    let padding = available_width
        .saturating_sub(status_text.len())
        .saturating_sub(help_display.len());

    let line = Line::from(vec![
        Span::styled(status_text, Style::default().fg(Theme::TEXT)),
        Span::raw(" ".repeat(padding)),
        Span::styled(help_display, Style::default().fg(Theme::SUBTEXT).dim()),
    ]);

    let paragraph = Paragraph::new(line).block(block);

    frame.render_widget(paragraph, area);
}
