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
    let panel_width = 350.0;
    
    // Left Panel (Log & Details)
    draw_log_panel(state, 10.0, top_margin, panel_width, screen_h - top_margin - 10.0);
    
    // Right Panel (Zones / Projects) - Hidden by default
    let zones_x = screen_w - panel_width - 10.0;
    let list_h = screen_h - top_margin - 10.0;
    let mut action = if state.show_build_menu {
        zones::draw_zone_list(state, zones_x, top_margin, panel_width, list_h)
    } else {
        None
    };
    
    // Bottom Right Buttons Area
    let btn_w = 120.0;
    let btn_h = 40.0;
    let spacing = 10.0;
    
    // Projects (Build) Button
    let projects_btn_x = screen_w - btn_w - 20.0;
    let projects_btn_y = screen_h - btn_h - 20.0;
    
    // Button color changes if menu is open
    let projects_color = if state.show_build_menu { GREEN } else { colors::ACCENT };
    
    draw_rectangle(projects_btn_x, projects_btn_y, btn_w, btn_h, projects_color);
    draw_rectangle_lines(projects_btn_x, projects_btn_y, btn_w, btn_h, 2.0, WHITE);
    draw_text("Projects", projects_btn_x + 20.0, projects_btn_y + 25.0, 20.0, BLACK);
    
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.0 >= projects_btn_x && mouse_pos.0 <= projects_btn_x + btn_w && 
           mouse_pos.1 >= projects_btn_y && mouse_pos.1 <= projects_btn_y + btn_h {
               action = Some(PlayerAction::ToggleBuildMenu);
        }
    }
    
    // Research Button (Above Projects)
    let research_btn_x = projects_btn_x;
    let research_btn_y = projects_btn_y - btn_h - spacing;
    
    // Draw Research Button
    draw_rectangle(research_btn_x, research_btn_y, btn_w, btn_h, colors::ACCENT);
    draw_rectangle_lines(research_btn_x, research_btn_y, btn_w, btn_h, 2.0, WHITE);
    draw_text("Research", research_btn_x + 20.0, research_btn_y + 25.0, 20.0, BLACK);
    
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.0 >= research_btn_x && mouse_pos.0 <= research_btn_x + btn_w && 
           mouse_pos.1 >= research_btn_y && mouse_pos.1 <= research_btn_y + btn_h {
               if action.is_none() {
                   action = Some(PlayerAction::ToggleTechTree);
               }
           }
    }
    
    // Selection Panel (Floating, Bottom Center)
    if let crate::data::Selection::None = state.selection {
        // No selection
    } else {
        draw_selection_panel(state, screen_w / 2.0 - 200.0, screen_h - 200.0, 400.0, 180.0);
    }
    
    action
}

fn draw_selection_panel(state: &GameState, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, colors::PANEL_BG);
    draw_rectangle_lines(x, y, w, h, 1.0, colors::ACCENT);
    
    match state.selection {
        crate::data::Selection::Zone(idx) => {
            if let Some(zone) = state.zones.get(idx) {
                if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
                     draw_text(&template.name, x + 10.0, y + 30.0, 30.0, WHITE);
                     draw_text(&format!("Condition: {:.0}%", zone.condition * 100.0), x + 10.0, y + 60.0, 20.0, LIGHTGRAY);
                     draw_text(&format!("Activity: {:.0}%", zone.activity * 100.0), x + 10.0, y + 85.0, 20.0, LIGHTGRAY);
                     
                     // Outputs
                     let mut output_y = 115.0;
                     if template.population.capacity > 0.0 {
                         draw_text(&format!("Housing: {:.0}", template.population.capacity), x + 10.0, y + output_y, 20.0, WHITE);
                     }
                     
                     if template.output.materials > 0.0 {
                         draw_text(&format!("Production: +{:.3} Material", template.output.materials), x + 10.0, y + output_y, 20.0, WHITE);
                     }
                     
                     // Status line at bottom (adjusted Y)
                     let status_y = y + 150.0;
                     if zone.dormant {
                         draw_text("STATUS: DORMANT (Restore in Zone List)", x + 10.0, status_y, 20.0, RED);
                     } else {
                         draw_text("STATUS: ACTIVE", x + 10.0, status_y, 20.0, GREEN);
                     }
                }
            }
        },
        crate::data::Selection::Agent(id) => {
             if let Some(agent) = state.agents.iter().find(|a| a.id == id) {
                 draw_text(&format!("Villager #{}", id % 1000), x + 10.0, y + 30.0, 30.0, WHITE);
                 draw_text(&format!("Energy: {:.0}%", agent.energy * 100.0), x + 10.0, y + 60.0, 20.0, get_bar_color(agent.energy));
                 draw_text(&format!("Hunger: {:.0}%", agent.hunger * 100.0), x + 10.0, y + 85.0, 20.0, get_bar_color(agent.hunger));
                 draw_text(&format!("Social: {:.0}%", agent.social * 100.0), x + 10.0, y + 110.0, 20.0, get_bar_color(agent.social));
                 
                 let state_text = match agent.state {
                     crate::simulation::agents::AgentState::Idle => "Idle".to_string(),
                     crate::simulation::agents::AgentState::Wandering { .. } => "Wandering".to_string(),
                     crate::simulation::agents::AgentState::Working { .. } => "Working".to_string(),
                     crate::simulation::agents::AgentState::Shopping { .. } => "Shopping".to_string(),
                     crate::simulation::agents::AgentState::Socializing { .. } => "Socializing".to_string(),
                     crate::simulation::agents::AgentState::GoingHome => "Going Home".to_string(),
                 };
                 draw_text(&format!("Doing: {}", state_text), x + 10.0, y + 150.0, 20.0, YELLOW);
             }
        },
        _ => {}
    }
}

fn get_bar_color(val: f32) -> Color {
    if val > 0.7 { GREEN } else if val > 0.3 { YELLOW } else { RED }
}

fn draw_log_panel(state: &GameState, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, colors::PANEL_BG);
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    draw_text("Town Chronicles", x + 10.0, y + 25.0, 24.0, colors::ACCENT);
    
    let start_y = y + 50.0;
    let max_y = y + h - 10.0;
    let line_height = 18.0;
    let font_size = 16.0;
    let text_width = w - 20.0;
    
    let mut current_y = start_y;
    
    // Get all entries and iterate REVERSE (newest first)
    // Use a larger window then break when we fill the box
    let recent = state.log.recent(50); 
    
    for entry in recent.iter().rev() {
        if current_y >= max_y { break; }
        
        let color = match entry.category {
             crate::narrative::LogCategory::System => colors::TEXT,
             crate::narrative::LogCategory::Zone => GREEN,
             crate::narrative::LogCategory::Population => PURPLE,
             crate::narrative::LogCategory::Milestone => GOLD,
             crate::narrative::LogCategory::Event => ORANGE,
        };
        
        let full_text = format!("[{:.1}h] {}", entry.timestamp, entry.message);
        let wrapped_lines = super::text_util::wrap_text(&full_text, font_size, text_width);
        
        for line in wrapped_lines {
             if current_y + line_height > max_y { break; }
             draw_text(&line, x + 10.0, current_y + 12.0, font_size, color);
             current_y += line_height;
        }
        
        // Extra spacing between entries
        current_y += 5.0;
    }
}
