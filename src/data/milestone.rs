//! Milestone definitions - Progression triggers loaded from JSON

use serde::{Deserialize, Serialize};

/// A reawakening milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub name: String,
    pub description: String,
    
    /// Conditions that must all be met
    pub conditions: Vec<MilestoneCondition>,
    
    /// Effects when milestone is reached
    pub effects: Vec<MilestoneEffect>,
}

/// A condition for triggering a milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MilestoneCondition {
    /// Population pressure must be at least this value
    PopulationMin { value: f32 },
    /// A specific resource must be at least this value
    ResourceMin { resource: String, value: f32 },
    /// A specific zone must be at this condition level
    ZoneCondition { zone_id: String, min_condition: f32 },
    /// Time played in hours
    TimePlayed { hours: f32 },
}

/// An effect applied when milestone is reached
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MilestoneEffect {
    /// Unlock a new zone
    UnlockZone { zone_id: String },
    /// Modify population saturation K
    ModifyPopulationK { delta: f32 },
    /// Add narrative log entry
    AddLog { message: String },
    /// Modify decay rates globally
    ModifyDecayRate { multiplier: f32 },
}
