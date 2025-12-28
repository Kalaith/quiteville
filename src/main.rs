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
async fn initialize_game() -> GameState {
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
    
    // Load Assets (Textures)
    let assets = assets::load_textures().await;
    
    let mut state = GameState::new(config, zone_templates, milestones, assets);
    
    // Set initial camera target so map (0,0) is at top-left of screen
    state.camera.target = vec2(screen_width() / 2.0, screen_height() / 2.0);
    
    // Initialize Map with Zones
    // Can't iterate state.zone_templates directly while borrowing state mutably?
    // Actually we can iterate state.zone_templates since we only need read access to templates,
    // and write access to map.
    for (_i, template) in state.zone_templates.iter().enumerate() {
        if let Some(rect) = template.map_rect {
            // Found a zone with map coords!
            // Set it to Ruins by default
            state.world_map.set_rect(
                rect.x, rect.y, rect.w, rect.h, 
                simulation::map::TileType::Ruins, 
                // We need the ID of the zone INSTANCE, not the template index.
                // But wait, the instances are created below.
                // The zone instance ID should match the index in state.zones.
                // But zones might not be created yet.
                // For MVP fixed map, let's assume 1 instance per template for unique ones.
                None // Will link zone_id when instance is added
            );
        }
    }
    
    // Add all starting zones (all start DORMANT - player must restore them)
    // AND link them to the map
    let zones_to_add = [
        "old_homestead", 
        "village_green", 
        "old_well",
        "community_market",
        "scavengers_workshop",
        "community_farm"
    ];
    
    for template_id in zones_to_add {
        // Find index of added zone
        let zone_idx = state.zones.len(); 
        state.add_zone(template_id);
        
        // Link map tiles to this new zone instance
        if let Some(template) = state.zone_templates.iter().find(|t| t.id == template_id) {
            if let Some(rect) = template.map_rect {
                state.world_map.set_rect(
                    rect.x, rect.y, rect.w, rect.h, 
                    simulation::map::TileType::Ruins, 
                    Some(zone_idx)
                );
            }
        }
    }
    
    // Add welcome log entry
    state.log.add(
        0.0,
        "Six abandoned sites await restoration. Press [1-6] to begin repairs.".to_string(),
        LogCategory::System,
    );
    
    state
}

/// Actions the player can take
#[derive(Debug, Clone)]
pub enum PlayerAction {
    RestoreZone(usize),  // Index into zones vec
    Select(data::Selection),
    ToggleTechTree,
    ToggleBuildMenu,
    ScrollZones(f32), // Delta
    Research(String), // Tech ID
    SpeedUp,             // Temporary speed boost for testing
    SlowDown,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = initialize_game().await;
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
        
        // Update Camera
        let mouse_pos = mouse_position();
        let right_panel_w = 350.0; // From layout.rs
        let is_over_build_menu = state.show_build_menu && mouse_pos.0 > screen_width() - right_panel_w;
        
        // Input is captured if Tech Tree is open (Modal) OR hovering Build Menu
        let input_captured = state.show_tech_tree || is_over_build_menu;
        
        state.camera.update(delta, input_captured);
        
        // Draw World (Behind UI)
        ui::map_renderer::draw_map(&state);
        
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
    
    // Check UI overlap
    let mouse_pos = mouse_position();
    let right_panel_w = 350.0;
    let is_over_build_menu = state.show_build_menu && mouse_pos.0 > screen_width() - right_panel_w;
    
    // If Tech Tree is open OR Mouse over Build Menu, block map interaction (clicks)
    if state.show_tech_tree || is_over_build_menu {
        return None;
    }

    // Restore zone with R key
    if is_key_pressed(KeyCode::R) {
        if !state.zones.is_empty() {
            return Some(PlayerAction::RestoreZone(0));
        }
    }
    
    // Number keys to restore specific zones
    for (i, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5, KeyCode::Key6].iter().enumerate() {
        if is_key_pressed(*key) && i < state.zones.len() {
            return Some(PlayerAction::RestoreZone(i));
        }
    }
    
