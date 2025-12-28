use macroquad::prelude::*;
use crate::data::GameState;
use crate::simulation::map::TileType;
use crate::simulation::camera::Camera2D;

pub const TILE_SIZE: f32 = 32.0;

/// Draw the world map and agents
pub fn draw_map(state: &GameState) {
    let map = &state.world_map;
    let camera = &state.camera;
    
    // 1. Draw Ground Tiles
    let start_x = 0;
    let end_x = map.width;
    let start_y = 0;
    let end_y = map.height;
    
    for y in start_y..end_y {
        for x in start_x..end_x {
            if let Some(tile) = map.get_tile(x, y) {
                let world_pos = vec2(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
                let screen_pos = camera.world_to_screen(world_pos);
                let size = TILE_SIZE * camera.zoom;
                
                // Culling
                if screen_pos.x + size < 0.0 || screen_pos.x > screen_width() ||
                   screen_pos.y + size < 0.0 || screen_pos.y > screen_height() {
                    continue;
                }
                
                // Determine Texture
                let tex_name = match tile.kind {
                    TileType::Grass => "tile_grass",
                    TileType::Dirt => "tile_dirt",
                    TileType::Water => "tile_water",
                    TileType::Floor => "tile_floor",
                    TileType::Wall => "tile_wall",
                    TileType::Ruins => "tile_ruins",
                };
                
                // If it's a zone and dormant, force ruins appearance on ground
                let is_dormant_zone = if let Some(z_idx) = tile.zone_id {
                    state.zones.get(z_idx).map(|z| z.dormant).unwrap_or(false)
                } else {
                    false
                };
                
                let final_tex = if is_dormant_zone { "tile_ruins" } else { tex_name };
                
                if let Some(tex) = state.assets.get(final_tex) {
                    draw_texture_ex(
                        tex,
                        screen_pos.x,
                        screen_pos.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(size, size)),
                            ..Default::default()
                        }
                    );
                } else {
                    // Fallback color
                    draw_rectangle(screen_pos.x, screen_pos.y, size, size, GRAY);
                }
                
                // Grid lines (faint)
                if camera.zoom > 0.8 {
                    draw_rectangle_lines(screen_pos.x, screen_pos.y, size, size, 1.0, Color::new(0.0, 0.0, 0.0, 0.1));
                }
            }
        }
    }
    
    // 2. Draw Buildings (Active Zones and Under Construction)
    for (zone_idx, zone) in state.zones.iter().enumerate() {
        // Skip dormant zones that aren't under construction
        if zone.dormant && !zone.is_under_construction() { continue; }
        
        if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
            if let Some(rect) = template.map_rect {
                let world_pos = vec2(rect.x as f32 * TILE_SIZE, rect.y as f32 * TILE_SIZE);
                let screen_pos = camera.world_to_screen(world_pos);
                
                // Calculate size based on map_rect dimensions matches the texture aspect ratio
                let width = rect.w as f32 * TILE_SIZE * camera.zoom;
                let height = rect.h as f32 * TILE_SIZE * camera.zoom;
                
                // Culling (Rough)
                if screen_pos.x + width < 0.0 || screen_pos.x > screen_width() ||
                   screen_pos.y + height < 0.0 || screen_pos.y > screen_height() {
                    continue;
                }
                
                // Check if zone is under construction
                let is_under_construction = zone.is_under_construction();
                
                // Map template ID to texture name
                let tex_name = match template.id.as_str() {
                    "old_homestead" => "building_homestead_large",
                    "old_well" => "building_well_large",
                    "village_green" => "building_village_green_large",
                    "community_market" => "building_market_large",
                    "scavengers_workshop" => "building_workshop_large",
                    "community_farm" => "building_farm_large",
                    "tent" => "building_tent_large",
                    "shack" => "building_shack_large",
                    "cottage" => "building_cottage_large",
                    "campfire" => "building_campfire_large",
                    "outhouse" => "building_outhouse_large",
                    "market_stall" => "building_stall_large",
                    "woodcutters_block" => "building_woodcutter_large",
                    "stone_quarry" => "building_quarry_large",
                    _ => "tile_ruins", // Fallback
                };
                
                // Tint for construction or normal
                let tint = if is_under_construction {
                    Color::new(0.6, 0.6, 0.8, 0.7) // Blue-ish transparent for construction
                } else {
                    WHITE
                };
                
                if let Some(tex) = state.assets.get(tex_name) {
                    draw_texture_ex(
                        tex,
                        screen_pos.x,
                        screen_pos.y,
                        tint,
                        DrawTextureParams {
                            dest_size: Some(vec2(width, height)),
                            ..Default::default()
                        }
                    );
                } else {
                    // Fallback box
                    let color = if is_under_construction { BLUE } else { BROWN };
                    draw_rectangle(screen_pos.x, screen_pos.y, width, height, color);
                }
                
                // Draw construction progress bar for zones under construction
                if is_under_construction {
                    let progress = zone.construction_progress(template.construction_work);
                    let bar_width = width;
                    let bar_height = 8.0 * camera.zoom;
                    let bar_y = screen_pos.y + height + 2.0 * camera.zoom;
                    
                    // Background bar
                    draw_rectangle(screen_pos.x, bar_y, bar_width, bar_height, DARKGRAY);
                    // Progress fill
                    draw_rectangle(screen_pos.x, bar_y, bar_width * progress, bar_height, YELLOW);
                    // Border
                    draw_rectangle_lines(screen_pos.x, bar_y, bar_width, bar_height, 1.0, BLACK);
                    
                    // "Under Construction" text
                    if camera.zoom > 0.5 {
                        let text = format!("{:.0}%", progress * 100.0);
                        draw_text(&text, screen_pos.x + width / 2.0 - 15.0, bar_y - 2.0, 14.0 * camera.zoom, WHITE);
                    }
                }
                
                // Highlight selected zone
                if matches!(state.selection, crate::data::Selection::Zone(idx) if idx == zone_idx) {
                    draw_rectangle_lines(screen_pos.x - 2.0, screen_pos.y - 2.0, width + 4.0, height + 4.0, 3.0, GOLD);
                }
            }
        }
    }
    
    // 3. Draw Agents
    draw_agents(state, camera);
    
    // 4. Draw Seasonal Overlay
    let season_tint = state.season_state.season.color_tint();
    if season_tint[3] > 0.01 {
        draw_rectangle(
            0.0, 0.0,
            screen_width(), screen_height(),
            Color::new(season_tint[0], season_tint[1], season_tint[2], season_tint[3])
        );
    }
    
    // 5. Draw Day/Night Overlay
    draw_day_night_overlay(state.game_hour);
    
    // 6. Draw Weather Effects
    draw_weather_effects(&state.season_state);
    
    // 7. Draw Season/Weather HUD
    draw_season_hud(state);
}

