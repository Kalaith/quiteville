/// Break a long string into lines that fit within a pixel width
pub fn wrap_text(text: &str, font_size: f32, max_width: f32) -> Vec<String> {
    macroquad_toolkit::ui::wrap_text(text, max_width, font_size)
}
