use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;
use super::colors;

/// Draw the tech tree window
pub fn draw_tech_tree_window(state: &GameState, x: f32, y: f32, w: f32, h: f32) -> Option<PlayerAction> {
    // Background/Modal
    draw_rectangle(x, y, w, h, colors::PANEL_BG);
    draw_rectangle_lines(x, y, w, h, 2.0, colors::ACCENT);
    
    // Header
    draw_text("Research & Development", x + 20.0, y + 30.0, 30.0, WHITE);
    draw_text("Unlock new capabilities to expand the town.", x + 20.0, y + 55.0, 20.0, LIGHTGRAY);
    
    // Close Button
    if draw_button("Close", x + w - 100.0, y + 10.0, 90.0, 30.0) {
        return Some(PlayerAction::ToggleTechTree);
    }
    
    let center_x = x + w / 2.0;
    let center_y = y + h / 2.0;
    let node_w = 180.0;
    let node_h = 80.0;
    
    let mut clicked_tech = None;
    
    // Draw connections first
    for tech in &state.tech_tree {
        if let Some(parent_id) = &tech.parent_id {
            if let Some(parent) = state.tech_tree.iter().find(|t| &t.id == parent_id) {
                let start_x = center_x + parent.x + node_w/2.0;
                let start_y = center_y + parent.y + node_h/2.0;
                let end_x = center_x + tech.x + node_w/2.0;
                let end_y = center_y + tech.y + node_h/2.0;
                
                let color = if parent.unlocked { WHITE } else { DARKGRAY };
                draw_line(start_x, start_y, end_x, end_y, 2.0, color);
            }
        }
    }
    
    // Draw Nodes
    for tech in &state.tech_tree {
        let nx = center_x + tech.x;
        let ny = center_y + tech.y;
        
        let can_unlock = if tech.unlocked {
            false
        } else if let Some(parent_id) = &tech.parent_id {
            state.tech_tree.iter().find(|t| &t.id == parent_id).map(|p| p.unlocked).unwrap_or(false)
        } else {
            true // No parent = unlocked by default (to buy)
        };
        
        let afford = state.resources.materials >= tech.cost;
        let bg_color = if tech.unlocked {
            GREEN
        } else if can_unlock {
            if afford { BLUE } else { Color::new(0.5, 0.0, 0.0, 1.0) }
        } else {
            DARKGRAY
        };
        
        draw_rectangle(nx, ny, node_w, node_h, bg_color);
        draw_rectangle_lines(nx, ny, node_w, node_h, 2.0, WHITE);
        
        // Text
        draw_text(&tech.name, nx + 5.0, ny + 20.0, 20.0, WHITE);
        if tech.unlocked {
             draw_text("RESEARCHED", nx + 5.0, ny + 40.0, 16.0, LIGHTGRAY);
        } else {
             draw_text(&format!("Cost: {:.0} Mat", tech.cost), nx + 5.0, ny + 40.0, 16.0, YELLOW);
             
             // Tooltip/Description on hover?
             // For now just draw desc
             if can_unlock {
                 // Make it a button
                 let mouse_pos = mouse_position();
                 if mouse_pos.0 >= nx && mouse_pos.0 <= nx + node_w &&
                    mouse_pos.1 >= ny && mouse_pos.1 <= ny + node_h {
                        
                        // Show tooltip
                        let _tooltip = format!("{}\nEffect: {:?}", tech.description, tech.effect);
                        // Simplified tooltip for now
                        draw_text(&tech.description, x + 20.0, y + h - 30.0, 20.0, WHITE);
                        
                        if is_mouse_button_pressed(MouseButton::Left) && afford {
                            clicked_tech = Some(tech.id.clone());
                        }
                 }
             }
        }
    }
    
    if let Some(id) = clicked_tech {
        Some(PlayerAction::Research(id))
    } else {
        None
    }
}

fn draw_button(text: &str, x: f32, y: f32, w: f32, h: f32) -> bool {
    let mouse_pos = mouse_position();
    let is_hover = mouse_pos.0 >= x && mouse_pos.0 <= x + w && mouse_pos.1 >= y && mouse_pos.1 <= y + h;
    
    let color = if is_hover { GRAY } else { DARKGRAY };
    draw_rectangle(x, y, w, h, color);
    draw_text(text, x + 10.0, y + 20.0, 20.0, WHITE);
    
    is_hover && is_mouse_button_pressed(MouseButton::Left)
}
