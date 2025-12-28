//! Population pressure - The central driver of the game

use serde::{Deserialize, Serialize};

/// Population pressure state
/// 
/// Population is NOT a count of people. It's a pressure value that:
/// - Generates needs
/// - Creates momentum
/// - Drives zone activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationPressure {
    /// Current pressure value (0 to infinity, but saturates via formulas)
    pressure: f32,
    
    /// Growth rate per tick (affected by attractiveness)
    growth_rate: f32,
    
    /// Decay rate per tick (base decay when no attractiveness)
    decay_rate: f32,
}

impl Default for PopulationPressure {
    fn default() -> Self {
        Self {
            pressure: 0.0,
            growth_rate: 0.1,
            decay_rate: 0.005,
        }
    }
}

impl PopulationPressure {
    /// Create with initial pressure
    pub fn new(initial: f32) -> Self {
        Self {
            pressure: initial,
            ..Default::default()
        }
    }

    /// Get current pressure value
    pub fn value(&self) -> f32 {
        self.pressure
    }

    /// Update pressure based on attractiveness and housing capacity
    pub fn tick(&mut self, attractiveness: f32, capacity: f32, delta_time: f32) {
        // Growth is driven by attractiveness BUT limited by housing capacity
        // Logistic growth-like behavior: slows as it approaches capacity.
        let space_factor = if capacity > 0.0 {
            (1.0 - (self.pressure / capacity)).max(0.0)
        } else {
            0.0 // No space = no growth
        };
        
        let growth = self.growth_rate * attractiveness * space_factor * delta_time;
        
        // Decay is always present
        // If over capacity (pressure > capacity), add extra decay to simulate overcrowding
        let overcrowding_factor = if self.pressure > capacity {
            1.0 + (self.pressure - capacity) * 0.1
        } else {
            1.0
        };
        
        let decay = self.decay_rate * overcrowding_factor * delta_time;
        
        self.pressure += growth - decay;
        self.pressure = self.pressure.max(0.0);
    }

    /// Apply external pressure change (from events, milestones, etc.)
    pub fn apply_delta(&mut self, delta: f32) {
        self.pressure = (self.pressure + delta).max(0.0);
    }

    /// Set growth rate (modified by zones, milestones)
    pub fn set_growth_rate(&mut self, rate: f32) {
        self.growth_rate = rate.max(0.0);
    }

    /// Set decay rate
    pub fn set_decay_rate(&mut self, rate: f32) {
        self.decay_rate = rate.max(0.0);
    }
}
