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
    let left_h = screen_h - top_margin - 10.0;
    let log_h = left_h * 0.6;
    let details_h = left_h * 0.4;
    
    draw_log_panel(state, 10.0, top_margin, panel_width, log_h);
    
    // Draw Details Panel (if selection exists)
    let panel_action = if let crate::data::Selection::None = state.selection {
        None
    } else {
        draw_selection_panel(state, 10.0, top_margin + log_h + 5.0, panel_width, details_h - 5.0)
    };
    
    // Right Panel (Zones / Projects) - Hidden by default
    let zones_x = screen_w - panel_width - 10.0;
    // Stop above buttons (approx 80px from bottom)
    let list_h = screen_h - top_margin - 80.0;
    
    // Combine actions (prioritize panel action if any, or list action)
    let mut action = panel_action;
    
    if state.show_build_menu {
        if let Some(act) = zones::draw_zone_list(state, zones_x, top_margin, panel_width, list_h) {
            if action.is_none() {
                action = Some(act);
            }
        }
    }
    
    // Bottom Center Buttons Area
    let btn_w = 120.0;
    let btn_h = 40.0;
    let spacing = 10.0;
    
    // Calculate centered position for 2 buttons
    let total_w = btn_w * 2.0 + spacing;
    let start_x = (screen_w - total_w) / 2.0;
    let btn_y = screen_h - btn_h - 20.0;
    
    // 1. Research Button (Left)
    let research_btn_x = start_x;
    let research_btn_y = btn_y;
    
    draw_rectangle(research_btn_x, research_btn_y, btn_w, btn_h, colors::ACCENT);
    draw_rectangle_lines(research_btn_x, research_btn_y, btn_w, btn_h, 2.0, WHITE);
    draw_text("Research (R)", research_btn_x + 10.0, research_btn_y + 25.0, 18.0, BLACK); // Added (R) hint
    
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.0 >= research_btn_x && mouse_pos.0 <= research_btn_x + btn_w && 
           mouse_pos.1 >= research_btn_y && mouse_pos.1 <= research_btn_y + btn_h {
               if action.is_none() {
                   action = Some(PlayerAction::ToggleTechTree);
               }
           }
    }
    
    // 2. Projects (Build) Button (Right)
    let projects_btn_x = start_x + btn_w + spacing;
    let projects_btn_y = btn_y;
    
    let projects_color = if state.show_build_menu { GREEN } else { colors::ACCENT };
    
    draw_rectangle(projects_btn_x, projects_btn_y, btn_w, btn_h, projects_color);
    draw_rectangle_lines(projects_btn_x, projects_btn_y, btn_w, btn_h, 2.0, WHITE);
    draw_text("Projects (B)", projects_btn_x + 10.0, projects_btn_y + 25.0, 18.0, BLACK); // Added (B) hint
    
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if mouse_pos.0 >= projects_btn_x && mouse_pos.0 <= projects_btn_x + btn_w && 
           mouse_pos.1 >= projects_btn_y && mouse_pos.1 <= projects_btn_y + btn_h {
               action = Some(PlayerAction::ToggleBuildMenu);
        }
    }
    
    // Selection Panel (Floating, Bottom Center) - REMOVED per user request
    // Moved to Left Panel

    
    action
}

fn draw_selection_panel(state: &GameState, x: f32, y: f32, w: f32, h: f32) -> Option<PlayerAction> {
    draw_rectangle(x, y, w, h, colors::PANEL_BG);
    draw_rectangle_lines(x, y, w, h, 1.0, colors::ACCENT);
    
    let mut action = None;
    
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
                         output_y += 25.0;
                     }
                     
                     if template.output.materials > 0.0 {
                         draw_text(&format!("Production: +{:.3} Material", template.output.materials), x + 10.0, y + output_y, 20.0, WHITE);
                         output_y += 25.0;
                     }
                     
                     // Status line at bottom (adjusted Y dynamically)
                     let status_y = y + h - 50.0; // Stick to bottom
                     
                     if zone.is_under_construction() {
                         // Show construction progress
                         let progress = zone.construction_progress(template.construction_work);
                         draw_text(&format!("BUILDING: {:.0}%", progress * 100.0), x + 10.0, status_y + 20.0, 20.0, YELLOW);
                     } else if zone.condition < 1.0 {
                         // Restore/Repair Button
                         let btn_h = 30.0;
                         let btn_w = 120.0;
                         let btn_x = x + 10.0;
                         let btn_y = status_y;
                         
                         draw_rectangle(btn_x, btn_y, btn_w, btn_h, if zone.dormant { RED } else { ORANGE });
                         draw_rectangle_lines(btn_x, btn_y, btn_w, btn_h, 2.0, WHITE);
                         let label = if zone.dormant { "Restore" } else { "Repair" };
                         draw_text(label, btn_x + 20.0, btn_y + 20.0, 20.0, WHITE);
                         
                         // Show cost next to it
                         draw_text(&format!("Cost: {:.0}", template.construction_cost), btn_x + btn_w + 10.0, btn_y + 20.0, 20.0, LIGHTGRAY);
                         
                         if is_mouse_button_pressed(MouseButton::Left) {
                             let mouse_pos = mouse_position();
                             if mouse_pos.0 >= btn_x && mouse_pos.0 <= btn_x + btn_w &&
                                mouse_pos.1 >= btn_y && mouse_pos.1 <= btn_y + btn_h {
                                    action = Some(PlayerAction::RestoreZone(idx));
                             }
                         }
                     } else {
                         draw_text("STATUS: FULLY RESTORED", x + 10.0, status_y + 20.0, 20.0, GREEN);
                     }
                }
            }
        },
        crate::data::Selection::Agent(id) => {
             if let Some(agent) = state.agents.iter().find(|a| a.id == id) {
                 draw_text(&format!("Villager #{}", id % 1000), x + 10.0, y + 30.0, 30.0, WHITE);
                 draw_text(&format!("Job: {}", agent.job.name()), x + 10.0, y + 55.0, 18.0, colors::ACCENT);
                 draw_text(&format!("Energy: {:.0}%", agent.energy * 100.0), x + 10.0, y + 80.0, 20.0, get_bar_color(agent.energy));
                 draw_text(&format!("Hunger: {:.0}%", agent.hunger * 100.0), x + 10.0, y + 105.0, 20.0, get_bar_color(agent.hunger));
                 draw_text(&format!("Social: {:.0}%", agent.social * 100.0), x + 10.0, y + 130.0, 20.0, get_bar_color(agent.social));
                 draw_text(&format!("Spirit: {:.0}%", agent.spirit * 100.0), x + 10.0, y + 155.0, 20.0, get_bar_color(agent.spirit));
                 
                 let state_text = match agent.state {
                     crate::simulation::agents::AgentState::Idle => "Idle".to_string(),
                     crate::simulation::agents::AgentState::Wandering { .. } => "Walking".to_string(),
                     crate::simulation::agents::AgentState::Working { .. } => "Working".to_string(),
                     crate::simulation::agents::AgentState::Shopping { .. } => "Shopping".to_string(),
                     crate::simulation::agents::AgentState::Socializing { .. } => "Socializing".to_string(),
                     crate::simulation::agents::AgentState::GoingHome => "Going Home".to_string(),
                     crate::simulation::agents::AgentState::Sleeping => "Sleeping".to_string(),
                     crate::simulation::agents::AgentState::Building { .. } => "Building".to_string(),
                 };
                 draw_text(&format!("Doing: {}", state_text), x + 10.0, y + 185.0, 20.0, YELLOW);
             }
        },
        _ => {}
    }
    
    action
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
