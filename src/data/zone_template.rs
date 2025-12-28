//! Zone template - Data-driven zone definitions

use serde::{Deserialize, Serialize};

/// A zone template loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneTemplate {
    pub id: String,
    pub name: String,
    pub category: ZoneCategory,
    
    /// Base throughput before condition/activity multipliers
    pub base_throughput: f32,
    
    /// Cost to build/restore (material units)
    #[serde(default)]
    pub construction_cost: f32,
    
    /// Work units required to complete construction
    #[serde(default = "default_construction_work")]
    pub construction_work: f32,
    
    /// Materials required for construction
    #[serde(default)]
    pub construction_materials: ResourceDelta,
    
    /// How fast diminishing returns kick in (higher = faster plateau)
    pub saturation_bias: f32,
    
    /// Resource effects when zone is active
    pub output: ResourceDelta,
    
    /// Resource costs to maintain the zone
    pub upkeep: ResourceDelta,
    
    /// Population effects
    pub population: PopulationEffect,
    
    /// Decay behavior
    pub decay: DecayModel,
    
    /// Map coordinates for this zone (Phase 4.2)
    #[serde(default)]
    pub map_rect: Option<MapRect>,
    
    /// Tech ID required to unlock this zone (if any)
    #[serde(default)]
    pub locked_by_tech: Option<String>,
}

fn default_construction_work() -> f32 {
    10.0 // Default 10 work units
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MapRect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoneCategory {
    Residential,
    Market,
    Infrastructure,
    Cultural,
    Transit,
    Utility,
}

/// Resource delta (can be positive or negative)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ResourceDelta {
    #[serde(default)]
    pub materials: f32,
    #[serde(default)]
    pub maintenance: f32,
    #[serde(default)]
    pub stability: f32,
    #[serde(default)]
    pub attractiveness: f32,
}

/// How a zone affects population pressure
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct PopulationEffect {
    /// How much this zone attracts new population
    pub attraction: f32,
    /// How much this zone strains population capacity
    pub strain: f32,
    /// Housing capacity provided by this zone
    #[serde(default)]
    pub capacity: f32,
    /// How fast population decays without this zone
    pub decay: f32,
}

/// Zone decay behavior
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DecayModel {
    /// Natural condition decay per tick
    pub natural_rate: f32,
    /// Condition threshold below which zone becomes dormant
    pub neglect_threshold: f32,
}

impl Default for DecayModel {
    fn default() -> Self {
        Self {
            natural_rate: 0.001,
            neglect_threshold: 0.1,
        }
    }
}
