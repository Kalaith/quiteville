//! Wonders - Mega-projects that span multiple sessions

use serde::{Deserialize, Serialize};

/// Types of wonders that can be built
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Wonder {
    /// Great Library - Boosts research speed
    GreatLibrary,
    /// Colosseum of Heroes - Attracts more villagers
    ColosseumOfHeroes,
    /// Sky Forge - Boosts production
    SkyForge,
    /// The Cloud Spire - Ultimate endgame wonder
    CloudSpire,
}

impl Wonder {
    pub fn name(&self) -> &'static str {
        match self {
            Wonder::GreatLibrary => "The Great Library",
            Wonder::ColosseumOfHeroes => "Colosseum of Heroes",
            Wonder::SkyForge => "The Sky Forge",
            Wonder::CloudSpire => "The Cloud Spire",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Wonder::GreatLibrary => "A monument to knowledge. Boosts research speed by 50%.",
            Wonder::ColosseumOfHeroes => "Honor the greatest villagers. +25% population growth.",
            Wonder::SkyForge => "Harness the power of the heavens. +30% production.",
            Wonder::CloudSpire => "The ultimate achievement. Unlocks the ending.",
        }
    }
    
    /// Get the stages required to build this wonder
    pub fn stages(&self) -> Vec<WonderStage> {
        match self {
            Wonder::GreatLibrary => vec![
                WonderStage::new("Foundation", "Lay the foundation stones", 100.0),
                WonderStage::new("Pillars", "Erect the great pillars", 200.0),
                WonderStage::new("Halls", "Build the reading halls", 300.0),
                WonderStage::new("Collection", "Gather the ancient texts", 200.0),
            ],
            Wonder::ColosseumOfHeroes => vec![
                WonderStage::new("Arena Floor", "Prepare the arena grounds", 150.0),
                WonderStage::new("Seating", "Build the spectator stands", 250.0),
                WonderStage::new("Gates", "Construct the grand gates", 200.0),
                WonderStage::new("Statues", "Erect the hero statues", 150.0),
            ],
            Wonder::SkyForge => vec![
                WonderStage::new("Forge Base", "Build the forge foundation", 200.0),
                WonderStage::new("Bellows", "Install the great bellows", 300.0),
                WonderStage::new("Chimney", "Raise the sky chimney", 250.0),
                WonderStage::new("Ignition", "Light the eternal flame", 100.0),
            ],
            Wonder::CloudSpire => vec![
                WonderStage::new("Base Platform", "Create the floating platform", 500.0),
                WonderStage::new("First Tier", "Build the first sky level", 600.0),
                WonderStage::new("Second Tier", "Extend towards the clouds", 700.0),
                WonderStage::new("Observatory", "Add the celestial observatory", 500.0),
                WonderStage::new("Pinnacle", "Complete the spire's peak", 700.0),
            ],
        }
    }
    
    /// Total resources needed to complete
    pub fn total_cost(&self) -> f32 {
        self.stages().iter().map(|s| s.cost).sum()
    }
    
    /// Is this the endgame wonder?
    pub fn is_endgame(&self) -> bool {
        matches!(self, Wonder::CloudSpire)
    }
}

/// A single stage in wonder construction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WonderStage {
    pub name: String,
    pub description: String,
    pub cost: f32,
}

impl WonderStage {
    pub fn new(name: &str, description: &str, cost: f32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            cost,
        }
    }
}

/// A wonder construction site on the region map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WonderSite {
    /// Which wonder is being built
    pub wonder: Wonder,
    /// Current construction stage (0-indexed)
    pub current_stage: usize,
    /// Resources invested in current stage
    pub stage_progress: f32,
    /// Whether construction is complete
    pub completed: bool,
    /// When construction started (game time)
    pub started_at: f32,
    /// When completed (if applicable)
    pub completed_at: Option<f32>,
}

impl WonderSite {
    pub fn new(wonder: Wonder, game_time: f32) -> Self {
        Self {
            wonder,
            current_stage: 0,
            stage_progress: 0.0,
            completed: false,
            started_at: game_time,
            completed_at: None,
        }
    }
    
    /// Get current stage info
    pub fn current_stage_info(&self) -> Option<WonderStage> {
        let stages = self.wonder.stages();
        stages.get(self.current_stage).cloned()
    }
    
    /// Get progress percentage for current stage (0.0 - 1.0)
    pub fn stage_progress_percent(&self) -> f32 {
        if let Some(stage) = self.current_stage_info() {
            (self.stage_progress / stage.cost).min(1.0)
        } else {
            1.0
        }
    }
    
    /// Get overall progress percentage (0.0 - 1.0)
    pub fn overall_progress(&self) -> f32 {
        let stages = self.wonder.stages();
        let total_cost = self.wonder.total_cost();
        
        let mut completed_cost: f32 = stages.iter()
            .take(self.current_stage)
            .map(|s| s.cost)
            .sum();
        completed_cost += self.stage_progress;
        
        completed_cost / total_cost
    }
    
    /// Contribute resources to the wonder
    /// Returns (resources used, stage completed, wonder completed)
    pub fn contribute(&mut self, materials: f32, game_time: f32) -> (f32, bool, bool) {
        if self.completed {
            return (0.0, false, false);
        }
        
        let stages = self.wonder.stages();
        if self.current_stage >= stages.len() {
            return (0.0, false, false);
        }
        
        let stage = &stages[self.current_stage];
        let needed = stage.cost - self.stage_progress;
        let used = materials.min(needed);
        
        self.stage_progress += used;
        
        let mut stage_completed = false;
        let mut wonder_completed = false;
        
        // Check if stage is complete
        if self.stage_progress >= stage.cost {
            stage_completed = true;
            self.current_stage += 1;
            self.stage_progress = 0.0;
            
            // Check if wonder is complete
            if self.current_stage >= stages.len() {
                self.completed = true;
                self.completed_at = Some(game_time);
                wonder_completed = true;
            }
        }
        
        (used, stage_completed, wonder_completed)
    }
}

/// Check if Cloud Spire can be unlocked
pub fn can_build_cloud_spire(
    completed_wonders: &[Wonder],
    legacy_points: u32,
    population: f32,
) -> bool {
    // Requires at least 3 other wonders
    let other_wonders = completed_wonders.iter()
        .filter(|w| !w.is_endgame())
        .count();
    
    other_wonders >= 3 && legacy_points >= 1000 && population >= 50.0
}

/// Buffs provided by completed wonders
#[derive(Debug, Clone, Default)]
pub struct WonderBuffs {
    pub research_speed: f32,
    pub population_growth: f32,
    pub production: f32,
}

impl WonderBuffs {
    pub fn from_completed(wonders: &[Wonder]) -> Self {
        let mut buffs = Self {
            research_speed: 1.0,
            population_growth: 1.0,
            production: 1.0,
        };
        
        for wonder in wonders {
            match wonder {
                Wonder::GreatLibrary => buffs.research_speed *= 1.5,
                Wonder::ColosseumOfHeroes => buffs.population_growth *= 1.25,
                Wonder::SkyForge => buffs.production *= 1.3,
                Wonder::CloudSpire => {
                    // Cloud Spire boosts everything slightly
                    buffs.research_speed *= 1.1;
                    buffs.population_growth *= 1.1;
                    buffs.production *= 1.1;
                }
            }
        }
        
        buffs
    }
}
