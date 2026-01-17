use devicons::FileIcon as DevIcon;
use ratatui::style::Color;

/// File icon with color based on file extension
pub struct FileIcon {
    pub icon: String,
    pub color: Color,
}

impl FileIcon {
    pub fn from_filename(filename: &str) -> Self {
        // Use devicons library to get the icon and color
        let dev_icon = DevIcon::from(filename);

        // Convert the hex color to ratatui Color
        let color = parse_hex_color(&dev_icon.color);

        FileIcon {
            icon: dev_icon.icon.to_string(),
            color,
        }
    }
}

/// Parse a hex color string (e.g., "#e44d26" or "e44d26") to ratatui Color
fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Color::Rgb(158, 158, 158); // Default gray
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(158);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(158);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(158);

    Color::Rgb(r, g, b)
}
