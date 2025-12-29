use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;
use crate::ui::theme;
use crate::ui::theme::colors;

/// Draw the active guide dialog if any
pub fn draw_guide_dialog(state: &GameState) -> Option<PlayerAction> {
    if let Some(dialog) = &state.tutorial.active_dialog {
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        let w = 600.0;
        let h = 200.0;
        let x = (screen_w - w) / 2.0;
        let y = screen_h - h - 100.0; // Bottom center
        
        // Modal logic (block clicks behind?)
        // If modal, maybe draw a dark overlay
        if dialog.is_modal {
            draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.5));
        }
        
        theme::draw_panel(x, y, w, h);
        
        // Portrait Area (Left)
        let portrait_size = 120.0;
        let portrait_x = x + 20.0;
        let portrait_y = y + (h - portrait_size)/2.0;
        
        draw_rectangle(portrait_x, portrait_y, portrait_size, portrait_size, DARKGRAY);
        draw_rectangle_lines(portrait_x, portrait_y, portrait_size, portrait_size, 2.0, colors::ACCENT);
        
        // Draw Portrait Text or Texture
        // Placeholder for Artie
        if let Some(tex) = state.assets.get("agent_villager") { // Use generic villager for now
             draw_texture_ex(
                tex, 
                portrait_x, portrait_y, 
                WHITE, 
                DrawTextureParams {
                    dest_size: Some(vec2(portrait_size, portrait_size)),
                    ..Default::default()
                }
            );
        } else {
            draw_text("Artie", portrait_x + 30.0, portrait_y + 60.0, 20.0, WHITE);
        }
        
        // Content Area (Right)
        let content_x = portrait_x + portrait_size + 20.0;
        let content_y = y + 20.0;
        
        // Speaker Name
        theme::draw_header(&dialog.speaker, content_x, content_y);
        
        // Dialogue Text
        // Needs wrapping
        let font_size = 20.0;
        let text_w = w - (content_x - x) - 20.0;
        
        let wrapped = crate::ui::text_util::wrap_text(&dialog.text, font_size, text_w);
        let mut text_y = content_y + 40.0;
        for line in wrapped {
            draw_text(&line, content_x, text_y, font_size, colors::TEXT);
            text_y += 24.0;
        }
        
        // Buttons row
        let btn_w = 100.0;
        let btn_h = 40.0;
        let btn_y = y + h - btn_h - 20.0;
        
        // Skip Tutorial button (left)
        if !state.tutorial.is_complete() {
            let skip_x = x + 20.0 + portrait_size + 20.0;
            if theme::draw_button(skip_x, btn_y, btn_w, btn_h, "Skip") {
                return Some(PlayerAction::SkipTutorial);
            }
        }
        
        // Got it button (right)
        let btn_x = x + w - btn_w - 20.0;
        if theme::draw_button(btn_x, btn_y, btn_w, btn_h, "Got it") {
            return Some(PlayerAction::DismissDialog);
        }
        
        // Special case: if modal, block all other interactions by consuming input?
        // Returning None here means no other UI consumes it, but we drew an overlay.
        // Actually, Input handling is done via "if mouse clicked check UI".
        // Since this is drawn last (on top), its button check happens last? 
        // No, typically UI is drawn back-to-front. 
        // If we want to block input, `draw_game_ui` needs to handle it.
    }
    
    None
}
