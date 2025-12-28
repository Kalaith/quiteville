//! Core resources and formula implementations

use serde::{Deserialize, Serialize};

/// The core resources that drive the game
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Resources {
    // === Abstract Resources (Original) ===
    /// Raw materials for building and repair
    pub materials: f32,
    /// How well it stays running
    pub maintenance: f32,
    /// How much population pressure grows
    pub attractiveness: f32,
    /// How predictable the city is
    pub stability: f32,
    
    // === Raw Materials (Phase 2) ===
    /// Raw logs from woodcutters
    #[serde(default)]
    pub logs: f32,
    /// Raw stone chunks from quarry
    #[serde(default)]
    pub stone_chunks: f32,
    /// Raw grain from farms
    #[serde(default)]
    pub grain: f32,
    
    // === Processed Materials (Phase 2) ===
    /// Processed lumber (from logs)
    #[serde(default)]
    pub lumber: f32,
    /// Cut stone (from stone chunks)
    #[serde(default)]
    pub cut_stone: f32,
    /// Flour (from grain)
    #[serde(default)]
    pub flour: f32,
}

impl Resources {
    /// Create resources with initial values (legacy)
    pub fn new(materials: f32, maintenance: f32, attractiveness: f32, stability: f32) -> Self {
        Self {
            materials,
            maintenance,
            attractiveness,
            stability,
            logs: 0.0,
            stone_chunks: 0.0,
            grain: 0.0,
            lumber: 0.0,
            cut_stone: 0.0,
            flour: 0.0,
        }
    }

    /// Apply a delta to resources (can be positive or negative)
    pub fn apply_delta(&mut self, delta: &crate::data::ResourceDelta) {
        self.materials += delta.materials;
        self.maintenance += delta.maintenance;
        self.attractiveness += delta.attractiveness;
        self.stability += delta.stability;
        
        // Soft floor at 0 (resources can't go negative)
        self.materials = self.materials.max(0.0);
        self.maintenance = self.maintenance.max(0.0);
        self.attractiveness = self.attractiveness.max(0.0);
        self.stability = self.stability.max(0.0);
    }
    
    /// Apply a raw/processed resource delta
    pub fn apply_resource_change(&mut self, resource: ResourceType, amount: f32) {
        match resource {
            ResourceType::Materials => self.materials = (self.materials + amount).max(0.0),
            ResourceType::Logs => self.logs = (self.logs + amount).max(0.0),
            ResourceType::StoneChunks => self.stone_chunks = (self.stone_chunks + amount).max(0.0),
            ResourceType::Grain => self.grain = (self.grain + amount).max(0.0),
            ResourceType::Lumber => self.lumber = (self.lumber + amount).max(0.0),
            ResourceType::CutStone => self.cut_stone = (self.cut_stone + amount).max(0.0),
            ResourceType::Flour => self.flour = (self.flour + amount).max(0.0),
        }
    }
    
    /// Check if we have enough of a resource
    pub fn has(&self, resource: ResourceType, amount: f32) -> bool {
        self.get(resource) >= amount
    }
    
    /// Get amount of a specific resource
    pub fn get(&self, resource: ResourceType) -> f32 {
        match resource {
            ResourceType::Materials => self.materials,
            ResourceType::Logs => self.logs,
            ResourceType::StoneChunks => self.stone_chunks,
            ResourceType::Grain => self.grain,
            ResourceType::Lumber => self.lumber,
            ResourceType::CutStone => self.cut_stone,
            ResourceType::Flour => self.flour,
        }
    }
}

/// Types of resources for the resource chain system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Materials,
    Logs,
    StoneChunks,
    Grain,
    Lumber,
    CutStone,
    Flour,
}

impl ResourceType {
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::Materials => "Materials",
            ResourceType::Logs => "Logs",
            ResourceType::StoneChunks => "Stone Chunks",
            ResourceType::Grain => "Grain",
            ResourceType::Lumber => "Lumber",
            ResourceType::CutStone => "Cut Stone",
            ResourceType::Flour => "Flour",
        }
    }
    
    /// Is this a raw material?
    pub fn is_raw(&self) -> bool {
        matches!(self, ResourceType::Logs | ResourceType::StoneChunks | ResourceType::Grain)
    }
    
    /// Is this a processed material?
    pub fn is_processed(&self) -> bool {
        matches!(self, ResourceType::Lumber | ResourceType::CutStone | ResourceType::Flour)
    }
}

