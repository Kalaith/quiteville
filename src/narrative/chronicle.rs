//! Town chronicle for tracking history and events

use serde::{Deserialize, Serialize};

/// Types of events that can be recorded in the chronicle
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChronicleEventType {
    /// A new villager arrived
    VillagerArrived { name: String },
    /// A villager departed or died
    VillagerLost { name: String, reason: String },
    /// A building was constructed
    BuildingConstructed { building_name: String },
    /// A building was upgraded
    BuildingUpgraded { from: String, to: String },
    /// A technology was researched
    TechResearched { tech_name: String },
    /// A milestone was achieved
    MilestoneAchieved { milestone_name: String },
    /// Season changed
    SeasonChanged { season: String },
    /// A disaster occurred
    Disaster { description: String },
    /// Special event
    Special { description: String },
}

/// A single event in the town's history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleEvent {
    /// Game time when event occurred (in hours)
    pub timestamp: f32,
    /// The day number
    pub day: u32,
    /// Type of event
    pub event_type: ChronicleEventType,
}

impl ChronicleEvent {
    pub fn new(timestamp: f32, event_type: ChronicleEventType) -> Self {
        Self {
            timestamp,
            day: (timestamp / 24.0) as u32 + 1,
            event_type,
        }
    }
    
    /// Get a display string for this event
    pub fn display_text(&self) -> String {
        match &self.event_type {
            ChronicleEventType::VillagerArrived { name } => {
                format!("{} joined the town", name)
            },
            ChronicleEventType::VillagerLost { name, reason } => {
                format!("{} left: {}", name, reason)
            },
            ChronicleEventType::BuildingConstructed { building_name } => {
                format!("{} was built", building_name)
            },
            ChronicleEventType::BuildingUpgraded { from, to } => {
                format!("{} upgraded to {}", from, to)
            },
            ChronicleEventType::TechResearched { tech_name } => {
                format!("Discovered: {}", tech_name)
            },
            ChronicleEventType::MilestoneAchieved { milestone_name } => {
                format!("⭐ {}", milestone_name)
            },
            ChronicleEventType::SeasonChanged { season } => {
                format!("{} has begun", season)
            },
            ChronicleEventType::Disaster { description } => {
                format!("⚠ {}", description)
            },
            ChronicleEventType::Special { description } => {
                description.clone()
            },
        }
    }
}

/// The town's historical record
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TownChronicle {
    /// All recorded events
    events: Vec<ChronicleEvent>,
    /// Maximum events to store (oldest are dropped)
    #[serde(skip)]
    max_events: usize,
}

impl TownChronicle {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events,
        }
    }
    
    /// Record a new event
    pub fn record(&mut self, timestamp: f32, event_type: ChronicleEventType) {
        self.events.push(ChronicleEvent::new(timestamp, event_type));
        
        // Trim if too many
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }
    
    /// Get all events
    pub fn events(&self) -> &[ChronicleEvent] {
        &self.events
    }
    
    /// Get events from a specific day
    pub fn events_on_day(&self, day: u32) -> Vec<&ChronicleEvent> {
        self.events.iter().filter(|e| e.day == day).collect()
    }
    
    /// Get the most recent N events
    pub fn recent(&self, count: usize) -> Vec<&ChronicleEvent> {
        self.events.iter().rev().take(count).collect()
    }
    
    /// Get total number of events
    pub fn len(&self) -> usize {
        self.events.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// A simplified record of a past town
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownRecord {
    /// Name of the town
    pub name: String,
    /// When it was archived (game time)
    pub timestamp: f32,
    /// Final population
    pub population: u32,
    /// Reason it was archived (e.g., "Prospered", "Abandoned")
    pub outcome: String,
    /// Its full chronicle (optional, to save space we might summarize later)
    pub chronicle: TownChronicle,
}

/// A record of a notable villager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VillagerRecord {
    pub name: String,
    pub description: String,
    pub feats: Vec<String>,
    pub timestamp_added: f32,
}

/// Buff types granted by ancestral spirits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AncestorBuff {
    /// +10% production
    ProductionBoost,
    /// +10% morale/spirit
    MoraleBoost,
    /// +5% luck on random events
    LuckBoost,
    /// +5% population growth
    GrowthBoost,
}

