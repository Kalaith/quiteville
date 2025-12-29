//! Biome system for region generation

use serde::{Deserialize, Serialize};

/// Biome types that affect resource availability and weather
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Biome {
    /// Temperate grasslands - balanced resources
    Plains,
    /// Dense woodland - bonus wood, less stone
    Forest,
    /// Rocky terrain - bonus stone, harsh conditions
    Mountains,
    /// Arid region - bonus stone, less food
    Desert,
    /// Seaside - bonus trade and fish
    Coast,
    /// Frozen terrain - furs and harsh winters
    Tundra,
    /// Wetlands - high food, disease risk
    Swamp,
}

impl Default for Biome {
    fn default() -> Self {
        Biome::Plains
    }
}

impl Biome {
    /// Display name for UI
    pub fn name(&self) -> &'static str {
        match self {
            Biome::Plains => "Plains",
            Biome::Forest => "Forest",
            Biome::Mountains => "Mountains",
            Biome::Desert => "Desert",
            Biome::Coast => "Coast",
            Biome::Tundra => "Tundra",
            Biome::Swamp => "Swamp",
        }
    }
    
    /// Description for tooltips
    pub fn description(&self) -> &'static str {
        match self {
            Biome::Plains => "Temperate grasslands with balanced resources",
            Biome::Forest => "Dense woodland with abundant timber",
            Biome::Mountains => "Rocky terrain rich in stone and ore",
            Biome::Desert => "Arid region with little vegetation",
            Biome::Coast => "Seaside location ideal for trade",
            Biome::Tundra => "Frozen terrain with valuable furs",
            Biome::Swamp => "Wetlands rich in food but prone to disease",
        }
    }
    
    /// Color tint for map display (RGBA)
    pub fn map_color(&self) -> [f32; 4] {
        match self {
            Biome::Plains => [0.5, 0.8, 0.4, 1.0],   // Green
            Biome::Forest => [0.2, 0.5, 0.2, 1.0],   // Dark green
            Biome::Mountains => [0.6, 0.5, 0.4, 1.0], // Brown/gray
            Biome::Desert => [0.9, 0.8, 0.5, 1.0],   // Sand yellow
            Biome::Coast => [0.3, 0.5, 0.8, 1.0],    // Blue
            Biome::Tundra => [0.8, 0.9, 0.95, 1.0],  // Ice white/blue
            Biome::Swamp => [0.3, 0.4, 0.3, 1.0],    // Murky green
        }
    }
    
    /// Wood gathering multiplier
    pub fn wood_multiplier(&self) -> f32 {
        match self {
            Biome::Plains => 1.0,
            Biome::Forest => 2.0,
            Biome::Mountains => 0.5,
            Biome::Desert => 0.2,
            Biome::Coast => 0.3,
            Biome::Tundra => 0.3,
            Biome::Swamp => 0.8,
        }
    }
    
    /// Stone gathering multiplier
    pub fn stone_multiplier(&self) -> f32 {
        match self {
            Biome::Plains => 1.0,
            Biome::Forest => 0.5,
            Biome::Mountains => 2.5,
            Biome::Desert => 1.5,
            Biome::Coast => 0.5,
            Biome::Tundra => 0.8,
            Biome::Swamp => 0.2,
        }
    }
    
    /// Food production multiplier
    pub fn food_multiplier(&self) -> f32 {
        match self {
            Biome::Plains => 1.2,
            Biome::Forest => 0.8,
            Biome::Mountains => 0.3,
            Biome::Desert => 0.4,
            Biome::Coast => 1.5, // Fishing
            Biome::Tundra => 0.3,
            Biome::Swamp => 1.5, // Abundant hunting/fishing
        }
    }
    
    /// Trade income multiplier
    pub fn trade_multiplier(&self) -> f32 {
        match self {
            Biome::Plains => 1.0,
            Biome::Forest => 0.5,
            Biome::Mountains => 0.5,
            Biome::Desert => 0.8,
            Biome::Coast => 2.0,
            Biome::Tundra => 1.5, // Valuable furs
            Biome::Swamp => 0.4,
        }
    }
    
    /// Temperature tendency (affects seasonal severity)
    pub fn temperature_bias(&self) -> f32 {
        match self {
            Biome::Plains => 0.0,
            Biome::Forest => -0.1, // Slightly cooler
            Biome::Mountains => -0.3, // Cold
            Biome::Desert => 0.4, // Hot
            Biome::Coast => 0.0, // Moderate
            Biome::Tundra => -0.5, // Very cold
            Biome::Swamp => 0.1, // Humid and warm
        }
    }
}

