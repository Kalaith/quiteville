use serde::{Deserialize, Serialize};
// use crate::data::ResourceDelta;

/// Condition for unlocking a tech node
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UnlockCondition {
    /// Standard cost-based purchase
    Cost { materials: f32 },
    /// Requires a milestone to be achieved
    MilestoneAchieved { milestone_id: String },
    /// Requires a certain number of zones built
    ZonesBuilt { count: u32 },
    /// Requires specific zone to be active
    ZoneActive { zone_id: String },
    /// Automatically available once parent is unlocked
    ParentOnly,
}

impl Default for UnlockCondition {
    fn default() -> Self {
        Self::ParentOnly
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: f32, // Material cost for now
    pub parent_id: Option<String>,
    pub unlocked: bool,
    pub effect: TechEffect,
    
    /// Optional unlock condition (if None, just requires cost and parent)
    #[serde(default)]
    pub unlock_condition: Option<UnlockCondition>,
    
    // UI layout hint
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechEffect {
    ProductionMulti(f32), // e.g. 1.1 for +10%
    EfficiencyMulti(f32), // Reduces upkeep e.g. 0.9 for -10% cost
    AttractivenessFlat(f32),
    HousingGlobal(f32),
}

impl TechNode {
    pub fn new(id: &str, name: &str, desc: &str, cost: f32, parent: Option<&str>, effect: TechEffect, x: f32, y: f32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: desc.to_string(),
            cost,
            parent_id: parent.map(|s| s.to_string()),
            unlocked: false,
            effect,
            unlock_condition: None,
            x,
            y,
        }
    }
    
    pub fn with_unlock_condition(mut self, condition: UnlockCondition) -> Self {
        self.unlock_condition = Some(condition);
        self
    }
}

pub fn default_tech_tree() -> Vec<TechNode> {
    vec![
        // Tier 1: Basics
        TechNode::new(
            "masonry", "Basic Masonry", "Better construction techniques increase production by 10%.", 
            5.0, None, TechEffect::ProductionMulti(1.1), 0.0, 0.0
        ),
        
        // Tier 2: Specialized
        TechNode::new(
            "logistics", "Logistics", "Optimized paths reduce maintenance costs by 15%.", 
            15.0, Some("masonry"), TechEffect::EfficiencyMulti(0.85), -100.0, 100.0
        ),
        TechNode::new(
            "urban_planning", "Urban Planning", "Proper zoning makes the town more attractive.", 
            20.0, Some("masonry"), TechEffect::AttractivenessFlat(0.5), 100.0, 100.0
        ),
        
        // Tier 3: Expansion
        TechNode::new(
            "insulation", "Better Insulation", "Warm homes allow for higher housing density (+5 Capacity).", 
            50.0, Some("urban_planning"), TechEffect::HousingGlobal(5.0), 100.0, 200.0
        ),
        TechNode::new(
            "automation", "Renovation Tools", "Advanced tools drastically boost production (+25%).", 
            75.0, Some("logistics"), TechEffect::ProductionMulti(1.25), -100.0, 200.0
        ),
    ]
}