/// Draw weather particle effects
fn draw_weather_effects(season_state: &crate::simulation::seasons::SeasonState) {
    use crate::simulation::seasons::Weather;
    
    let (color, count, speed) = match season_state.weather {
        Weather::Rain => (Color::new(0.6, 0.6, 1.0, 0.3), 100, 400.0),
        Weather::Storm => (Color::new(0.4, 0.4, 0.8, 0.4), 150, 600.0),
        Weather::Snow => (Color::new(1.0, 1.0, 1.0, 0.5), 80, 60.0),
        _ => return,
    };
    
    // Simple particle simulation using time as seed
    let time = macroquad::time::get_time() as f32;
    for i in 0..count {
        let seed = (i as f32) * 7.13;
        let x = ((seed * 123.456 + time * 50.0) % screen_width()).abs();
        let y = ((seed * 789.012 + time * speed) % screen_height()).abs();
        
        match season_state.weather {
            Weather::Snow => {
                draw_circle(x, y, 2.0, color);
            },
            _ => {
                // Rain drops
                draw_line(x, y, x - 2.0, y + 10.0, 1.0, color);
            }
        }
    }
}

/// Draw season and weather info in corner
fn draw_season_hud(state: &GameState) {
    let text = state.season_state.display_string();
    let x = screen_width() - 200.0;
    let y = 50.0;
    
    // Background
    draw_rectangle(x - 5.0, y - 15.0, 195.0, 25.0, Color::new(0.0, 0.0, 0.0, 0.5));
    draw_text(&text, x, y, 18.0, WHITE);
}

