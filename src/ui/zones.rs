use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;
use super::colors;

/// Draw the list of zones with interactive buttons
pub fn draw_zone_list(state: &GameState, x: f32, y: f32, w: f32, h: f32) -> Option<PlayerAction> {
    
    // Draw Background Panel for the List
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.1, 0.1, 0.95));
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    // Header
    draw_text("Zones", x + 10.0, y + 25.0, 24.0, colors::TEXT);
    
    let list_y = y + 40.0;
    let list_h = h - 50.0; // Margin at bottom
    
    // Content metrics
    let card_height = 100.0;
    let margin = 10.0;
    let total_content_h = (state.zones.len() as f32) * (card_height + margin);
    
    // Mouse Interaction for Scroll
    let mouse_pos = mouse_position();
    let is_hover = mouse_pos.0 >= x && mouse_pos.0 <= x + w && mouse_pos.1 >= list_y && mouse_pos.1 <= list_y + list_h;
    
    let mut scroll_action = None;
    
    if is_hover {
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            // Scroll speed
            let scroll_speed = 30.0;
            let delta = -wheel_y * scroll_speed;
             
            // Calculate new offset potential
            // We can't modify state here, so emit action
            scroll_action = Some(PlayerAction::ScrollZones(delta));
        }
    }
    
    // Clipping (Scissor)
    unsafe {
        let mut gl = macroquad::window::get_internal_gl();
        gl.flush(); // Flush batch before setting scissor
        
        let screen_h = screen_height();
        // Convert Y to bottom-up
        let scissor_y = screen_h - (list_y + list_h);
        
        // Ensure values are positive integers
        let sx = x as i32;
        let sy = scissor_y as i32;
        let sw = w as i32;
        let sh = list_h as i32;
        
        gl.quad_gl.scissor(Some((sx, sy, sw, sh)));
    }
    
    let mut action_to_emit = scroll_action;
    
    for (i, zone) in state.zones.iter().enumerate() {
        let card_realtive_y = i as f32 * (card_height + margin) - state.zones_scroll_offset;
        let card_y = list_y + card_realtive_y;
        
        // Visibility Culling
        if card_realtive_y + card_height < 0.0 || card_realtive_y > list_h {
            continue;
        }
        
        // Draw individual zone card
        if let Some(action) = draw_zone_card(state, zone, i, x + 5.0, card_y, w - 10.0, card_height) {
            action_to_emit = Some(action);
        }
    }
    
    // End Scissor
    unsafe {
        let mut gl = macroquad::window::get_internal_gl();
        gl.flush();
        gl.quad_gl.scissor(None);
    }
    
    // Draw Scrollbar if needed
    if total_content_h > list_h {
        let scroll_pct = state.zones_scroll_offset / (total_content_h - list_h);
        let bar_h = (list_h / total_content_h) * list_h;
        let bar_y = list_y + scroll_pct * (list_h - bar_h);
        let bar_x = x + w - 5.0;
        
        draw_rectangle(bar_x, bar_y.clamp(list_y, list_y + list_h - bar_h), 4.0, bar_h, LIGHTGRAY);
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
    // Stats & Effects
    let mut y_offset = 50.0;
    
    draw_text(
        &format!("Condition: {:.0}%", zone.condition * 100.0),
        x + 10.0, y + y_offset, 18.0, colors::TEXT
    );
    y_offset += 20.0;
    
    // Effects Summary
    let mut effects = Vec::new();
    
    if template.population.capacity > 0.0 {
        effects.push(format!("Housing: +{:.0}", template.population.capacity));
    }
    
    if template.output.materials > 0.0 {
         effects.push(format!("Materials: +{:.3}", template.output.materials));
    }
    
    if template.output.attractiveness > 0.0 {
        effects.push(format!("Attract: +{:.1}", template.output.attractiveness));
    }
    
    if template.output.stability > 0.0 {
        effects.push(format!("Stability: +{:.1}", template.output.stability));
    }
    
    // Join effects into a single line or two
    // For small card, maybe just 1-2 lines.
    let effects_str = effects.join(", ");
    draw_text(&effects_str, x + 10.0, y + y_offset, 16.0, LIGHTGRAY);

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
        let cost = template.construction_cost;
        let can_afford = state.resources.materials >= cost;
        
        let btn_color = if can_afford {
            if is_hover { GREEN } else { DARKGREEN }
        } else {
            RED
        };
        
        draw_rectangle(btn_x, btn_y, btn_w, btn_h, btn_color);
        draw_text(&format!("Restore ({:.1})", cost), btn_x + 5.0, btn_y + 20.0, 16.0, WHITE);
        
        if is_hover && is_mouse_button_pressed(MouseButton::Left) {
            return Some(PlayerAction::RestoreZone(index));
        }
    }
    
    None
}
