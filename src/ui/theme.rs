use macroquad::prelude::*;

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

/// Helper to draw a stylized panel
pub fn draw_panel(x: f32, y: f32, w: f32, h: f32) {
    // Shadow
    draw_rectangle(x + 4.0, y + 4.0, w, h, Color::new(0.0, 0.0, 0.0, 0.5));
    
    // Main Body
    draw_rectangle(x, y, w, h, colors::PANEL_BG);
    
    // Border
    draw_rectangle_lines(x, y, w, h, 2.0, colors::PANEL_BORDER);
    
    // Inner bevel (fake)
    draw_rectangle_lines(x + 2.0, y + 2.0, w - 4.0, h - 4.0, 1.0, Color::new(0.2, 0.2, 0.22, 0.5));
}

/// Helper to draw a standard button
/// Returns true if clicked
pub fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    let mouse_pos = mouse_position();
    let is_hover = mouse_pos.0 >= x && mouse_pos.0 <= x + w && mouse_pos.1 >= y && mouse_pos.1 <= y + h;
    let is_clicked = is_hover && is_mouse_button_released(MouseButton::Left);
    let is_pressed = is_hover && is_mouse_button_down(MouseButton::Left);
    
    let bg_color = if is_pressed {
        colors::BUTTON_BG
    } else if is_hover {
        colors::BUTTON_HOVER
    } else {
        colors::BUTTON_BG
    };
    
    let offset_y = if is_pressed { 2.0 } else { 0.0 };
    
    // Shadow
    if !is_pressed {
         draw_rectangle(x + 2.0, y + 2.0, w, h, Color::new(0.0, 0.0, 0.0, 0.3));
    }
    
    draw_rectangle(x, y + offset_y, w, h, bg_color);
    draw_rectangle_lines(x, y + offset_y, w, h, 2.0, colors::PANEL_BORDER);
    
    // Text centering
    let font_size = 18.0;
    let text_dim = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + (w - text_dim.width) / 2.0;
    let text_y = y + offset_y + (h + text_dim.height) / 2.0 - 2.0; // Visual adjustment
    
    draw_text(text, text_x, text_y, font_size, colors::TEXT);
    
    is_clicked
}

/// Helper to draw a header text
pub fn draw_header(text: &str, x: f32, y: f32) {
    draw_text(text, x + 2.0, y + 2.0, 30.0, Color::new(0.0, 0.0, 0.0, 0.5)); // Shadow
    draw_text(text, x, y, 30.0, colors::ACCENT);
}