/// Draw a screen-wide tint based on time of day
fn draw_day_night_overlay(game_hour: f32) {
    let h = game_hour % 24.0;
    
    // Calculate tint color based on time
    // Dawn: 5-7, Day: 7-18, Dusk: 18-20, Night: 20-5
    let (r, g, b, a) = if h >= 5.0 && h < 7.0 {
        // Dawn - orange/pink tint fading out
        let t = (h - 5.0) / 2.0; // 0 to 1
        (1.0, 0.8 + t * 0.2, 0.6 + t * 0.4, 0.2 * (1.0 - t))
    } else if h >= 7.0 && h < 18.0 {
        // Day - no overlay
        (1.0, 1.0, 1.0, 0.0)
    } else if h >= 18.0 && h < 20.0 {
        // Dusk - orange/red tint fading in to night
        let t = (h - 18.0) / 2.0; // 0 to 1
        (1.0 - t * 0.3, 0.7 - t * 0.3, 0.4 - t * 0.2, 0.1 + t * 0.2)
    } else {
        // Night - deep blue tint
        (0.1, 0.1, 0.3, 0.4)
    };
    
    if a > 0.01 {
        draw_rectangle(
            0.0, 0.0,
            screen_width(), screen_height(),
            Color::new(r, g, b, a)
        );
    }
}

fn draw_agents(state: &GameState, camera: &Camera2D) {
    for agent in &state.agents {
        let screen_pos = camera.world_to_screen(agent.pos);
        let size = 24.0 * camera.zoom; // Agents slightly larger sprite
        
        // Culling
        if screen_pos.x + size < 0.0 || screen_pos.x > screen_width() ||
           screen_pos.y + size < 0.0 || screen_pos.y > screen_height() {
            continue;
        }
        
        // Agent Body (Texture)
        if let Some(tex) = state.assets.get("agent_villager") {
             // Center the sprite
             let draw_x = screen_pos.x - size/2.0;
             let draw_y = screen_pos.y - size/2.0;
             
             // Tint with agent color
             let color = Color::new(agent.color[0], agent.color[1], agent.color[2], 1.0);
             
             draw_texture_ex(
                tex,
                draw_x,
                draw_y,
                color,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    ..Default::default()
                }
            );
        } else {
            // Fallback Circle
            let color = Color::new(agent.color[0], agent.color[1], agent.color[2], agent.color[3]);
            draw_circle(screen_pos.x, screen_pos.y, size / 2.0, color);
        }
        
        // State Indicators (Thoughts/Emotes) - Using Icons
        let icon_name = match agent.state {
             crate::simulation::agents::AgentState::Shopping { .. } => "icon_thought_shopping",
             crate::simulation::agents::AgentState::Working { .. } => "icon_thought_working",
             crate::simulation::agents::AgentState::Socializing { .. } => "icon_thought_social",
             crate::simulation::agents::AgentState::GoingHome => "icon_thought_sleep",
             _ => "",
        };
        
        if !icon_name.is_empty() {
             if let Some(tex) = state.assets.get(icon_name) {
                 let icon_size = 20.0 * camera.zoom;
                 draw_texture_ex(
                    tex,
                    screen_pos.x, // Offset slightly
                    screen_pos.y - size,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(icon_size, icon_size)),
                        ..Default::default()
                    }
                );
             } else {
                 // Fallback Text if icon missing
                 let text = match agent.state {
                     crate::simulation::agents::AgentState::Shopping { .. } => "$",
                     crate::simulation::agents::AgentState::Working { .. } => "W",
                     crate::simulation::agents::AgentState::Socializing { .. } => "<3",
                     crate::simulation::agents::AgentState::GoingHome => "Zzz",
                     _ => "!",
                 };
                 draw_text(text, screen_pos.x, screen_pos.y - size, 20.0, WHITE);
             }
        }
        
        // Status Indicator (Hungry/Tired?)
        if agent.hunger < 0.3 && icon_name.is_empty() {
            draw_text("!", screen_pos.x, screen_pos.y - size, 20.0, RED);
        }
    }
}