// ============================================================================
// CORE FORMULAS - From formulas_and_statistics.md
// These are the "unbreakable" formulas that prevent spiral issues.
// ============================================================================

/// Calculate effective population from pressure.
/// Formula: P / (P + K)
/// Returns value in range [0, 1), never reaches 1.
pub fn effective_population(pressure: f32, k: f32) -> f32 {
    if pressure <= 0.0 {
        return 0.0;
    }
    pressure / (pressure + k)
}

/// Material factor for output calculation.
/// Formula: M / (M + 1)
/// Returns minimum 0.1 even at 0 materials (zones still produce something)
pub fn material_factor(materials: f32) -> f32 {
    const MIN_FACTOR: f32 = 0.1;
    if materials <= 0.0 {
        return MIN_FACTOR;
    }
    (materials / (materials + 1.0)).max(MIN_FACTOR)
}

/// Maintenance factor for output calculation.
/// Formula: √M / (√M + 1)
/// Returns minimum 0.1 even at 0 maintenance
pub fn maintenance_factor(maintenance: f32) -> f32 {
    const MIN_FACTOR: f32 = 0.1;
    if maintenance <= 0.0 {
        return MIN_FACTOR;
    }
    let sqrt_m = maintenance.sqrt();
    (sqrt_m / (sqrt_m + 1.0)).max(MIN_FACTOR)
}

/// Stability factor for output calculation.
/// Formula: ln(S + 1) / ln(S + 2)
/// Returns minimum 0.1 even at 0 stability
pub fn stability_factor(stability: f32) -> f32 {
    const MIN_FACTOR: f32 = 0.1;
    if stability <= 0.0 {
        return MIN_FACTOR;
    }
    ((stability + 1.0).ln() / (stability + 2.0).ln()).max(MIN_FACTOR)
}

/// Calculate final output from base output and resource factors.
/// Formula: Base × MaterialFactor × MaintenanceFactor × StabilityFactor
pub fn calculate_output(base: f32, resources: &Resources) -> f32 {
    base * material_factor(resources.materials)
        * maintenance_factor(resources.maintenance)
        * stability_factor(resources.stability)
}

/// Calculate maintenance cost from effective population.
/// Formula: β × EffectivePopulation²
/// The squared term ensures growth pressure always pushes back.
pub fn maintenance_cost(effective_pop: f32, coefficient: f32) -> f32 {
    coefficient * effective_pop * effective_pop
}

/// Calculate offline gain.
/// Formula: Output × log(TimeAway + 1)
/// Logarithmic scaling prevents AFK abuse.
pub fn offline_gain(output: f32, hours_away: f32) -> f32 {
    output * (hours_away + 1.0).ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_population_saturation() {
        // Early game: P=5, K=10 -> 0.33
        let result = effective_population(5.0, 10.0);
        assert!((result - 0.333).abs() < 0.01);
        
        // Late game: P=1000, K=10 -> ~0.99
        let result = effective_population(1000.0, 10.0);
        assert!(result > 0.98 && result < 1.0);
        
        // Never reaches 1.0
        let result = effective_population(1_000_000.0, 10.0);
        assert!(result < 1.0);
    }

    #[test]
    fn test_material_factor_diminishing_returns() {
        // Low materials: M=1 -> 0.5
        assert!((material_factor(1.0) - 0.5).abs() < 0.01);
        
        // High materials: M=50 -> ~0.98
        let result = material_factor(50.0);
        assert!(result > 0.95 && result < 1.0);
    }

    #[test]
    fn test_maintenance_cost_scaling() {
        // Cost scales with population squared
        let cost_low = maintenance_cost(0.33, 0.02);
        let cost_high = maintenance_cost(0.99, 0.02);
        
        // High population costs ~9x more
        assert!(cost_high / cost_low > 8.0);
    }

    #[test]
    fn test_offline_gain_logarithmic() {
        let output = 8.0;
        
        // 1 hour away
        let gain_1h = offline_gain(output, 1.0);
        
        // 72 hours away
        let gain_72h = offline_gain(output, 72.0);
        
        // 72x time should NOT give 72x gain (log scaling)
        assert!(gain_72h < gain_1h * 10.0);
    }
}
