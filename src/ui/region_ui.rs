//! Region map UI rendering

use macroquad::prelude::*;
use crate::region::{RegionMap, TownNode, TradeManager};

/// Render the region/world map with trade info
pub fn draw_region_map(
    region: &RegionMap, 
    trade_manager: &TradeManager,
    screen_width: f32, 
    screen_height: f32
) {
    // Map dimensions with padding
    let padding = 50.0;
    let map_width = screen_width - padding * 2.0;
    let map_height = screen_height - padding * 2.0;
    
    // Background
    draw_rectangle(0.0, 0.0, screen_width, screen_height, Color::from_rgba(20, 30, 20, 255));
    
    // Draw routes first (below nodes)
    for route in &region.routes {
        let from = region.get_node(route.from);
        let to = region.get_node(route.to);
        
        if let (Some(from), Some(to)) = (from, to) {
            let from_pos = node_to_screen(from, padding, map_width, map_height);
            let to_pos = node_to_screen(to, padding, map_width, map_height);
            
            let color = if route.discovered {
                if route.quality > 0.5 {
                    Color::from_rgba(180, 160, 140, 255) // Paved road
                } else {
                    Color::from_rgba(120, 100, 80, 255) // Dirt road
                }
            } else {
                Color::from_rgba(60, 60, 60, 100) // Undiscovered
            };
            
            let thickness = if route.discovered { 3.0 } else { 1.0 };
            draw_line(from_pos.x, from_pos.y, to_pos.x, to_pos.y, thickness, color);
        }
    }
    
    // Draw caravans on routes (uses Caravan::get_visual_position and TradeGood::name)
    for caravan in &trade_manager.caravans {
        if let Some(route) = trade_manager.routes.iter().find(|r| r.id == caravan.route_id) {
            let from = region.get_node(route.from_town);
            let to = region.get_node(route.to_town);
            
            if let (Some(from_node), Some(to_node)) = (from, to) {
                let from_screen = node_to_screen(from_node, padding, map_width, map_height);
                let to_screen = node_to_screen(to_node, padding, map_width, map_height);
                
                let caravan_pos = caravan.get_visual_position(from_screen, to_screen);
                
                // Draw caravan icon
                draw_circle(caravan_pos.x, caravan_pos.y, 8.0, ORANGE);
                draw_text("🚚", caravan_pos.x - 6.0, caravan_pos.y + 4.0, 12.0, WHITE);
                
                // Show cargo name on hover would go here  
                let _cargo_name = caravan.cargo.name();
            }
        }
    }
    
    // Draw nodes
    for node in &region.nodes {
        let pos = node_to_screen(node, padding, map_width, map_height);
        let is_selected = region.selected_node == Some(node.id);
        let is_active = region.active_town_id == Some(node.id);
        
        // Biome background circle
        let biome_color = node.biome.map_color();
        let bg_color = Color::new(biome_color[0], biome_color[1], biome_color[2], 0.5);
        
        if node.is_wonder_site {
            // Wonder sites are diamond shaped
            draw_poly(pos.x, pos.y, 4, 35.0, 45.0, bg_color);
        } else {
            draw_circle(pos.x, pos.y, 35.0, bg_color);
        }
        
        // Node circle or diamond
        let node_color = if node.is_wonder_site {
            if node.wonder_site.is_some() {
                PURPLE // Active construction
            } else {
                MAGENTA // Available wonder site
            }
        } else if node.settled {
            if node.is_capital {
                GOLD
            } else {
                GREEN
            }
        } else {
            GRAY
        };
        
        if node.is_wonder_site {
            draw_poly(pos.x, pos.y, 4, 20.0, 45.0, node_color);
            
            // Show construction progress ring if building
            if let Some(ref wonder_site) = node.wonder_site {
                let progress = wonder_site.overall_progress();
                if !wonder_site.completed {
                    // Draw progress arc (simple circle for now)
                    let progress_color = Color::new(0.2, 0.8, 0.2, progress);
                    draw_circle_lines(pos.x, pos.y, 28.0, 3.0 * progress, progress_color);
                } else {
                    // Completed wonder glow
                    draw_circle_lines(pos.x, pos.y, 30.0, 3.0, GOLD);
                }
            }
        } else {
            draw_circle(pos.x, pos.y, 20.0, node_color);
        }
        
        // Selection ring
        if is_selected {
            if node.is_wonder_site {
                draw_poly_lines(pos.x, pos.y, 4, 28.0, 45.0, 3.0, WHITE);
            } else {
                draw_circle_lines(pos.x, pos.y, 28.0, 3.0, WHITE);
            }
        }
        
        // Active indicator
        if is_active {
            draw_circle_lines(pos.x, pos.y, 24.0, 2.0, YELLOW);
        }
        
        // Capital star or Wonder icon
        if node.is_capital {
            draw_text("★", pos.x - 8.0, pos.y + 6.0, 24.0, GOLD);
        } else if node.is_wonder_site {
            if let Some(ref ws) = node.wonder_site {
                if ws.completed {
                    draw_text("🏛️", pos.x - 10.0, pos.y + 6.0, 20.0, GOLD);
                } else {
                    draw_text("⚒️", pos.x - 8.0, pos.y + 6.0, 16.0, WHITE);
                }
            } else {
                draw_text("◆", pos.x - 6.0, pos.y + 6.0, 18.0, MAGENTA);
            }
        }
        
        // Name label
        let name_x = pos.x - measure_text(&node.name, None, 16, 1.0).width / 2.0;
        draw_text(&node.name, name_x, pos.y + 40.0, 16.0, WHITE);
        
        // Biome or Wonder label
        let label: String = if let Some(ref ws) = node.wonder_site {
            if ws.completed {
                ws.wonder.name().to_string()
            } else {
                // Show current stage name and stage progress
                if let Some(stage) = ws.current_stage_info() {
                    let stage_pct = (ws.stage_progress_percent() * 100.0) as i32;
                    format!("{}: {} ({}%)", ws.wonder.name(), stage.name, stage_pct)
                } else {
                    let progress = (ws.overall_progress() * 100.0) as i32;
                    format!("{}: {}%", ws.wonder.name(), progress)
                }
            }
        } else if node.is_wonder_site {
            "Wonder Site".to_string()
        } else {
            node.biome.name().to_string()
        };
        let label_x = pos.x - measure_text(&label, None, 12, 1.0).width / 2.0;
        draw_text(&label, label_x, pos.y + 55.0, 12.0, LIGHTGRAY);
    }
    
    // Draw title
    draw_text("REGION MAP", screen_width / 2.0 - 80.0, 35.0, 32.0, WHITE);
    
    // Draw region info panel (uses active_town, settled_count, routes_from)
    let panel_x = 10.0;
    let panel_y = 60.0;
    draw_rectangle(panel_x, panel_y, 200.0, 140.0, Color::from_rgba(0, 0, 0, 180));
    draw_text("Region Status", panel_x + 10.0, panel_y + 20.0, 16.0, WHITE);
    
    // Use settled_count method
    let settled = region.settled_count();
    draw_text(&format!("Settled Towns: {}/{}", settled, region.nodes.len()), 
        panel_x + 10.0, panel_y + 40.0, 14.0, LIGHTGRAY);
    
    // Use active_town method
    if let Some(active) = region.active_town() {
        draw_text(&format!("Current: {}", active.name), 
            panel_x + 10.0, panel_y + 58.0, 14.0, GREEN);
            
        // Use routes_from on RegionMap
        let connected_routes = region.routes_from(active.id);
        draw_text(&format!("Connected Routes: {}", connected_routes.len()), 
            panel_x + 10.0, panel_y + 76.0, 14.0, LIGHTGRAY);
            
        // Show travel times using Route::travel_time
        for (i, route) in connected_routes.iter().take(2).enumerate() {
            let travel = route.travel_time();
            draw_text(&format!("  Route {}: {:.1} days", i + 1, travel),
                panel_x + 10.0, panel_y + 94.0 + i as f32 * 16.0, 12.0, GRAY);
        }
    }
    
    // Trade info (uses TradeManager::routes_from and routes_to)
    let caravan_count = trade_manager.active_caravan_count();
    draw_text(&format!("Caravans: {}", caravan_count), 
        panel_x + 10.0, panel_y + 130.0, 14.0, ORANGE);
    
    // Show trade routes for current town
    if let Some(active_id) = region.active_town_id {
        let export_routes = trade_manager.routes_from(active_id).len();
        let import_routes = trade_manager.routes_to(active_id).len();
        draw_text(&format!("Trade: {}↑ {}↓", export_routes, import_routes),
            panel_x + 100.0, panel_y + 130.0, 14.0, LIGHTGRAY);
    }
    
    // Draw instructions
    draw_text("Click a town to select • Press M to return to town", 
        screen_width / 2.0 - 180.0, screen_height - 20.0, 16.0, LIGHTGRAY);
}

