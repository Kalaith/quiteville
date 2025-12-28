//! Zone runtime state

use serde::{Deserialize, Serialize};
use crate::data::ZoneTemplate;

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
}

impl Zone {
    /// Create a new zone from a template
    pub fn new(template_id: &str) -> Self {
        Self {
            template_id: template_id.to_string(),
            condition: 0.0,
            activity: 0.0,
            dormant: true,
            reawakening_stage: 0,
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
        }
    }

    /// Calculate effective throughput based on condition, activity, and saturation
    /// Formula: base × condition × saturation(activity, bias)
    /// We removed the extra 'activity' multiplier to prevent double-penalty at low pop.
    pub fn calculate_throughput(&self, template: &ZoneTemplate) -> f32 {
        if self.dormant {
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
        if !self.dormant {
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
