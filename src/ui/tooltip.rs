//! Tooltip rendering for zones and agents

use macroquad::prelude::*;
use crate::data::{GameState, ZoneTemplate};
use crate::zones::Zone;
use crate::simulation::agents::Agent;

const TOOLTIP_BG: Color = Color::new(0.1, 0.1, 0.1, 0.9);
const TOOLTIP_BORDER: Color = Color::new(0.8, 0.8, 0.8, 1.0);
const TOOLTIP_TEXT: Color = WHITE;
const TOOLTIP_PADDING: f32 = 8.0;
const TOOLTIP_LINE_HEIGHT: f32 = 18.0;

/// Draw a simple tooltip at the given position
pub fn draw_tooltip(text: &str, pos: Vec2) {
    let lines: Vec<&str> = text.lines().collect();
    let max_width = lines.iter()
        .map(|l| measure_text(l, None, 16, 1.0).width)
        .fold(0.0_f32, f32::max);
    
    let width = max_width + TOOLTIP_PADDING * 2.0;
    let height = lines.len() as f32 * TOOLTIP_LINE_HEIGHT + TOOLTIP_PADDING * 2.0;
    
    // Clamp to screen
    let x = pos.x.min(screen_width() - width - 10.0);
    let y = pos.y.min(screen_height() - height - 10.0);
    
    // Background
    draw_rectangle(x, y, width, height, TOOLTIP_BG);
    draw_rectangle_lines(x, y, width, height, 1.0, TOOLTIP_BORDER);
    
    // Text
    for (i, line) in lines.iter().enumerate() {
        draw_text(
            line,
            x + TOOLTIP_PADDING,
            y + TOOLTIP_PADDING + (i as f32 + 0.8) * TOOLTIP_LINE_HEIGHT,
            16.0,
            TOOLTIP_TEXT,
        );
    }
}

/// Draw a tooltip for a zone
pub fn draw_zone_tooltip(zone: &Zone, template: &ZoneTemplate, mouse_pos: Vec2) {
    let mut text = format!("{}\n", template.name);
    text.push_str(&format!("Category: {:?}\n", template.category));
    text.push_str(&format!("Condition: {:.0}%\n", zone.condition * 100.0));
    text.push_str(&format!("Activity: {:.0}%\n", zone.activity * 100.0));
    
    if zone.is_under_construction() {
        let progress = zone.construction_progress(template.construction_work);
        text.push_str(&format!("⚒ Building: {:.0}%", progress * 100.0));
    } else if zone.dormant {
        text.push_str("Status: Dormant");
    } else {
        text.push_str("Status: Active");
    }
    
    draw_tooltip(&text, mouse_pos + vec2(15.0, 15.0));
}

/// Draw a tooltip for an agent
pub fn draw_agent_tooltip(agent: &Agent, mouse_pos: Vec2) {
    let state_name = match agent.state {
        crate::simulation::agents::AgentState::Idle => "Idle",
        crate::simulation::agents::AgentState::Wandering { .. } => "Walking",
        crate::simulation::agents::AgentState::Working { .. } => "Working",
        crate::simulation::agents::AgentState::Shopping { .. } => "Shopping",
        crate::simulation::agents::AgentState::Socializing { .. } => "Socializing",
        crate::simulation::agents::AgentState::GoingHome => "Going Home",
        crate::simulation::agents::AgentState::Sleeping => "Sleeping",
        crate::simulation::agents::AgentState::Building { .. } => "Building",
        crate::simulation::agents::AgentState::Hauling { .. } => "Hauling",
    };
    
    let mut trait_summary = String::new();
    if !agent.traits.is_empty() {
        trait_summary.push_str("\n\nTraits:");
        for tr in &agent.traits {
            trait_summary.push_str(&format!("\n• {} ({:+}% work)", 
                tr.name(), 
                ((tr.work_speed_modifier() - 1.0) * 100.0) as i32
            ));
            // Just accessing description to use the method, could display on advanced hover
            let _desc = tr.description(); 
        }
    }

    let text = format!(
        "Villager #{}\nJob: {}\n{}\n\nEnergy: {:.0}%\nHunger: {:.0}%\nSocial: {:.0}%\nSpirit: {:.0}%{}",
        agent.id % 1000,
        agent.job.name(),
        state_name,
        agent.energy * 100.0,
        (1.0 - agent.hunger) * 100.0, // Invert: low hunger = fed
        agent.social * 100.0,
        agent.spirit * 100.0,
        trait_summary
    );
    
    draw_tooltip(&text, mouse_pos + vec2(15.0, 15.0));
}

/// Check if mouse is hovering over a zone and return it
pub fn get_hovered_zone(state: &GameState, mouse_world_pos: Vec2) -> Option<(usize, &Zone, &ZoneTemplate)> {
    for (idx, zone) in state.zones.iter().enumerate() {
        if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
            if let Some(rect) = template.map_rect {
                let zone_x = rect.x as f32 * crate::ui::map_renderer::TILE_SIZE;
                let zone_y = rect.y as f32 * crate::ui::map_renderer::TILE_SIZE;
                let zone_w = rect.w as f32 * crate::ui::map_renderer::TILE_SIZE;
                let zone_h = rect.h as f32 * crate::ui::map_renderer::TILE_SIZE;
                
                if mouse_world_pos.x >= zone_x && mouse_world_pos.x <= zone_x + zone_w &&
                   mouse_world_pos.y >= zone_y && mouse_world_pos.y <= zone_y + zone_h {
                    return Some((idx, zone, template));
                }
            }
        }
    }
    None
}

/// Check if mouse is hovering over an agent and return it
pub fn get_hovered_agent(state: &GameState, mouse_world_pos: Vec2) -> Option<&Agent> {
    const AGENT_RADIUS: f32 = 16.0;
    for agent in &state.agents {
        if agent.pos.distance(mouse_world_pos) < AGENT_RADIUS {
            return Some(agent);
        }
    }
    None
}
