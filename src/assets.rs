//! Assets module - Embedded assets for WASM compatibility
//!
//! All game data is loaded using include_str! to ensure WebGL builds work correctly.

/// Game configuration loaded from embedded JSON
pub const CONFIG_JSON: &str = include_str!("../assets/config.json");

/// Zone definitions
pub const ZONES_JSON: &str = include_str!("../assets/zones.json");

/// Milestone definitions
pub const MILESTONES_JSON: &str = include_str!("../assets/milestones.json");

/// Load and parse the game configuration
pub fn load_config() -> Result<crate::data::GameConfig, serde_json::Error> {
    serde_json::from_str(CONFIG_JSON)
}

/// Load and parse zone templates
pub fn load_zones() -> Result<Vec<crate::data::ZoneTemplate>, serde_json::Error> {
    serde_json::from_str(ZONES_JSON)
}

/// Load and parse milestones
pub fn load_milestones() -> Result<Vec<crate::data::Milestone>, serde_json::Error> {
    serde_json::from_str(MILESTONES_JSON)
}
