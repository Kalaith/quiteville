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

    /// Create a zone that starts active
    pub fn new_active(template_id: &str) -> Self {
        Self {
            template_id: template_id.to_string(),
            condition: 1.0,
            activity: 0.5,
            dormant: false,
            reawakening_stage: 1,
            construction_state: ConstructionState::Complete,
        }
    }

    /// Create a new zone under construction
    pub fn new_under_construction(template_id: &str) -> Self {
        Self {
            template_id: template_id.to_string(),
            condition: 0.0,
            activity: 0.0,
            dormant: true,
            reawakening_stage: 0,
            construction_state: ConstructionState::UnderConstruction {
                work_done: 0.0,
                materials_deposited: false,
            },
        }
    }

    /// Check if zone is under construction
    pub fn is_under_construction(&self) -> bool {
        matches!(self.construction_state, ConstructionState::UnderConstruction { .. })
    }

    /// Apply construction work to a zone under construction
    /// Returns true if construction completed
    pub fn apply_construction_work(&mut self, work_amount: f32, required_work: f32) -> bool {
        if let ConstructionState::UnderConstruction { ref mut work_done, materials_deposited } = self.construction_state {
            if !materials_deposited {
                return false; // Can't work without materials
            }
            *work_done += work_amount;
            if *work_done >= required_work {
                self.construction_state = ConstructionState::Complete;
                self.condition = 1.0;
                self.dormant = false;
                self.reawakening_stage = 1;
                return true;
            }
        }
        false
    }

    /// Deposit materials for construction
    pub fn deposit_construction_materials(&mut self) {
        if let ConstructionState::UnderConstruction { ref mut materials_deposited, .. } = self.construction_state {
            *materials_deposited = true;
        }
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

    /// Check if zone should become dormant
    pub fn should_go_dormant(&self, template: &ZoneTemplate) -> bool {
        self.condition < template.decay.neglect_threshold
    }

    /// Apply condition decay
    pub fn apply_decay(&mut self, template: &ZoneTemplate, delta_time: f32) {
        if !self.dormant && !self.is_under_construction() {
            self.condition -= template.decay.natural_rate * delta_time;
            self.condition = self.condition.max(0.0);
        }
    }

    /// Restore some condition (player action)
    pub fn restore(&mut self, amount: f32) {
        self.condition = (self.condition + amount).min(1.0);
        if self.dormant && self.condition > 0.1 {
            self.dormant = false;
        }
    }

    /// Update activity based on population pressure
    pub fn update_activity(&mut self, effective_population: f32) {
        // Activity slowly moves toward effective population
        let target = effective_population;
        let lerp_speed = 0.1;
        self.activity += (target - self.activity) * lerp_speed;
        self.activity = self.activity.clamp(0.0, 1.0);
    }
}