impl AncestorBuff {
    pub fn name(&self) -> &'static str {
        match self {
            AncestorBuff::ProductionBoost => "Production Blessing",
            AncestorBuff::MoraleBoost => "Spirit of Joy",
            AncestorBuff::LuckBoost => "Fortune's Favor",
            AncestorBuff::GrowthBoost => "Blessing of Fertility",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            AncestorBuff::ProductionBoost => "+10% resource production",
            AncestorBuff::MoraleBoost => "+10% villager morale",
            AncestorBuff::LuckBoost => "+5% positive event chance",
            AncestorBuff::GrowthBoost => "+5% population growth",
        }
    }
}

/// A retired hero who provides buffs from beyond
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestorSpirit {
    /// The original hero record
    pub hero: VillagerRecord,
    /// What buff this ancestor provides
    pub buff: AncestorBuff,
    /// When they were retired
    pub retired_at: f32,
}

impl AncestorSpirit {
    pub fn new(hero: VillagerRecord, buff: AncestorBuff, game_time: f32) -> Self {
        Self {
            hero,
            buff,
            retired_at: game_time,
        }
    }
}

/// Global progress tracking across multiple towns
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dynasty {
    /// Global currency for unlocking meta-upgrades
    pub legacy_points: u32,
    /// History of all past towns
    pub past_towns: Vec<TownRecord>,
    /// Collection of notable villagers (not yet retired)
    pub hall_of_heroes: Vec<VillagerRecord>,
    /// Retired heroes who provide buffs
    pub ancestors: Vec<AncestorSpirit>,
    /// Completed wonders
    pub completed_wonders: Vec<super::wonders::Wonder>,
}

impl Dynasty {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_legacy_points(&mut self, amount: u32) {
        self.legacy_points += amount;
    }
    
    pub fn add_town_record(&mut self, record: TownRecord) {
        self.past_towns.push(record);
    }
    
    pub fn add_hero(&mut self, hero: VillagerRecord) {
        self.hall_of_heroes.push(hero);
    }
    
    /// Retire a hero from the Hall of Heroes to become an ancestor
    pub fn retire_hero(&mut self, hero_name: &str, game_time: f32) -> Option<AncestorBuff> {
        // Find and remove from hall of heroes
        let pos = self.hall_of_heroes.iter().position(|h| h.name == hero_name)?;
        let hero = self.hall_of_heroes.remove(pos);
        
        // Determine buff based on hero's feats
        let buff = if hero.feats.iter().any(|f| f.contains("build")) {
            AncestorBuff::ProductionBoost
        } else if hero.feats.iter().any(|f| f.contains("social")) {
            AncestorBuff::MoraleBoost
        } else if hero.feats.iter().any(|f| f.contains("Lived")) {
            AncestorBuff::GrowthBoost
        } else {
            AncestorBuff::LuckBoost
        };
        
        let spirit = AncestorSpirit::new(hero, buff, game_time);
        self.ancestors.push(spirit);
        
        Some(buff)
    }
    
    /// Add a completed wonder
    pub fn add_wonder(&mut self, wonder: super::wonders::Wonder) {
        if !self.completed_wonders.contains(&wonder) {
            self.completed_wonders.push(wonder);
        }
    }
    
    /// Calculate total ancestor buffs
    pub fn ancestor_buffs(&self) -> AncestorBuffTotals {
        let mut totals = AncestorBuffTotals::default();
        
        for ancestor in &self.ancestors {
            match ancestor.buff {
                AncestorBuff::ProductionBoost => totals.production += 0.10,
                AncestorBuff::MoraleBoost => totals.morale += 0.10,
                AncestorBuff::LuckBoost => totals.luck += 0.05,
                AncestorBuff::GrowthBoost => totals.growth += 0.05,
            }
        }
        
        totals
    }
}

/// Accumulated ancestor buff percentages
#[derive(Debug, Clone, Default)]
pub struct AncestorBuffTotals {
    pub production: f32,
    pub morale: f32,
    pub luck: f32,
    pub growth: f32,
}
