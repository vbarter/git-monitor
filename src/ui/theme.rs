#![allow(dead_code)]

use ratatui::style::Color;

/// Catppuccin-inspired color theme
pub struct Theme;

impl Theme {
    // Status colors
    pub const STAGED: Color = Color::Rgb(166, 227, 161);     // Green #A6E3A1
    pub const MODIFIED: Color = Color::Rgb(249, 226, 175);   // Yellow #F9E2AF
    pub const ADDED: Color = Color::Rgb(137, 180, 250);      // Blue #89B4FA
    pub const DELETED: Color = Color::Rgb(243, 139, 168);    // Red #F38BA8
    pub const UNTRACKED: Color = Color::Rgb(108, 112, 134);  // Gray #6C7086
    pub const RENAMED: Color = Color::Rgb(203, 166, 247);    // Purple #CBA6F7
    pub const CONFLICTED: Color = Color::Rgb(245, 194, 231); // Pink #F5C2E7

    // UI colors
    pub const BACKGROUND: Color = Color::Rgb(30, 30, 46);    // Base #1E1E2E
    pub const SURFACE: Color = Color::Rgb(49, 50, 68);       // Surface0 #313244
    pub const OVERLAY: Color = Color::Rgb(69, 71, 90);       // Surface1 #45475A
    pub const TEXT: Color = Color::Rgb(205, 214, 244);       // Text #CDD6F4
    pub const SUBTEXT: Color = Color::Rgb(166, 173, 200);    // Subtext0 #A6ADC8
    pub const BORDER: Color = Color::Rgb(88, 91, 112);       // Surface2 #585B70
    pub const ACCENT: Color = Color::Rgb(137, 180, 250);     // Blue #89B4FA
    pub const HIGHLIGHT: Color = Color::Rgb(180, 190, 254);  // Lavender #B4BEFE

    // Diff colors
    pub const DIFF_ADD: Color = Color::Rgb(166, 227, 161);   // Green
    pub const DIFF_DEL: Color = Color::Rgb(243, 139, 168);   // Red
    pub const DIFF_HUNK: Color = Color::Rgb(137, 180, 250);  // Blue

    // Animation colors
    pub const FLASH_BRIGHT: Color = Color::Rgb(255, 230, 150);  // Warm yellow
    pub const FLASH_DIM: Color = Color::Rgb(180, 160, 100);     // Dim yellow
    pub const FLASH_BG: Color = Color::Rgb(80, 70, 50);         // Warm background for flash
}
