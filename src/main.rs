//! Quiteville - An Idle Town Builder
//! 
//! A relaxing idle town builder about reviving a small town that grows when you're not watching.

mod assets;
mod data;
mod economy;
mod zones;
mod population;
mod simulation;
mod narrative;
mod city;
mod ui;
mod save;

use macroquad::prelude::*;
use data::GameState;
use narrative::LogCategory;

fn window_conf() -> Conf {
    Conf {
        window_title: "Quiteville".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

/// Load all game data and create initial state
fn initialize_game() -> GameState {
    // Load config from embedded JSON
    let config = assets::load_config().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}", e);
        data::GameConfig::default()
    });
    
    // Load zone templates
    let zone_templates = assets::load_zones().unwrap_or_else(|e| {
        eprintln!("Failed to load zones: {}", e);
        Vec::new()
    });
    
    // Load milestones
    let milestones = assets::load_milestones().unwrap_or_else(|e| {
        eprintln!("Failed to load milestones: {}", e);
        Vec::new()
    });
    
    let mut state = GameState::new(config, zone_templates, milestones);
    
    // Add all starting zones (all start DORMANT - player must restore them)
    state.add_zone("old_homestead");
    state.add_zone("village_green");
    state.add_zone("old_well");
    
    // Add welcome log entry
    state.log.add(
        0.0,
        "Three abandoned sites await restoration. Press [1-3] to begin repairs.".to_string(),
        LogCategory::System,
    );
    
    state
}

/// Actions the player can take
#[derive(Debug, Clone)]
pub enum PlayerAction {
    RestoreZone(usize),  // Index into zones vec
    SpeedUp,             // Temporary speed boost for testing
    SlowDown,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = initialize_game();
    let mut tick_timer = simulation::TickTimer::new(state.config.tick_rate_seconds);
    let mut time_scale: f32 = 1.0;
    let mut paused = false;
    
    loop {
        let delta = get_frame_time();
        
        // Handle input (Keyboard)
        let mut action = handle_input(&state, &mut time_scale, &mut paused);
        
        // Process game ticks (if not paused)
        if !paused {
            let scaled_delta = delta * time_scale;
            let ticks = tick_timer.update(scaled_delta);
            
            if ticks > 0 {
                // Extract tick rate before mutable borrow
                let tick_rate = state.config.tick_rate_seconds;
                // Use batched simulation for efficiency
                simulation::simulate_ticks(&mut state, ticks, tick_rate);
            }
        }
        
        // Render UI (and get UI actions)
        // Background color
        clear_background(Color::from_rgba(30, 30, 40, 255));
        
        // Draw Game UI
        // If no keyboard action, check UI action
        if action.is_none() {
            action = ui::draw_game_ui(&state, time_scale, paused);
        } else {
            // Still draw UI even if we have keyboard action
            ui::draw_game_ui(&state, time_scale, paused);
        }
        
        // Apply action if any
        if let Some(act) = action {
            apply_action(&mut state, act);
        }
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

/// Handle player input, returns action if any
fn handle_input(state: &GameState, time_scale: &mut f32, paused: &mut bool) -> Option<PlayerAction> {
    // Pause toggle
    if is_key_pressed(KeyCode::Space) {
        *paused = !*paused;
    }
    
    // Time scale controls
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Equal) {
        *time_scale = (*time_scale * 2.0).min(64.0);
        return Some(PlayerAction::SpeedUp);
    }
    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Minus) {
        *time_scale = (*time_scale / 2.0).max(0.25);
        return Some(PlayerAction::SlowDown);
    }
    
    // Restore zone with R key
    if is_key_pressed(KeyCode::R) {
        if !state.zones.is_empty() {
            return Some(PlayerAction::RestoreZone(0));
        }
    }
    
    // Number keys to restore specific zones
    for (i, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3].iter().enumerate() {
        if is_key_pressed(*key) && i < state.zones.len() {
            return Some(PlayerAction::RestoreZone(i));
        }
    }
    
    None
}

/// Cost to restore a zone (in Materials)
const RESTORE_COST: f32 = 0.5;

/// Apply a player action to the game state
fn apply_action(state: &mut GameState, action: PlayerAction) {
    match action {
        PlayerAction::RestoreZone(index) => {
            // Check if we have enough materials
            if state.resources.materials < RESTORE_COST {
                state.log.add(
                    state.game_time_hours,
                    format!("Not enough materials! Need {:.1}", RESTORE_COST),
                    LogCategory::System,
                );
                return;
            }
            
            if let Some(zone) = state.zones.get_mut(index) {
                // Check if already at max condition
                if zone.condition >= 1.0 {
                    state.log.add(
                        state.game_time_hours,
                        "Zone is already fully restored.".to_string(),
                        LogCategory::System,
                    );
                    return;
                }
                
                // Deduct cost
                state.resources.materials -= RESTORE_COST;
                
                let old_condition = zone.condition;
                zone.restore(0.5); // Restore 50% condition (Buffed for tranquility)
                
                if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
                    state.log.add(
                        state.game_time_hours,
                        format!(
                            "Restored {} (-{:.1} Mat): {:.0}% → {:.0}%",
                            template.name,
                            RESTORE_COST,
                            old_condition * 100.0,
                            zone.condition * 100.0
                        ),
                        LogCategory::Zone,
                    );
                }
            }
        }
        PlayerAction::SpeedUp | PlayerAction::SlowDown => {
            // Time scale changes are handled in input, no state change needed
        }
    }
}

// Debug UI removed - replaced by ui module.
