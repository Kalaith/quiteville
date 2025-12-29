//! Achievement system for tracking player accomplishments

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Game achievements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Achievement {
    /// Build first residential zone
    FirstHouse,
    /// Restore 10 zones
    SettlerSpirit,
    /// Reach 50 population
    GrowingCommunity,
    /// Reach 100 population
    FirstHundred,
    /// Reach 500 population  
    TownProper,
    /// Accumulate 1000 materials
    Hoarder,
    /// Accumulate 10000 materials
    Wealthy,
    /// Accumulate 100000 materials
    Millionaire,
    /// Stability + attractiveness both > 0.8
    Utopia,
    /// Build 25 zones
    Builder,
    /// Build 50 zones
    MasterBuilder,
    /// Research all technologies
    Scholar,
    /// Complete first wonder
    LegacyFounder,
    /// Complete 3 wonders
    WonderWorker,
    /// Settle 5 different towns
    DynastyRuler,
    /// Immortalize first hero
    Remembered,
    /// Retire 10 heroes as ancestors
    AncestorWorship,
    /// Play for 10 in-game days
    Dedicated,
    /// Play for 100 in-game days
    Veteran,
    /// Survive first winter
    WinterSurvivor,
}

impl Achievement {
    /// Display name for UI
    pub fn name(&self) -> &'static str {
        match self {
            Achievement::FirstHouse => "First House",
            Achievement::SettlerSpirit => "Settler Spirit",
            Achievement::GrowingCommunity => "Growing Community",
            Achievement::FirstHundred => "First Hundred",
            Achievement::TownProper => "Town Proper",
            Achievement::Hoarder => "Hoarder",
            Achievement::Wealthy => "Wealthy",
            Achievement::Millionaire => "Millionaire",
            Achievement::Utopia => "Utopia",
            Achievement::Builder => "Builder",
            Achievement::MasterBuilder => "Master Builder",
            Achievement::Scholar => "Scholar",
            Achievement::LegacyFounder => "Legacy Founder",
            Achievement::WonderWorker => "Wonder Worker",
            Achievement::DynastyRuler => "Dynasty Ruler",
            Achievement::Remembered => "Remembered",
            Achievement::AncestorWorship => "Ancestor Worship",
            Achievement::Dedicated => "Dedicated",
            Achievement::Veteran => "Veteran",
            Achievement::WinterSurvivor => "Winter Survivor",
        }
    }
    
    /// Description for UI
    pub fn description(&self) -> &'static str {
        match self {
            Achievement::FirstHouse => "Build your first residential zone",
            Achievement::SettlerSpirit => "Restore 10 zones",
            Achievement::GrowingCommunity => "Reach 50 population",
            Achievement::FirstHundred => "Reach 100 population",
            Achievement::TownProper => "Reach 500 population",
            Achievement::Hoarder => "Accumulate 1,000 materials",
            Achievement::Wealthy => "Accumulate 10,000 materials",
            Achievement::Millionaire => "Accumulate 100,000 materials",
            Achievement::Utopia => "Achieve stability and attractiveness above 0.8",
            Achievement::Builder => "Build 25 zones",
            Achievement::MasterBuilder => "Build 50 zones",
            Achievement::Scholar => "Research all technologies",
            Achievement::LegacyFounder => "Complete your first wonder",
            Achievement::WonderWorker => "Complete 3 wonders",
            Achievement::DynastyRuler => "Settle 5 different towns",
            Achievement::Remembered => "Immortalize your first hero",
            Achievement::AncestorWorship => "Retire 10 heroes as ancestors",
            Achievement::Dedicated => "Play for 10 in-game days",
            Achievement::Veteran => "Play for 100 in-game days",
            Achievement::WinterSurvivor => "Survive your first winter",
        }
    }
    
    /// Icon/emoji for display
    pub fn icon(&self) -> &'static str {
        match self {
            Achievement::FirstHouse => "🏠",
            Achievement::SettlerSpirit => "⛺",
            Achievement::GrowingCommunity => "👥",
            Achievement::FirstHundred => "💯",
            Achievement::TownProper => "🏘️",
            Achievement::Hoarder => "📦",
            Achievement::Wealthy => "💰",
            Achievement::Millionaire => "💎",
            Achievement::Utopia => "🌟",
            Achievement::Builder => "🔨",
            Achievement::MasterBuilder => "🏗️",
            Achievement::Scholar => "📚",
            Achievement::LegacyFounder => "🏛️",
            Achievement::WonderWorker => "✨",
            Achievement::DynastyRuler => "👑",
            Achievement::Remembered => "⭐",
            Achievement::AncestorWorship => "🕯️",
            Achievement::Dedicated => "⏰",
            Achievement::Veteran => "🎖️",
            Achievement::WinterSurvivor => "❄️",
        }
    }
}

/// Manages unlocked achievements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AchievementManager {
    /// Set of unlocked achievements
    pub unlocked: HashSet<Achievement>,
    /// Newly unlocked achievements (for notification display)
    #[serde(skip)]
    pub newly_unlocked: Vec<Achievement>,
}

impl AchievementManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Unlock an achievement if not already unlocked
    /// Returns true if newly unlocked
    pub fn unlock(&mut self, achievement: Achievement) -> bool {
        if self.unlocked.insert(achievement) {
            self.newly_unlocked.push(achievement);
            true
        } else {
            false
        }
    }
    
    /// Check if an achievement is unlocked
    pub fn is_unlocked(&self, achievement: Achievement) -> bool {
        self.unlocked.contains(&achievement)
    }
    
    /// Get count of unlocked achievements
    pub fn count(&self) -> usize {
        self.unlocked.len()
    }
    
    /// Get total number of achievements
    pub fn total() -> usize {
        20 // Update when adding new achievements
    }
    
    /// Pop a newly unlocked achievement for display
    pub fn pop_notification(&mut self) -> Option<Achievement> {
        self.newly_unlocked.pop()
    }
    
    /// Get all unlocked achievements as a sorted vec
    pub fn unlocked_list(&self) -> Vec<Achievement> {
        let mut list: Vec<_> = self.unlocked.iter().copied().collect();
        list.sort_by_key(|a| a.name());
        list
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
