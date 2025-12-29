//! Achievement system for tracking player accomplishments
//! 
//! Achievement definitions are loaded from assets/achievements.json

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Achievement definition loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementDef {
    /// Unique identifier (snake_case)
    pub id: String,
    /// Display name for UI
    pub name: String,
    /// Description/requirements
    pub description: String,
    /// Emoji icon for display
    pub icon: String,
}

/// Manages unlocked achievements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AchievementManager {
    /// Set of unlocked achievement IDs
    pub unlocked: HashSet<String>,
    /// Newly unlocked achievement IDs (for notification display)
    #[serde(skip)]
    pub newly_unlocked: Vec<String>,
    /// Loaded achievement definitions (not serialized - reloaded at startup)
    #[serde(skip)]
    pub definitions: Vec<AchievementDef>,
}

impl AchievementManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize with loaded definitions
    pub fn with_definitions(definitions: Vec<AchievementDef>) -> Self {
        Self {
            definitions,
            ..Default::default()
        }
    }
    
    /// Set definitions (called after loading from JSON)
    pub fn set_definitions(&mut self, definitions: Vec<AchievementDef>) {
        self.definitions = definitions;
    }
    
    /// Unlock an achievement by ID if not already unlocked
    /// Returns true if newly unlocked
    pub fn unlock(&mut self, achievement_id: &str) -> bool {
        if self.unlocked.insert(achievement_id.to_string()) {
            self.newly_unlocked.push(achievement_id.to_string());
            true
        } else {
            false
        }
    }
    
    /// Get achievement definition by ID
    pub fn get_def(&self, id: &str) -> Option<&AchievementDef> {
        self.definitions.iter().find(|d| d.id == id)
    }
    
    /// Get count of unlocked achievements
    pub fn count(&self) -> usize {
        self.unlocked.len()
    }
    
    /// Get total number of achievements
    pub fn total(&self) -> usize {
        self.definitions.len()
    }
    
    /// Pop a newly unlocked achievement for display
    pub fn pop_notification(&mut self) -> Option<AchievementDef> {
        let id = self.newly_unlocked.pop()?;
        self.get_def(&id).cloned()
    }
    
    /// Get all unlocked achievements as sorted definitions
    pub fn unlocked_list(&self) -> Vec<&AchievementDef> {
        let mut list: Vec<_> = self.definitions.iter()
            .filter(|d| self.unlocked.contains(&d.id))
            .collect();
        list.sort_by_key(|a| &a.name);
        list
    }
    
    /// Check if specific achievement is unlocked
    pub fn is_unlocked(&self, id: &str) -> bool {
        self.unlocked.contains(id)
    }
}

/// Lifetime game statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameStats {
    /// Total zones placed/restored
    pub zones_restored: u32,
    /// Total resources collected over all time
    pub resources_collected: f32,
    /// Peak resources held at once
    pub peak_resources: f32,
    /// Total agents born
    pub agents_born: u32,
    /// Total agents died
    pub agents_died: u32,
    /// Total in-game play hours
    pub total_play_hours: f32,
    /// Technologies researched
    pub techs_researched: u32,
    /// Wonders completed
    pub wonders_built: u32,
    /// Heroes immortalized
    pub heroes_immortalized: u32,
    /// Heroes retired as ancestors
    pub heroes_retired: u32,
    /// Towns settled
    pub towns_settled: u32,
    /// Winters survived
    pub winters_survived: u32,
    /// Peak population reached
    pub peak_population: u32,
}

impl GameStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record resource collection
    pub fn add_resources(&mut self, amount: f32) {
        self.resources_collected += amount;
    }
    
    /// Update peak tracking
    pub fn update_peaks(&mut self, current_resources: f32, current_population: u32) {
        if current_resources > self.peak_resources {
            self.peak_resources = current_resources;
        }
        if current_population > self.peak_population {
            self.peak_population = current_population;
        }
    }
}
