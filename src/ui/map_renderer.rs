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
    
    // 2. Draw Buildings (Active Zones)
    for zone in &state.zones {
        if zone.dormant { continue; }
        
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
                
                // Map template ID to texture name
                let tex_name = match template.id.as_str() {
                    "old_homestead" => "building_homestead_large",
                    "old_well" => "building_well_large",
                    "village_green" => "building_village_green_large",
                    "community_market" => "building_market_large",
                    "scavengers_workshop" => "building_workshop_large",
                    "community_farm" => "building_farm_large",
                    _ => "tile_ruins", // Fallback
                };
                
                if let Some(tex) = state.assets.get(tex_name) {
                    draw_texture_ex(
                        tex,
                        screen_pos.x,
                        screen_pos.y,
                        WHITE, // Could tint based on condition?
                        DrawTextureParams {
                            dest_size: Some(vec2(width, height)),
                            ..Default::default()
                        }
                    );
                } else {
                    // Fallback box
                    draw_rectangle(screen_pos.x, screen_pos.y, width, height, BROWN);
                }
            }
        }
    }
    
    // 3. Draw Agents
    draw_agents(state, camera);
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
