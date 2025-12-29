//! Zone runtime state

use serde::{Deserialize, Serialize};
use crate::data::ZoneTemplate;

/// Construction state for zones being built
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConstructionState {
    /// Zone is not under construction (either not started or complete)
    None,
    /// Zone is under construction
    UnderConstruction {
        /// Work units completed so far
        work_done: f32,
        /// Whether required materials have been deposited
        materials_deposited: bool,
    },
    /// Zone construction is complete
    Complete,
}

impl Default for ConstructionState {
    fn default() -> Self {
        Self::None
    }
}

/// A zone's current runtime state (separate from template data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    /// Reference to template ID
    pub template_id: String,
    
    /// Physical integrity (0.0 - 1.0)
    pub condition: f32,
    
    /// Usage and life (0.0 - 1.0), driven by population pressure
    pub activity: f32,
    
    /// Whether zone is dormant (not producing)
    pub dormant: bool,
    
    /// Current reawakening stage (0 = not started)
    pub reawakening_stage: u8,
    
    /// Construction state for new zones
    #[serde(default)]
    pub construction_state: ConstructionState,
}

impl Zone {
    /// Create a new zone from a template (starts dormant, needs restoration)
    pub fn new(template_id: &str) -> Self {
        Self {
            template_id: template_id.to_string(),
            condition: 0.0,
            activity: 0.0,
            dormant: true,
            reawakening_stage: 0,
            construction_state: ConstructionState::None,
        }
    }

    /// Check if zone is under construction
    pub fn is_under_construction(&self) -> bool {
        matches!(self.construction_state, ConstructionState::UnderConstruction { .. })
    }

    /// Get construction progress as 0.0 - 1.0
    pub fn construction_progress(&self, required_work: f32) -> f32 {
        if let ConstructionState::UnderConstruction { work_done, .. } = self.construction_state {
            if required_work > 0.0 {
                return (work_done / required_work).min(1.0);
            }
        }
        match self.construction_state {
            ConstructionState::Complete => 1.0,
            _ => 0.0,
        }
    }

    /// Calculate effective throughput based on condition, activity, and saturation
    /// Formula: base × condition × saturation(activity, bias)
    /// We removed the extra 'activity' multiplier to prevent double-penalty at low pop.
    pub fn calculate_throughput(&self, template: &ZoneTemplate) -> f32 {
        if self.dormant || self.is_under_construction() {
            return 0.0;
        }

        let saturation = self.activity / (self.activity + template.saturation_bias);
        template.base_throughput * self.condition * saturation
    }

    /// Restore some condition (player action)
    pub fn restore(&mut self, amount: f32) {
        self.condition = (self.condition + amount).min(1.0);
        if self.dormant && self.condition > 0.1 {
            self.dormant = false;
        }
    }
}

