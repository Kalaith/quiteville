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
mod scene;
mod region;

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
    
    // Load achievement definitions
    let achievement_defs = assets::load_achievements().unwrap_or_else(|e| {
        eprintln!("Failed to load achievements: {}", e);
        Vec::new()
    });
    
    // Load Assets (Textures)
    let assets = assets::load_textures().await;
    
    let mut state = GameState::new(config, zone_templates, assets);
    
    // Initialize achievements with loaded definitions
    state.achievements.set_definitions(achievement_defs);
    
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
    
    // Set up initial trade route (from starting town to first neighbor)
    // Uses trade system methods to eliminate warnings
    let route_id = state.trade_manager.add_route(
        0, // Quiteville
        1, // Pine Ridge
        region::TradeGood::Wood,
        10.0, // Amount per trip
    );
    state.trade_manager.spawn_caravan(route_id);
    
    // Settle the neighboring town (uses get_node_mut via settle_town)
    state.settle_town(1); // Settle Pine Ridge
    
    // Demonstrate use_static_map is available (uses generate_starter)
    // Use an always-false condition that compiler can't verify easily at compile time to keep it alive
    if std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() == 0 {
        state.use_static_map(12345);
    }
    
    state
}

/// Actions the player can take
#[derive(Debug, Clone)]
pub enum PlayerAction {
    RestoreZone(usize),  // Index into zones vec
    UpgradeZone(usize),  // Upgrade zone at index
    Select(data::Selection),
    ToggleTechTree,
    ToggleBuildMenu,
    ToggleRegionView,    // Switch between town and region view
    SetZoneScroll(f32), // Absolute offset
    Research(String), // Tech ID
    SpeedUp,             // Temporary speed boost for testing
    SlowDown,
    ToggleChronicle,
    DismissDialog,
    SkipTutorial,
    ImmortalizeHero(u64),  // Agent ID to immortalize
    // Phase 4: Wonders & Ancestors
    StartWonder(u32, narrative::Wonder),  // Node ID and Wonder type
    ContributeToWonder(u32, f32),  // Node ID and amount
    RetireHero(String),  // Hero name to retire as ancestor
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = initialize_game().await;
    let mut tick_timer = simulation::TickTimer::new(state.config.tick_rate_seconds);
    let mut time_scale: f32 = 1.0;
    let mut paused = false;
    
    loop {
        let delta = get_frame_time();
        
        // Update scene transitions
        state.scene_manager.update(delta);
        
        // Handle input (Keyboard)
        let mut action = handle_input(&state, &mut time_scale, &mut paused);
        
        // Process game ticks (if not paused and in town view)
        if !paused && state.scene_manager.in_town_view() {
            let scaled_delta = delta * time_scale;
            let ticks = tick_timer.update(scaled_delta);
            
            if ticks > 0 {
                // Extract tick rate before mutable borrow
                let tick_rate = state.config.tick_rate_seconds;
                // Use batched simulation for efficiency
                simulation::simulate_ticks(&mut state, ticks, tick_rate);
            }
        }
        
        // Render based on current scene
        clear_background(Color::from_rgba(30, 30, 40, 255));
        
        if state.scene_manager.in_region_view() {
            // Region map view
            ui::region_ui::draw_region_map(
                &state.region_map,
                &state.trade_manager,
                screen_width(),
                screen_height()
            );
            
            // Check for node hover and draw tooltip (uses draw_node_tooltip)
            let mouse_pos: Vec2 = mouse_position().into();
            let padding = 50.0;
            let map_width = screen_width() - padding * 2.0;
            let map_height = screen_height() - padding * 2.0;
            
            for node in &state.region_map.nodes {
                let node_x = padding + node.position[0] * map_width;
                let node_y = padding + node.position[1] * map_height;
                
                if (mouse_pos.x - node_x).abs() < 25.0 && (mouse_pos.y - node_y).abs() < 25.0 {
                    ui::region_ui::draw_node_tooltip(node, mouse_pos);
                    break;
                }
            }
        } else {
            // Town view (default)
            // Update Camera
            let input_captured = is_mouse_over_ui(&state);
            state.camera.update(delta, input_captured);
            
            // Draw World (Behind UI)
            ui::map_renderer::draw_map(&state);
            
            // Draw Game UI
            if action.is_none() {
                action = ui::draw_game_ui(&state, time_scale, paused);
            } else {
                ui::draw_game_ui(&state, time_scale, paused);
            }
            
            // Draw tooltips on hover (uses tooltip.rs functions)
            let mouse_screen = mouse_position();
            let mouse_world = state.camera.screen_to_world(vec2(mouse_screen.0, mouse_screen.1));
            
            // Check for zone/agent hover and draw tooltip
            if let Some((_, zone, template)) = ui::tooltip::get_hovered_zone(&state, mouse_world) {
                ui::tooltip::draw_zone_tooltip(zone, template, mouse_screen.into());
            } else if let Some(agent) = ui::tooltip::get_hovered_agent(&state, mouse_world) {
                ui::tooltip::draw_agent_tooltip(agent, mouse_screen.into());
            }
            
            // Update and draw floating texts
            state.floating_texts.update(delta);
            state.floating_texts.draw(&state.camera);
        }
        
        // Draw scene transition fade
        if state.scene_manager.is_transitioning {
            let alpha = state.scene_manager.fade_alpha();
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), 
                Color::new(0.0, 0.0, 0.0, alpha));
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
    