    // Mouse Click Selection
    if is_mouse_button_released(MouseButton::Left) {
        let mouse_pos: Vec2 = mouse_position().into();
        let was_click = if let Some(start) = state.camera.drag_start {
            start.distance(mouse_pos) < 5.0
        } else {
            true
        };
        
        if was_click {
            let world_pos = state.camera.screen_to_world(mouse_pos);
            
            // 1. Check Agents (Top layer)
            if let Some(agent) = state.agents.iter().find(|a| a.pos.distance(world_pos) < 20.0) {
                return Some(PlayerAction::Select(data::Selection::Agent(agent.id)));
            }
            
            // 2. Check Zones (Tile layer)
            let tile_x = (world_pos.x / ui::map_renderer::TILE_SIZE).floor() as i32;
            let tile_y = (world_pos.y / ui::map_renderer::TILE_SIZE).floor() as i32;
            
            if tile_x >= 0 && tile_y >= 0 {
                if let Some(tile) = state.world_map.get_tile(tile_x as usize, tile_y as usize) {
                    if let Some(zone_id) = tile.zone_id {
                        return Some(PlayerAction::Select(data::Selection::Zone(zone_id)));
                    }
                }
            }
            
            return Some(PlayerAction::Select(data::Selection::None));
        }
    }
    
    None
}

/// Apply a player action to the game state
fn apply_action(state: &mut GameState, action: PlayerAction) {
    match action {
        PlayerAction::RestoreZone(index) => {
            // Get cost from template
            let mut cost = 1.0;
            let mut zone_name = "Unknown Zone".to_string();
            
            if let Some(zone) = state.zones.get(index) {
                if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
                    cost = template.construction_cost;
                    zone_name = template.name.clone();
                }
            }
            
            // Check if we have enough materials
            if state.resources.materials < cost {
                state.log.add(
                    state.game_time_hours,
                    format!("Not enough materials for {}! Need {:.1}", zone_name, cost),
                    LogCategory::System,
                );
                return;
            }
            
            if let Some(zone) = state.zones.get_mut(index) {
                // Check if already at max condition
                if zone.condition >= 1.0 {
                    state.log.add(
                        state.game_time_hours,
                        format!("{} is already fully restored.", zone_name),
                        LogCategory::System,
                    );
                    return;
                }
                
                // Deduct cost
                state.resources.materials -= cost;
                
                let old_condition = zone.condition;
                zone.restore(0.5); // Restore 50% condition
                
                state.log.add(
                    state.game_time_hours,
                    format!(
                        "Restored {} (-{:.1} Mat): {:.0}% → {:.0}%",
                        zone_name,
                        cost,
                        old_condition * 100.0,
                        zone.condition * 100.0
                    ),
                    LogCategory::Zone,
                );
            }
        }
        PlayerAction::Select(sel) => {
            state.selection = sel;
        }
        PlayerAction::ToggleTechTree => {
            state.show_tech_tree = !state.show_tech_tree;
        }
        PlayerAction::ToggleBuildMenu => {
            state.show_build_menu = !state.show_build_menu;
        }
        PlayerAction::ScrollZones(delta) => {
            state.zones_scroll_offset = (state.zones_scroll_offset + delta).max(0.0);
            // Optional: Clamp max if we knew content height?
            // For now just max(0.0) is enough to prevent negative scroll.
        }
        PlayerAction::Research(id) => {
            // Find index
            if let Some(pos) = state.tech_tree.iter().position(|t| t.id == id) {
                let cost = state.tech_tree[pos].cost;
                if state.resources.materials >= cost {
                    // Purchase
                    state.resources.materials -= cost;
                    state.tech_tree[pos].unlocked = true;
                    state.log.add(
                        state.game_time_hours,
                        format!("Researched: {}", state.tech_tree[pos].name),
                        LogCategory::System
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
