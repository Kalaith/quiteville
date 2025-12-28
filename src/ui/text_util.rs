use macroquad::prelude::*;

/// Break a long string into lines that fit within a pixel width
pub fn wrap_text(text: &str, font_size: f32, max_width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in text.split_whitespace() {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };
        
        let dimensions = measure_text(&test_line, None, font_size as u16, 1.0);
        
        if dimensions.width > max_width {
            // Line is full, push current_line and start new
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = word.to_string();
            } else {
                // Formatting error: single word too long? Push it anyway.
                lines.push(word.to_string());
                current_line = String::new();
            }
        } else {
            current_line = test_line;
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    lines
}
