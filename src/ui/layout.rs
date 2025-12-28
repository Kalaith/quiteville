use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;
use super::zones;
use super::colors;

/// Draw the main content layout
pub fn draw_main_layout(state: &GameState) -> Option<PlayerAction> {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let top_margin = 70.0;
    let split_x = screen_w * 0.4; // 40% Left, 60% Right
    
    // Left Panel (Log & Details)
    // For now just Log
    draw_log_panel(state, 10.0, top_margin, split_x - 20.0, screen_h - top_margin - 10.0);
    
    // Right Panel (Zones)
    let action = zones::draw_zone_list(state, split_x + 10.0, top_margin, screen_w - split_x - 20.0);
    
    action
}

fn draw_log_panel(state: &GameState, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, colors::PANEL_BG);
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    draw_text("Town Chronicles", x + 10.0, y + 25.0, 24.0, colors::ACCENT);
    
    let start_y = y + 50.0;
    let line_height = 20.0;
    // Show last 15-20 lines?
    // Using simple recent log for now.
    let recent = state.log.recent(15);
    
    for (i, entry) in recent.iter().enumerate() {
        let text_y = start_y + i as f32 * line_height;
        if text_y > y + h - 10.0 { break; }
        
        let color = match entry.category {
             crate::narrative::LogCategory::System => colors::TEXT,
             crate::narrative::LogCategory::Zone => GREEN,
             crate::narrative::LogCategory::Population => PURPLE,
             crate::narrative::LogCategory::Milestone => GOLD,
             crate::narrative::LogCategory::Event => ORANGE,
        };
        
        draw_text(
            &format!("[{:.1}h] {}", entry.timestamp, entry.message),
            x + 10.0, text_y, 16.0, color
        );
    }
}
