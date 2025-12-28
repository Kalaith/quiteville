use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;
use super::colors;

/// Draw the list of zones with interactive buttons
pub fn draw_zone_list(state: &GameState, x: f32, y: f32, w: f32) -> Option<PlayerAction> {
    // Header
    draw_text("Zones", x, y - 10.0, 24.0, colors::TEXT);
    
    let mut action_to_emit = None;
    let card_height = 100.0;
    let margin = 10.0;
    
    for (i, zone) in state.zones.iter().enumerate() {
        let card_y = y + i as f32 * (card_height + margin);
        
        // Draw individual zone card
        if let Some(action) = draw_zone_card(state, zone, i, x, card_y, w, card_height) {
            action_to_emit = Some(action);
        }
    }
    
    action_to_emit
}

fn draw_zone_card(
    state: &GameState, 
    zone: &crate::zones::Zone, 
    index: usize, 
    x: f32, y: f32, w: f32, h: f32
) -> Option<PlayerAction> {
    // Find template
    let template = state.zone_templates.iter().find(|t| t.id == zone.template_id)?;
    
    // Background
    let bg_color = if zone.dormant { 
        Color::new(0.2, 0.2, 0.2, 1.0) 
    } else { 
        colors::PANEL_BG 
    };
    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    // Name & Status
    let name_color = if zone.dormant { GRAY } else { colors::ACCENT };
    draw_text(&template.name, x + 10.0, y + 25.0, 24.0, name_color);
    
    // Stats (Condition, etc.)
    draw_text(
        &format!("Condition: {:.0}%", zone.condition * 100.0),
        x + 10.0, y + 50.0, 18.0, colors::TEXT
    );
    
    let cap_text = if template.population.capacity > 0.0 {
        format!("Housing: {:.0}", template.population.capacity)
    } else {
        "Housing: None".to_string()
    };
    draw_text(&cap_text, x + 10.0, y + 70.0, 16.0, LIGHTGRAY);

    // Interactive Button (Restore)
    // Only if damaged (< 100%) or dormant
    if zone.condition < 1.0 || zone.dormant {
        let btn_w = 120.0;
        let btn_h = 30.0;
        let btn_x = x + w - btn_w - 10.0;
        let btn_y = y + 10.0;
        
        let mouse_pos = mouse_position();
        let mx = mouse_pos.0;
        let my = mouse_pos.1;
        
        let is_hover = mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h;
        
        // Cost check
        let cost = 0.5; // Hardcoded for UI now, should pull from somewhere
        let can_afford = state.resources.materials >= cost;
        
        let btn_color = if can_afford {
            if is_hover { GREEN } else { DARKGREEN }
        } else {
            RED
        };
        
        draw_rectangle(btn_x, btn_y, btn_w, btn_h, btn_color);
        draw_text("Restore (0.5)", btn_x + 5.0, btn_y + 20.0, 16.0, WHITE);
        
        if is_hover && is_mouse_button_pressed(MouseButton::Left) {
            return Some(PlayerAction::RestoreZone(index));
        }
    }
    
    None
}
