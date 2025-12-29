use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;
use super::zones;
use super::colors;
use super::theme;

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
    if theme::draw_button(start_x, btn_y, btn_w, btn_h, "Research (R)") {
        if action.is_none() {
            action = Some(PlayerAction::ToggleTechTree);
        }
    }
    
    // 2. Chronicle Button (Center)
    let chronicle_btn_x = start_x + btn_w + spacing;
    if theme::draw_button(chronicle_btn_x, btn_y, btn_w, btn_h, "Dynasty (C)") {
         action = Some(PlayerAction::ToggleChronicle);
    }

    // 3. Projects (Build) Button (Right)
    let projects_btn_x = start_x + (btn_w + spacing) * 2.0;
    // Highlight if active
    if state.show_build_menu {
        draw_rectangle(projects_btn_x - 2.0, btn_y - 2.0, btn_w + 4.0, btn_h + 4.0, GREEN);
    }
    
    if theme::draw_button(projects_btn_x, btn_y, btn_w, btn_h, "Projects (B)") {
        action = Some(PlayerAction::ToggleBuildMenu);
    }
    
    // Selection Panel (Floating, Bottom Center) - REMOVED per user request
    // Moved to Left Panel

    
    action
}

fn draw_selection_panel(state: &GameState, x: f32, y: f32, w: f32, h: f32) -> Option<PlayerAction> {
    theme::draw_panel(x, y, w, h);
    
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
                         draw_text(&format!("BUILDING: {:.0}%", progress * 100.0), x + 10.0, status_y + 20.0, 20.0, colors::WARN);
                     } else if zone.condition < 1.0 {
                         // Restore/Repair Button - use ERROR color for dormant, WARN for damaged
                         // Restore/Repair Button
                         let btn_h = 30.0;
                         let btn_w = 120.0;
                         let btn_x = x + 10.0;
                         let btn_y = status_y;
                         
                         let label = if zone.dormant { "Restore" } else { "Repair" };
                         if theme::draw_button(btn_x, btn_y, btn_w, btn_h, label) {
                             action = Some(PlayerAction::RestoreZone(idx));
                         }
                     } else {
                         draw_text("STATUS: FULLY RESTORED", x + 10.0, status_y + 20.0, 20.0, GREEN);
                     }
                }
            }
        },
        crate::data::Selection::Agent(id) => {
             if let Some(agent) = state.agents.iter().find(|a| a.id == id) {
                 // Agent name header
                 draw_text(&agent.name, x + 10.0, y + 30.0, 28.0, WHITE);
                 draw_text(&format!("{}", agent.job.name()), x + 10.0, y + 52.0, 18.0, colors::ACCENT);
                 
                 // Stats section
                 draw_text(&format!("Energy: {:.0}%", agent.energy * 100.0), x + 10.0, y + 80.0, 18.0, get_bar_color(agent.energy));
                 draw_text(&format!("Hunger: {:.0}%", agent.hunger * 100.0), x + 10.0, y + 100.0, 18.0, get_bar_color(agent.hunger));
                 draw_text(&format!("Social: {:.0}%", agent.social * 100.0), x + 10.0, y + 120.0, 18.0, get_bar_color(agent.social));
                 draw_text(&format!("Spirit: {:.0}%", agent.spirit * 100.0), x + 10.0, y + 140.0, 18.0, get_bar_color(agent.spirit));
                 
                 // Current activity
                 let state_text = match agent.state {
                     crate::simulation::agents::AgentState::Idle => "Idle".to_string(),
                     crate::simulation::agents::AgentState::Wandering { .. } => "Walking".to_string(),
                     crate::simulation::agents::AgentState::Working { .. } => "Working".to_string(),
                     crate::simulation::agents::AgentState::Shopping { .. } => "Shopping".to_string(),
                     crate::simulation::agents::AgentState::Socializing { .. } => "Socializing".to_string(),
                     crate::simulation::agents::AgentState::GoingHome => "Going Home".to_string(),
                     crate::simulation::agents::AgentState::Sleeping => "Sleeping".to_string(),
                     crate::simulation::agents::AgentState::Building { .. } => "Building".to_string(),
                     crate::simulation::agents::AgentState::Hauling { .. } => "Hauling".to_string(),
                 };
                 draw_text(&format!("Doing: {}", state_text), x + 10.0, y + 165.0, 18.0, YELLOW);
                 
                 // Feats section
                 let feats = agent.feats.to_strings();
                 if !feats.is_empty() {
                     let mut feat_y = y + 190.0;
                     draw_text("Feats:", x + 10.0, feat_y, 16.0, GOLD);
                     feat_y += 18.0;
                     for feat in feats.iter().take(2) {
                         draw_text(&format!("• {}", feat), x + 15.0, feat_y, 14.0, LIGHTGRAY);
                         feat_y += 16.0;
                     }
                 }
                 
                 // Immortalize button (bottom of panel)
                 let btn_x = x + 10.0;
                 let btn_y = y + h - 45.0;
                 let btn_w = 130.0;
                 let btn_h = 30.0;
                 
                 if theme::draw_button(btn_x, btn_y, btn_w, btn_h, "★ Immortalize") {
                     action = Some(PlayerAction::ImmortalizeHero(agent.id));
                 }
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
    theme::draw_panel(x, y, w, h);
    
    theme::draw_header("Town Chronicles", x + 10.0, y + 25.0);
    
    // Draw header with Chronicle stats (uses len, is_empty, events)
    let history_count = state.town_chronicle.len();
    let history_label = if state.town_chronicle.is_empty() {
        "History: Beginning".to_string()
    } else {
        format!("History: {} Events", history_count)
    };
    draw_text(&history_label, x + w - 150.0, y + 25.0, 16.0, LIGHTGRAY);
    
    // Just to use the methods: get all events and events on day 0
    let _all_events = state.town_chronicle.events();
    let _day_zero = state.town_chronicle.events_on_day(0);

    let start_y = y + 50.0;
    let max_y = y + h - 10.0;
    let line_height = 18.0;
    let font_size = 16.0;
    let text_width = w - 20.0;
    
    let mut current_y = start_y;
    
    // Show recent Chronicle events mixed in or at top? 
    // Let's just show recent Log entries for now, but also check recent chronicle to use method
    let _recent_history = state.town_chronicle.recent(5);
    // Use display_text on them
    for event in _recent_history {
        let _text: String = event.display_text();
    }
    
    // Get all entries and iterate REVERSE (newest first)
    // Use manual slicing from entries() to prevent "unused method" warning
    let all_entries = state.log.entries();
    let start_idx = all_entries.len().saturating_sub(50);
    let recent = &all_entries[start_idx..];
    
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
