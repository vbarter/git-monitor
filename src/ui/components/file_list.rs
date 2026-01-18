use crate::app::{ActivePanel, App};
use crate::git::FileStatus;
use crate::ui::icons::FileIcon;
use crate::ui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

pub fn render_file_list(frame: &mut Frame, app: &App, area: Rect) {
    let is_active = app.active_panel == ActivePanel::FileList;

    let title = format!(" Changed Files ({}) ", app.files.len());
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

    // Available width inside the block (subtract borders)
    let inner_width = area.width.saturating_sub(2) as usize;

    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let is_selected = i == app.selected_index;
            let is_recently_changed = app.is_recently_changed(&file.path);

            // Status indicator and color
            let (status_char, status_color) = get_status_display(&file.status, file.staged);

            // Split path into filename and directory
            let (filename, directory) = split_path(&file.path);

            // Get file type icon
            let file_icon = FileIcon::from_filename(&filename);

            // Calculate number width for alignment (e.g., " 1." vs "10.")
            let total_files = app.files.len();
            let num_width = if total_files >= 100 {
                4 // "100."
            } else if total_files >= 10 {
                3 // "10."
            } else {
                2 // "1."
            };

            // Calculate available space for filename and directory
            // Format: "N. icon  filename directory... S"
            // Fixed parts: num(2-4) + space(1) + icon(1-2) + space(1) + space(1) + status(1)
            let fixed_width = num_width + 1 + 5;
            let available_for_path = inner_width.saturating_sub(fixed_width);

            // Build the display strings
            let (display_filename, display_dir) =
                format_filename_and_dir(&filename, &directory, available_for_path);

            // Calculate padding to right-align status
            // num(N) + space(1) + icon(2) + space(1) + filename + dir_with_space + padding + status(1) = inner_width
            let dir_display_len = if display_dir.is_empty() {
                0
            } else {
                display_dir.len() + 1 // +1 for space before dir
            };
            let content_len = num_width + 1 + 3 + display_filename.len() + dir_display_len;
            let padding = inner_width.saturating_sub(content_len + 1); // +1 for status char

            // Calculate animation state
            let (icon_color, text_color, bg_style) = if is_recently_changed {
                let progress = app.get_change_progress(&file.path).unwrap_or(0.0);
                let brightness = calculate_pulse_brightness(progress);

                // Interpolate icon color towards bright yellow/white during pulse
                let animated_icon_color = interpolate_color(
                    file_icon.color,
                    Theme::FLASH_BRIGHT,
                    brightness * 0.7,
                );

                // Make text brighter during pulse
                let animated_text_color = interpolate_color(
                    Theme::TEXT,
                    Theme::FLASH_BRIGHT,
                    brightness * 0.5,
                );

                let bg = Style::default()
                    .bg(interpolate_color(Theme::SURFACE, Theme::FLASH_BG, brightness));

                (animated_icon_color, animated_text_color, bg)
            } else if is_selected {
                (file_icon.color, Theme::TEXT, Style::default().bg(Theme::OVERLAY))
            } else {
                (file_icon.color, Theme::TEXT, Style::default())
            };

            // Build styled spans
            let num_display = format!("{:>width$} ", i + 1, width = num_width);
            let mut spans = vec![
                // Line number
                Span::styled(num_display, Style::default().fg(Theme::SUBTEXT).dim()),
                // File type icon (with animation)
                Span::styled(
                    format!("{} ", file_icon.icon),
                    Style::default().fg(icon_color),
                ),
                // Filename (with animation)
                Span::styled(display_filename.clone(), Style::default().fg(text_color)),
            ];

            // Directory (dim)
            if !display_dir.is_empty() {
                spans.push(Span::styled(
                    format!(" {}", display_dir),
                    Style::default().fg(Theme::SUBTEXT).dim(),
                ));
            }

            // Padding and status at the end
            spans.push(Span::raw(" ".repeat(padding.max(1))));
            spans.push(Span::styled(
                status_char.to_string(),
                Style::default().fg(status_color).bold(),
            ));

            let line = Line::from(spans);

            ListItem::new(line).style(bg_style)
        })
        .collect();

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Theme::OVERLAY)
            .add_modifier(Modifier::BOLD),
    );

    let mut state = ListState::default();
    state.select(Some(app.selected_index));

    frame.render_stateful_widget(list, area, &mut state);
}

