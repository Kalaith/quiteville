//! Scene management for switching between game views

use serde::{Deserialize, Serialize};

/// The different scenes/views in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scene {
    /// Main menu / title screen
    MainMenu,
    /// Town view - the primary gameplay scene
    TownView,
    /// Region/world map view - shows all towns
    RegionView,
    /// Loading screen between scenes
    Loading,
}

impl Default for Scene {
    fn default() -> Self {
        Scene::TownView
    }
}

/// Manages scene transitions and state
#[derive(Debug, Clone, Default)]
pub struct SceneManager {
    /// Current active scene
    pub current: Scene,
    /// Scene to transition to (if any)
    pending_transition: Option<Scene>,
    /// Transition progress (0.0 - 1.0)
    transition_progress: f32,
    /// Whether actively transitioning
    pub is_transitioning: bool,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            current: Scene::TownView,
            pending_transition: None,
            transition_progress: 0.0,
            is_transitioning: false,
        }
    }
    
    /// Request a scene change
    pub fn transition_to(&mut self, scene: Scene) {
        if scene != self.current && !self.is_transitioning {
            self.pending_transition = Some(scene);
            self.is_transitioning = true;
            self.transition_progress = 0.0;
        }
    }
    
    /// Toggle between town and region view
    pub fn toggle_region_view(&mut self) {
        match self.current {
            Scene::TownView => self.transition_to(Scene::RegionView),
            Scene::RegionView => self.transition_to(Scene::TownView),
            _ => {}
        }
    }
    
    /// Update transition progress
    /// Returns true when transition completes
    pub fn update(&mut self, delta: f32) -> bool {
        if !self.is_transitioning {
            return false;
        }
        
        // Fade speed
        self.transition_progress += delta * 2.0; // 0.5 seconds fade
        
        if self.transition_progress >= 1.0 {
            // Complete the transition
            if let Some(next) = self.pending_transition.take() {
                self.current = next;
            }
            self.is_transitioning = false;
            self.transition_progress = 0.0;
            return true;
        }
        
        false
    }
    
    /// Get the fade alpha for transition effects (0.0 = visible, 1.0 = black)
    pub fn fade_alpha(&self) -> f32 {
        if !self.is_transitioning {
            return 0.0;
        }
        
        // Fade out then fade in
        if self.transition_progress < 0.5 {
            self.transition_progress * 2.0 // 0 -> 1
        } else {
            (1.0 - self.transition_progress) * 2.0 // 1 -> 0
        }
    }
    
    /// Check if we're in town view
    pub fn in_town_view(&self) -> bool {
        self.current == Scene::TownView
    }
    
    /// Check if we're in region view
    pub fn in_region_view(&self) -> bool {
        self.current == Scene::RegionView
    }
}