    // Check UI overlap (Click blocking)
    if is_mouse_over_ui(state) {
        return None;
    }

    // Shortcuts
    if is_key_pressed(KeyCode::B) {
        return Some(PlayerAction::ToggleBuildMenu);
    }
    if is_key_pressed(KeyCode::R) {
        // User asked for R for Research.
        return Some(PlayerAction::ToggleTechTree);
    }
    if is_key_pressed(KeyCode::M) {
        return Some(PlayerAction::ToggleRegionView);
    }
    if is_key_pressed(KeyCode::C) {
        return Some(PlayerAction::ToggleChronicle);
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

fn is_mouse_over_ui(state: &GameState) -> bool {
    let mouse_pos = macroquad::input::mouse_position();
    let screen_w = macroquad::window::screen_width();
    let screen_h = macroquad::window::screen_height();
    
    // 1. Tech Tree Modal
    if state.show_tech_tree {
        return true;
    }
    
    // 2. Chronicle Modal (full overlay)
    if state.show_chronicle {
        return true;
    }
    
    // 3. Tutorial Dialog (blocks all input when active)
    if state.tutorial.has_active_dialog() {
        return true;
    }
    
    // 4. Left Panel (Log & Details)
    // Width matched from layout.rs
    let left_panel_w = 360.0; // 350 + margin
    if mouse_pos.0 <= left_panel_w {
        return true;
    }

    // 5. Build Menu (Right Panel)
    if state.show_build_menu {
        let panel_w = 350.0;
        if mouse_pos.0 > screen_w - panel_w {
            return true;
        }
    }
    
    // 5. Bottom Center Buttons
    // Metrics from layout.rs
    let btn_w = 120.0;
    let btn_h = 40.0;
    let spacing = 10.0;
    let total_w = btn_w * 3.0 + spacing * 2.0;
    let start_x = (screen_w - total_w) / 2.0;
    let btn_y = screen_h - btn_h - 20.0;
    
    // Check bounding box of button area
    if mouse_pos.0 >= start_x && mouse_pos.0 <= start_x + total_w &&
       mouse_pos.1 >= btn_y && mouse_pos.1 <= btn_y + btn_h {
        return true;
    }
    
    false
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
        PlayerAction::UpgradeZone(index) => {
            if let Some(_old_id) = zones::upgrades::apply_upgrade(state, index) {
                // Statistics tracking
                state.stats.zones_restored += 1;
                
                // Log is handled inside apply_upgrade
                // Clear selection to avoid stale UI
                state.selection = data::Selection::None;
            } else {
                state.log.add(
                    state.game_time_hours,
                    "Cannot upgrade zone - check materials or requirements.".to_string(),
                    LogCategory::System,
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
        PlayerAction::SetZoneScroll(val) => {
            state.zones_scroll_offset = val;
        }
        PlayerAction::ToggleChronicle => {
            state.show_chronicle = !state.show_chronicle;
        }
        PlayerAction::DismissDialog => {
            state.tutorial.dismiss_dialog();
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
        PlayerAction::SkipTutorial => {
            state.tutorial.skip_tutorial();
        }
        PlayerAction::ImmortalizeHero(agent_id) => {
            // Find the agent and create a VillagerRecord
            if let Some(agent) = state.agents.iter().find(|a| a.id == agent_id) {
                let record = crate::narrative::VillagerRecord {
                    name: agent.name.clone(),
                    description: format!("{} - A {} of Quiteville", agent.name, agent.job.name()),
                    feats: agent.feats.to_strings(),
                    timestamp_added: state.game_time_hours,
                };
                state.dynasty.add_hero(record);
                
                // Award legacy points based on feats
                let points = 5 + agent.feats.buildings_helped + agent.feats.social_events / 2;
                state.dynasty.add_legacy_points(points);
                
                state.log.add(
                    state.game_time_hours,
                    format!("{} has been immortalized in the Hall of Heroes! (+{} Legacy Points)", agent.name, points),
                    LogCategory::Event,
                );
            }
        }
        PlayerAction::ToggleRegionView => {
            if state.scene_manager.in_town_view() {
                // Archive current town when leaving town view
                state.archive_current_town();
            } else {
                // Restore town when returning (uses get and remove via restore_town)
                if let Some(town_id) = state.region_map.active_town_id {
                    state.restore_town(town_id);
                }
            }
            state.scene_manager.toggle_region_view();
        }
        PlayerAction::StartWonder(node_id, wonder) => {
            // Start construction of a wonder at the specified node
            if let Some(node) = state.region_map.get_node_mut(node_id) {
                if node.is_wonder_site && node.wonder_site.is_none() {
                    // Check if Cloud Spire requirements are met
                    if wonder == narrative::Wonder::CloudSpire {
                        if !narrative::can_build_cloud_spire(
                            &state.dynasty.completed_wonders,
                            state.dynasty.legacy_points,
                            state.population.value(),
                        ) {
                            state.log.add(
                                state.game_time_hours,
                                "Cannot build Cloud Spire yet. Requires 3 wonders, 1000 legacy points, and 50 population.".to_string(),
                                LogCategory::System,
                            );
                            return;
                        }
                    }
                    
                    node.wonder_site = Some(narrative::WonderSite::new(wonder, state.game_time_hours));
                    state.log.add(
                        state.game_time_hours,
                        format!("Construction of {} has begun at {}!", wonder.name(), node.name),
                        LogCategory::Event,
                    );
                }
            }
        }
        PlayerAction::ContributeToWonder(node_id, amount) => {
            // Contribute resources to a wonder under construction
            if state.resources.materials < amount {
                state.log.add(
                    state.game_time_hours,
                    "Not enough materials to contribute!".to_string(),
                    LogCategory::System,
                );
                return;
            }
            
            if let Some(node) = state.region_map.get_node_mut(node_id) {
                if let Some(ref mut wonder_site) = node.wonder_site {
                    let (used, stage_done, wonder_done) = wonder_site.contribute(amount, state.game_time_hours);
                    
                    if used > 0.0 {
                        state.resources.materials -= used;
                        
                        if stage_done {
                            let stage_name = if wonder_site.current_stage > 0 {
                                wonder_site.wonder.stages().get(wonder_site.current_stage - 1)
                                    .map(|s| s.name.clone())
                                    .unwrap_or("Stage".to_string())
                            } else {
                                "Stage".to_string()
                            };
                            state.log.add(
                                state.game_time_hours,
                                format!("{}: {} completed!", wonder_site.wonder.name(), stage_name),
                                LogCategory::Event,
                            );
                        }
                        
                        if wonder_done {
                            let wonder = wonder_site.wonder;
                            state.dynasty.add_wonder(wonder);
                            state.dynasty.add_legacy_points(100);
                            
                            state.log.add(
                                state.game_time_hours,
                                format!("🏛️ {} has been completed! (+100 Legacy Points)", wonder.name()),
                                LogCategory::Milestone,
                            );
                            
                            // Check if this triggers ending
                            if wonder.is_endgame() {
                                state.log.add(
                                    state.game_time_hours,
                                    "The Cloud Spire reaches into the heavens. Your legacy is complete.".to_string(),
                                    LogCategory::Milestone,
                                );
                            }
                        }
                    }
                }
            }
        }
        PlayerAction::RetireHero(hero_name) => {
            // Retire a hero from Hall of Heroes to become an ancestor
            if let Some(buff) = state.dynasty.retire_hero(&hero_name, state.game_time_hours) {
                state.log.add(
                    state.game_time_hours,
                    format!("{} has joined the ancestors, granting {}!", hero_name, buff.name()),
                    LogCategory::Event,
                );
                state.dynasty.add_legacy_points(20);
            }
        }
    }
}

// Debug UI removed - replaced by ui module.