/// Split a path into (filename, directory)
fn split_path(path: &str) -> (String, String) {
    if let Some(pos) = path.rfind('/') {
        let filename = path[pos + 1..].to_string();
        let directory = path[..pos].to_string();
        (filename, directory)
    } else {
        (path.to_string(), String::new())
    }
}

/// Format filename and directory to fit within available width
/// Returns (display_filename, display_directory)
fn format_filename_and_dir(filename: &str, directory: &str, available: usize) -> (String, String) {
    let filename_len = filename.len();

    if filename_len >= available {
        // Filename alone is too long, truncate it
        if available > 3 {
            return (format!("{}...", &filename[..available - 3]), String::new());
        } else {
            return (
                filename[..available.min(filename_len)].to_string(),
                String::new(),
            );
        }
    }

    if directory.is_empty() {
        return (filename.to_string(), String::new());
    }

    // Space available for directory (with 1 space separator)
    let dir_available = available.saturating_sub(filename_len + 1);

    if dir_available == 0 {
        return (filename.to_string(), String::new());
    }

    let dir_len = directory.len();

    if dir_len <= dir_available {
        // Full directory fits
        (filename.to_string(), directory.to_string())
    } else if dir_available > 4 {
        // Truncate directory with ellipsis at the start
        let visible = dir_available - 3;
        let start = dir_len.saturating_sub(visible);
        (
            filename.to_string(),
            format!("...{}", &directory[start..]),
        )
    } else {
        // Not enough space for directory
        (filename.to_string(), String::new())
    }
}

fn get_status_display(status: &FileStatus, staged: bool) -> (&'static str, Color) {
    let color = match status {
        FileStatus::Modified => {
            if staged {
                Theme::STAGED
            } else {
                Theme::MODIFIED
            }
        }
        FileStatus::Added => Theme::ADDED,
        FileStatus::Deleted => Theme::DELETED,
        FileStatus::Renamed => Theme::RENAMED,
        FileStatus::Untracked => Theme::UNTRACKED,
        FileStatus::Conflicted => Theme::CONFLICTED,
    };

    (status.symbol(), color)
}

/// Calculate pulse brightness for animation effect
fn calculate_pulse_brightness(progress: f64) -> f64 {
    let pulses = 2.5;
    let pulse_progress = progress * pulses * std::f64::consts::PI * 2.0;
    ((pulse_progress.sin() + 1.0) / 2.0) * (1.0 - progress)
}

