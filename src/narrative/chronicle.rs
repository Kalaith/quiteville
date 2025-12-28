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
pub struct Chronicle {
    /// All recorded events
    events: Vec<ChronicleEvent>,
    /// Maximum events to store (oldest are dropped)
    #[serde(skip)]
    max_events: usize,
}

impl Chronicle {
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
