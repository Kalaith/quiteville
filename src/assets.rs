//! Assets module - Embedded assets for WASM compatibility
//!
//! All game data is loaded using include_str! to ensure WebGL builds work correctly.

/// Game configuration loaded from embedded JSON
pub const CONFIG_JSON: &str = include_str!("../assets/config.json");

/// Zone definitions
pub const ZONES_JSON: &str = include_str!("../assets/zones.json");

/// Achievement definitions
pub const ACHIEVEMENTS_JSON: &str = include_str!("../assets/achievements.json");

/// Load and parse the game configuration
pub fn load_config() -> Result<crate::data::GameConfig, serde_json::Error> {
    serde_json::from_str(CONFIG_JSON)
}

/// Load and parse zone templates
pub fn load_zones() -> Result<Vec<crate::data::ZoneTemplate>, serde_json::Error> {
    serde_json::from_str(ZONES_JSON)
}

/// Load and parse achievement definitions
pub fn load_achievements() -> Result<Vec<crate::data::AchievementDef>, serde_json::Error> {
    serde_json::from_str(ACHIEVEMENTS_JSON)
}

use std::collections::HashMap;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct GameAssets {
    pub textures: HashMap<String, Texture2D>,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}

impl GameAssets {
    pub fn get(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
}

pub async fn load_textures() -> GameAssets {
    let mut textures = HashMap::new();
    
    // List of assets to load (matching filenames in art_prompts.json)
    let asset_names = [
        "tile_grass", "tile_dirt", "tile_water", "tile_floor", "tile_wall", "tile_ruins",
        "building_homestead_large", "building_well_large", "building_village_green_large",
        "building_market_large", "building_workshop_large", "building_farm_large",
        "agent_villager", 
        "icon_thought_shopping", "icon_thought_working", "icon_thought_social", "icon_thought_sleep"
    ];
    
    for name in asset_names {
        // macroquad::load_texture requires path relative to executable or assets folder
        // For cargo run, "assets/" usually works if at root.
        let path = format!("assets/{}.png", name);
        match load_texture(&path).await {
            Ok(tex) => {
                tex.set_filter(FilterMode::Nearest);
                textures.insert(name.to_string(), tex);
            },
            Err(e) => {
                eprintln!("Failed to load texture {}: {}", path, e);
            }
        }
    }
    
    GameAssets { textures }
}
