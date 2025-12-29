use macroquad::prelude::*;
use crate::data::GameState;
use crate::ui::theme::colors;

/// Draw the top bar with resources and time
pub fn draw_top_bar(state: &GameState, time_scale: f32, paused: bool) {
    let screen_w = screen_width();
    let bar_height = 60.0;
    
    // Background
    draw_rectangle(0.0, 0.0, screen_w, bar_height, colors::PANEL_BG);
    draw_line(0.0, bar_height, screen_w, bar_height, 1.0, GRAY);
    
    // Time & Status (Left)
    let status_text = if paused { "PAUSED" } else { "RUNNING" };
    let time_text = format!("Time: {:.1}h | x{:.0} | {}", state.game_time_hours, time_scale, status_text);
    draw_text(&time_text, 10.0, 35.0, 20.0, colors::TEXT);
    
    // Resources (Right/Center)
    // Layout: Materials | Maintenance | Attractiveness | Stability (Pop/Cap)
    
    let r = &state.resources;
    let start_x = 300.0;
    let spacing = 180.0;
    
    draw_resource_item("Materials", r.materials, start_x, 35.0, GREEN);
    draw_resource_item("Maint.", r.maintenance, start_x + spacing, 35.0, ORANGE);
    draw_resource_item("Attr.", r.attractiveness, start_x + spacing * 2.0, 35.0, PINK);
    draw_resource_item("Stab.", r.stability, start_x + spacing * 3.0, 35.0, SKYBLUE);
    
    // Pop/Cap
    let cap = state.calculate_housing_capacity();
    let pop_text = format!("Pop: {:.0}/{:.0}", state.population.value(), cap);
    draw_text(&pop_text, start_x + spacing * 4.0, 35.0, 20.0, PURPLE);
}

fn draw_resource_item(label: &str, value: f32, x: f32, y: f32, color: Color) {
    draw_text(&format!("{}: {:.1}", label, value), x, y, 20.0, color);
}
