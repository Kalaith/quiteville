//! Game configuration - Balance values loaded from JSON

use serde::{Deserialize, Serialize};

/// Global game configuration - ALL balance values come from here
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Population saturation constant K (higher = slower saturation)
    pub population_k: f32,
    
    /// Maintenance cost coefficient (β in cost = β × EffectivePop²)
    pub maintenance_cost_coefficient: f32,
    
    /// Offline gain time cap in hours
    pub offline_time_cap_hours: f32,
    
    /// Starting resources
    pub starting_resources: ResourceDefaults,
    
    /// Game tick rate in seconds
    pub tick_rate_seconds: f32,
}

/// Default starting resource values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefaults {
    pub materials: f32,
    pub maintenance: f32,
    pub attractiveness: f32,
    pub stability: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        // These are fallbacks ONLY - real values come from config.json
        Self {
            population_k: 10.0,
            maintenance_cost_coefficient: 0.02,
            offline_time_cap_hours: 72.0,
            starting_resources: ResourceDefaults {
                materials: 1.0,
                maintenance: 1.0,
                attractiveness: 0.5,
                stability: 1.0,
            },
            tick_rate_seconds: 1.0,
        }
    }
}
