use crate::app::{ActivePanel, App};
use crate::ui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_diff_view(frame: &mut Frame, app: &App, area: Rect) {
    let is_active = app.active_panel == ActivePanel::DiffView;

    let title = match app.selected_file() {
        Some(file) => format!(" Diff: {} ", file.path),
        None => " Diff Preview ".to_string(),
    };

    let border_color = if is_active {
        Theme::ACCENT
    } else {
        Theme::BORDER
    };

    let block = Block::default()
        .title(title)
        .title_style(Style::default().fg(Theme::TEXT).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let diff_content = app.get_diff().unwrap_or_default();

    if diff_content.is_empty() {
        let empty_message = if app.files.is_empty() {
            "No changes detected"
        } else {
            "Select a file to view diff"
        };

        let paragraph = Paragraph::new(empty_message)
            .block(block)
            .style(Style::default().fg(Theme::SUBTEXT))
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
        return;
    }

    // Parse and colorize diff content
    let styled_lines: Vec<Line> = diff_content
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let (style, prefix) = if line.starts_with("@@") {
                (
                    Style::default().fg(Theme::DIFF_HUNK).bold(),
                    "",
                )
            } else if line.starts_with('+') && !line.starts_with("+++") {
                (
                    Style::default().fg(Theme::DIFF_ADD),
                    "",
                )
            } else if line.starts_with('-') && !line.starts_with("---") {
                (
                    Style::default().fg(Theme::DIFF_DEL),
                    "",
                )
            } else {
                (Style::default().fg(Theme::SUBTEXT), "")
            };

            Line::from(Span::styled(format!("{}{}", prefix, line), style))
        })
        .collect();

    let total_lines = styled_lines.len();
    let visible_lines = (area.height.saturating_sub(2)) as usize;

    // Clamp scroll offset
    let max_scroll = total_lines.saturating_sub(visible_lines);
    let scroll_offset = app.diff_scroll.min(max_scroll);

    let paragraph = Paragraph::new(styled_lines)
        .block(block)
        .scroll((scroll_offset as u16, 0));

    frame.render_widget(paragraph, area);

    // Render scroll indicator if needed
    if total_lines > visible_lines {
        let scroll_percent = if max_scroll > 0 {
            (scroll_offset * 100) / max_scroll
        } else {
            0
        };
        let indicator = format!(" {}% ", scroll_percent);
        let indicator_area = Rect {
            x: area.x + area.width - indicator.len() as u16 - 2,
            y: area.y,
            width: indicator.len() as u16,
            height: 1,
        };
        frame.render_widget(
            Paragraph::new(indicator).style(Style::default().fg(Theme::SUBTEXT).dim()),
            indicator_area,
        );
    }
}