/// Interpolate between two colors
fn interpolate_color(from: Color, to: Color, factor: f64) -> Color {
    match (from, to) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * factor) as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * factor) as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * factor) as u8;
            Color::Rgb(r, g, b)
        }
        _ => to,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== split_path tests ====================

    #[test]
    fn test_split_path_with_directory() {
        let (filename, directory) = split_path("src/ui/components/file_list.rs");
        assert_eq!(filename, "file_list.rs");
        assert_eq!(directory, "src/ui/components");
    }

    #[test]
    fn test_split_path_single_directory() {
        let (filename, directory) = split_path("src/main.rs");
        assert_eq!(filename, "main.rs");
        assert_eq!(directory, "src");
    }

    #[test]
    fn test_split_path_no_directory() {
        let (filename, directory) = split_path("README.md");
        assert_eq!(filename, "README.md");
        assert_eq!(directory, "");
    }

    #[test]
    fn test_split_path_deep_nested() {
        let (filename, directory) = split_path("a/b/c/d/e/f.txt");
        assert_eq!(filename, "f.txt");
        assert_eq!(directory, "a/b/c/d/e");
    }

    // ==================== format_filename_and_dir tests ====================

    #[test]
    fn test_format_filename_and_dir_enough_space() {
        let (filename, dir) = format_filename_and_dir("main.rs", "src", 20);
        assert_eq!(filename, "main.rs");
        assert_eq!(dir, "src");
    }

    #[test]
    fn test_format_filename_and_dir_no_directory() {
        let (filename, dir) = format_filename_and_dir("README.md", "", 20);
        assert_eq!(filename, "README.md");
        assert_eq!(dir, "");
    }

    #[test]
    fn test_format_filename_and_dir_truncate_directory() {
        // filename(7) + space(1) + dir needs truncation
        let (filename, dir) = format_filename_and_dir("main.rs", "src/ui/components", 15);
        assert_eq!(filename, "main.rs");
        // Available for dir: 15 - 7 - 1 = 7, so truncate with "..."
        assert!(dir.starts_with("..."));
    }

    #[test]
    fn test_format_filename_and_dir_no_space_for_directory() {
        // filename(10) fits in available(12), but no space for directory
        let (filename, dir) = format_filename_and_dir("config.toml", "src", 12);
        assert_eq!(filename, "config.toml");
        assert_eq!(dir, "");
    }

    #[test]
    fn test_format_filename_and_dir_truncate_filename() {
        let (filename, dir) = format_filename_and_dir("extremely_long_filename.rs", "src", 10);
        // Filename alone is too long, truncate it
        assert!(filename.ends_with("..."));
        assert_eq!(dir, "");
    }

    #[test]
    fn test_format_filename_and_dir_exact_fit() {
        // filename(4) + space(1) + dir(3) = 8
        let (filename, dir) = format_filename_and_dir("test", "abc", 8);
        assert_eq!(filename, "test");
        assert_eq!(dir, "abc");
    }

    // ==================== calculate_pulse_brightness tests ====================

    #[test]
    fn test_calculate_pulse_brightness_at_start() {
        let brightness = calculate_pulse_brightness(0.0);
        // At progress=0, sin(0)=0, so brightness = (0+1)/2 * 1 = 0.5
        assert!((brightness - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_calculate_pulse_brightness_at_end() {
        let brightness = calculate_pulse_brightness(1.0);
        // At progress=1, factor (1-progress) = 0, so brightness = 0
        assert!((brightness - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_pulse_brightness_in_range() {
        // Brightness should always be between 0 and 1
        for i in 0..=100 {
            let progress = i as f64 / 100.0;
            let brightness = calculate_pulse_brightness(progress);
            assert!(brightness >= 0.0 && brightness <= 1.0,
                "Brightness {} out of range at progress {}", brightness, progress);
        }
    }

    // ==================== interpolate_color tests ====================

    #[test]
    fn test_interpolate_color_factor_zero() {
        let from = Color::Rgb(0, 0, 0);
        let to = Color::Rgb(255, 255, 255);
        let result = interpolate_color(from, to, 0.0);
        assert_eq!(result, Color::Rgb(0, 0, 0));
    }

    #[test]
    fn test_interpolate_color_factor_one() {
        let from = Color::Rgb(0, 0, 0);
        let to = Color::Rgb(255, 255, 255);
        let result = interpolate_color(from, to, 1.0);
        assert_eq!(result, Color::Rgb(255, 255, 255));
    }

    #[test]
    fn test_interpolate_color_factor_half() {
        let from = Color::Rgb(0, 0, 0);
        let to = Color::Rgb(200, 100, 50);
        let result = interpolate_color(from, to, 0.5);
        assert_eq!(result, Color::Rgb(100, 50, 25));
    }

    #[test]
    fn test_interpolate_color_non_rgb_returns_to() {
        let from = Color::Red;
        let to = Color::Blue;
        let result = interpolate_color(from, to, 0.5);
        assert_eq!(result, Color::Blue);
    }

    #[test]
    fn test_interpolate_color_mixed_types() {
        let from = Color::Red;
        let to = Color::Rgb(100, 100, 100);
        let result = interpolate_color(from, to, 0.5);
        // Non-RGB from returns to
        assert_eq!(result, Color::Rgb(100, 100, 100));
    }
}
