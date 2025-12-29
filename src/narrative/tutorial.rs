//! Tutorial system - Contextual hints and intro sequence

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Tutorial progression stages
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TutorialState {
    /// Opening cinematic and welcome
    Intro,
    /// Waiting for player to restore first zone
    WaitingFirstZone,
    /// First zone restored, waiting for agents
    WaitingAgents,
    /// Agents arrived, explain resources
    ResourceManagement,
    /// All basics covered
    Advanced,
    /// Tutorial completed or skipped
    Completed,
}

/// Contextual hint triggers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HintTrigger {
    FirstZoneRestored,
    FirstAgentSpawned,
    LowResources,
    FirstNight,
    FirstSeason,
    TechTreeOpened,
    ChronicleOpened,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogData {
    pub speaker: String,
    pub text: String,
    pub is_modal: bool,
}

/// Tutorial context passed from game state
pub struct TutorialContext {
    pub zones_active: usize,
    pub agent_count: usize,
    pub materials: f32,
    pub game_hour: f32,
    pub day: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialManager {
    pub state: TutorialState,
    pub active_dialog: Option<DialogData>,
    pub camera_intro_progress: f32,
    pub shown_hints: HashSet<HintTrigger>,
    pub skipped: bool,
    /// Track previous counts to detect changes
    prev_zones: usize,
    prev_agents: usize,
    prev_day: u32,
}

impl Default for TutorialManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TutorialManager {
    pub fn new() -> Self {
        Self {
            state: TutorialState::Intro,
            active_dialog: None,
            camera_intro_progress: 0.0,
            shown_hints: HashSet::new(),
            skipped: false,
            prev_zones: 0,
            prev_agents: 0,
            prev_day: 1,
        }
    }
    
    /// Skip the tutorial entirely
    pub fn skip_tutorial(&mut self) {
        self.skipped = true;
        self.state = TutorialState::Completed;
        self.active_dialog = None;
    }
    
    /// Check if tutorial is complete or skipped
    pub fn is_complete(&self) -> bool {
        self.skipped || self.state == TutorialState::Completed
    }

    /// Main update - check triggers and show dialogs
    pub fn update(&mut self, ctx: TutorialContext, delta: f32) {
        if self.is_complete() {
            return;
        }
        
        // Don't trigger new dialogs if one is active
        if self.active_dialog.is_some() {
            return;
        }
        
        match self.state {
            TutorialState::Intro => {
                // Advance camera intro
                if self.camera_intro_progress < 1.0 {
                    self.camera_intro_progress += delta * 0.2; // 5 seconds total
                    if self.camera_intro_progress >= 1.0 {
                        self.camera_intro_progress = 1.0;
                        // Show welcome dialog after camera pans
                        self.show_dialog(
                            "Uncle Artie",
                            "Welcome back to Quiteville, kid. Or what's left of it... Your granddad built this place from nothing. Now it's your turn to bring it back.",
                            true
                        );
                    }
                }
            }
            
            TutorialState::WaitingFirstZone => {
                // Check if player restored a zone
                if ctx.zones_active > self.prev_zones && !self.shown_hints.contains(&HintTrigger::FirstZoneRestored) {
                    self.shown_hints.insert(HintTrigger::FirstZoneRestored);
                    self.show_dialog(
                        "Uncle Artie",
                        "That's the spirit! Each restored building brings new life to the town. Keep an eye on your materials though - we'll need more to keep going.",
                        false
                    );
                    self.state = TutorialState::WaitingAgents;
                }
            }
            
            TutorialState::WaitingAgents => {
                // Check if first agent spawned
                if ctx.agent_count > 0 && !self.shown_hints.contains(&HintTrigger::FirstAgentSpawned) {
                    self.shown_hints.insert(HintTrigger::FirstAgentSpawned);
                    self.show_dialog(
                        "Uncle Artie",
                        "Would you look at that! Someone moved in! These folks have their own needs - food, rest, something to do. Click on 'em to learn more.",
                        false
                    );
                    self.state = TutorialState::ResourceManagement;
                }
            }
            
            TutorialState::ResourceManagement => {
                // Check for low resources warning
                if ctx.materials < 5.0 && !self.shown_hints.contains(&HintTrigger::LowResources) {
                    self.shown_hints.insert(HintTrigger::LowResources);
                    self.show_dialog(
                        "Uncle Artie",
                        "Running low on materials, eh? The workshop scavenges for more each day. Time is on our side - this town rebuilds itself slowly.",
                        false
                    );
                }
                
                // Check for first night
                if ctx.game_hour >= 20.0 && !self.shown_hints.contains(&HintTrigger::FirstNight) {
                    self.shown_hints.insert(HintTrigger::FirstNight);
                    self.show_dialog(
                        "Uncle Artie", 
                        "Night's falling. The villagers will head home to rest. Good time to plan your next moves. Press [R] to see what you can research.",
                        false
                    );
                    self.state = TutorialState::Advanced;
                }
            }
            
            TutorialState::Advanced => {
                // All hints shown, mark complete
                if self.shown_hints.len() >= 4 {
                    self.state = TutorialState::Completed;
                }
            }
            
            TutorialState::Completed => {
                // Nothing to do
            }
        }
        
        // Update tracking
        self.prev_zones = ctx.zones_active;
        self.prev_agents = ctx.agent_count;
        self.prev_day = ctx.day;
    }
    
    pub fn show_dialog(&mut self, speaker: &str, text: &str, modal: bool) {
        self.active_dialog = Some(DialogData {
            speaker: speaker.to_string(),
            text: text.to_string(),
            is_modal: modal,
        });
    }
    
    pub fn dismiss_dialog(&mut self) {
        self.active_dialog = None;
        
        // Progress state after dismissing intro dialog
        if self.state == TutorialState::Intro && self.camera_intro_progress >= 1.0 {
            self.state = TutorialState::WaitingFirstZone;
        }
    }
    
    /// Trigger a specific hint if not shown yet
    pub fn trigger_hint(&mut self, trigger: HintTrigger) {
        if self.is_complete() || self.shown_hints.contains(&trigger) {
            return;
        }
        
        self.shown_hints.insert(trigger.clone());
        
        match trigger {
            HintTrigger::TechTreeOpened => {
                self.show_dialog(
                    "Uncle Artie",
                    "The tech tree shows all the improvements we can make. Each one costs materials but makes the town stronger.",
                    false
                );
            }
            HintTrigger::ChronicleOpened => {
                self.show_dialog(
                    "Uncle Artie",
                    "The Chronicle records our history. When a villager does something remarkable, you can immortalize them here for all time.",
                    false
                );
            }
            _ => {} // Other triggers handled in update()
        }
    }
}
