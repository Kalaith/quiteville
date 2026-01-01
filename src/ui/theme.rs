use macroquad::prelude::*;

// Import toolkit utilities
pub use macroquad_toolkit::ui::panel_with_shadow as draw_panel;

pub mod colors {
    use macroquad::prelude::*;
    pub const PANEL_BG: Color = Color::new(0.15, 0.15, 0.18, 0.95);
    pub const PANEL_BORDER: Color = Color::new(0.3, 0.3, 0.35, 1.0);
    pub const TEXT: Color = Color::new(0.9, 0.9, 0.9, 1.0);
    pub const ACCENT: Color = Color::new(0.4, 0.8, 0.4, 1.0);
    pub const SECONDARY: Color = Color::new(0.6, 0.5, 0.8, 1.0);  // Purple for ancestors
    pub const WARN: Color = Color::new(0.9, 0.6, 0.2, 1.0);
    pub const BUTTON_BG: Color = Color::new(0.25, 0.25, 0.3, 1.0);
    pub const BUTTON_HOVER: Color = Color::new(0.35, 0.35, 0.4, 1.0);
}

/// Helper to draw a standard button
/// Returns true if clicked
pub fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    let style = macroquad_toolkit::ui::ButtonStyle {
        normal: colors::BUTTON_BG,
        hovered: colors::BUTTON_HOVER,
        pressed: colors::BUTTON_BG,
        border: colors::PANEL_BORDER,
        text_color: colors::TEXT,
    };

    // Quiteville uses on_release behavior
    macroquad_toolkit::ui::button_on_release(x, y, w, h, text, &style)
}

/// Helper to draw a header text
pub fn draw_header(text: &str, x: f32, y: f32) {
    draw_text(text, x + 2.0, y + 2.0, 30.0, Color::new(0.0, 0.0, 0.0, 0.5)); // Shadow
    draw_text(text, x, y, 30.0, colors::ACCENT);
}