/// Convert node position (0-1) to screen coordinates
fn node_to_screen(node: &TownNode, padding: f32, map_width: f32, map_height: f32) -> Vec2 {
    vec2(
        padding + node.position[0] * map_width,
        padding + node.position[1] * map_height
    )
}

/// Draw a tooltip for a hovered node (uses biome multipliers)
pub fn draw_node_tooltip(node: &TownNode, mouse_pos: Vec2) {
    // Use biome methods for display
    let biome = node.biome;
    let mut lines = vec![
        node.name.clone(),
        format!("Biome: {}", biome.name()),
        biome.description().to_string(),
        String::new(),
        format!("Wood: {:.1}x (base: {:.1}x)", 
            node.resource_potentials.wood, biome.wood_multiplier()),
        format!("Stone: {:.1}x (base: {:.1}x)", 
            node.resource_potentials.stone, biome.stone_multiplier()),
        format!("Food: {:.1}x (base: {:.1}x)", 
            node.resource_potentials.food, biome.food_multiplier()),
        format!("Trade: {:.1}x (base: {:.1}x)", 
            node.resource_potentials.trade, biome.trade_multiplier()),
        format!("Temp Bias: {:+.1}", biome.temperature_bias()),
    ];
    
    if node.settled {
        lines.push(String::new());
        lines.push("Status: Settled".to_string());
        if node.is_capital {
            lines.push("★ Capital".to_string());
        }
    } else {
        lines.push(String::new());
        lines.push("Status: Unsettled".to_string());
    }
    
    let max_width = lines.iter()
        .map(|l| measure_text(l, None, 14, 1.0).width)
        .fold(0.0_f32, f32::max);
    
    let width = max_width + 20.0;
    let height = lines.len() as f32 * 18.0 + 15.0;
    
    let x = (mouse_pos.x + 15.0).min(screen_width() - width - 10.0);
    let y = (mouse_pos.y + 15.0).min(screen_height() - height - 10.0);
    
    // Background
    draw_rectangle(x, y, width, height, Color::from_rgba(20, 20, 20, 230));
    draw_rectangle_lines(x, y, width, height, 1.0, GRAY);
    
    // Text
    for (i, line) in lines.iter().enumerate() {
        let color = if i == 0 { WHITE } else { LIGHTGRAY };
        draw_text(line, x + 10.0, y + 18.0 + i as f32 * 18.0, 14.0, color);
    }
}
