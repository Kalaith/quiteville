//! UI module - "Dumb" UI rendering and input handling
//! 
//! UI reads state and emits actions. It does not modify state directly.

use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;

pub mod resources;
pub mod zones;
pub mod layout;

/// Constants for UI Layout
pub mod colors {
    use macroquad::prelude::*;
    pub const BACKGROUND: Color = Color::new(0.1, 0.1, 0.12, 1.0);
    pub const PANEL_BG: Color = Color::new(0.15, 0.15, 0.18, 1.0);
    pub const TEXT: Color = Color::new(0.9, 0.9, 0.9, 1.0);
    pub const ACCENT: Color = Color::new(0.4, 0.8, 0.4, 1.0); // Greenish
    pub const WARN: Color = Color::new(0.9, 0.6, 0.2, 1.0);
    pub const ERROR: Color = Color::new(0.9, 0.3, 0.3, 1.0);
}

/// Draw the entire game UI and return any player action triggered
pub fn draw_game_ui(state: &GameState, time_scale: f32, paused: bool) -> Option<PlayerAction> {
    clear_background(colors::BACKGROUND);
    
    // 1. Top Bar (Resources & Time)
    resources::draw_top_bar(state, time_scale, paused);
    
    // 2. Main Content Area (Layout)
    // Left: Stats/Log. Right: Zones.
    let action = layout::draw_main_layout(state);
    
    action
}
