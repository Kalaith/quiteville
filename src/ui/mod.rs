//! UI module - "Dumb" UI rendering and input handling
//! 
//! UI reads state and emits actions. It does not modify state directly.

use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;

pub mod resources;
pub mod zones;
pub mod layout;
pub mod map_renderer;
pub mod tech;
pub mod text_util;
pub mod tooltip;
pub mod floating_text;
pub mod region_ui;
pub mod chronicle_ui;
pub mod particles;
pub mod theme;
pub mod dialog_ui;

/// Draw the entire game UI and return any player action triggered
pub fn draw_game_ui(state: &GameState, time_scale: f32, paused: bool) -> Option<PlayerAction> {

    
    // 1. Top Bar (Resources & Time)
    resources::draw_top_bar(state, time_scale, paused);
    
    // 2. Main Content Area (Layout)
    let action = layout::draw_main_layout(state);
    
    // 3. Tech Tree Modal
    if state.show_tech_tree {
        // Draw centered modal
        let w = screen_width() * 0.8;
        let h = screen_height() * 0.8;
        let x = (screen_width() - w) / 2.0;
        let y = (screen_height() - h) / 2.0;
        
        if let Some(tech_act) = tech::draw_tech_tree_window(state, x, y, w, h) {
            return Some(tech_act);
        }
    }

    // 4. Chronicle Modal
    if state.show_chronicle {
        // Draw centered modal
        let w = screen_width() * 0.9;
        let h = screen_height() * 0.9;
        let x = (screen_width() - w) / 2.0;
        let y = (screen_height() - h) / 2.0;
        
        if let Some(act) = chronicle_ui::draw_chronicle_ui(state, x, y, w, h) {
            return Some(act);
        }
    }

    // 5. Guide Dialog (Overlay)
    if let Some(act) = dialog_ui::draw_guide_dialog(state) {
        return Some(act);
    }
    
    action
}
