//! Game event log

use serde::{Deserialize, Serialize};

/// A single log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: f32,  // Game time in hours
    pub message: String,
    pub category: LogCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogCategory {
    System,
    Zone,
    Population,
    Milestone,
    Event,
}

/// The game log - narrative events displayed to player
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameLog {
    entries: Vec<LogEntry>,
    max_entries: usize,
}

impl GameLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn add(&mut self, timestamp: f32, message: String, category: LogCategory) {
        self.entries.push(LogEntry {
            timestamp,
            message,
            category,
        });
        
        // Trim old entries
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn recent(&self, count: usize) -> &[LogEntry] {
        let start = self.entries.len().saturating_sub(count);
        &self.entries[start..]
    }
}
