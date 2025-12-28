//! Game state - The root state struct containing all game data

use serde::{Deserialize, Serialize};
use crate::data::{GameConfig, ZoneTemplate, Milestone};
use crate::economy::Resources;
use crate::zones::Zone;
use crate::population::PopulationPressure;
use crate::narrative::GameLog;

/// The root game state - everything the game needs to run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Global game configuration (loaded from JSON)
    #[serde(skip)]
    pub config: GameConfig,
    
    /// Zone templates (loaded from JSON)
    #[serde(skip)]
    pub zone_templates: Vec<ZoneTemplate>,
    
    /// Milestone definitions (loaded from JSON) 
    #[serde(skip)]
    pub milestones: Vec<Milestone>,
    
    /// Loaded Game Assets (Textures)
    #[serde(skip)]
    pub assets: crate::assets::GameAssets,
    
    /// Tech Tree
    pub tech_tree: Vec<crate::data::TechNode>,
    
    /// Current resources
    pub resources: Resources,
    
    /// Population pressure
    pub population: PopulationPressure,
    
    /// Active zones
    pub zones: Vec<Zone>,
    
    /// The 2D World Map
    pub world_map: crate::simulation::map::WorldMap,
    /// Viewport camera (stored in state to persist position)
    #[serde(skip)] // Don't serialize camera for now
    pub camera: crate::simulation::camera::Camera2D,
    
    /// Active agents (villagers)
    #[serde(skip)]
    pub agents: Vec<crate::simulation::agents::Agent>,
    
    /// Game log
    pub log: GameLog,
    
    /// Total game time in hours
    pub game_time_hours: f32,
    
    /// Current game hour (0-24 cycle for day/night)
    #[serde(default)]
    pub game_hour: f32,
    
    // UI State
    #[serde(skip)]
    pub show_tech_tree: bool,
    #[serde(skip)]
    pub show_build_menu: bool,
    #[serde(skip)]
    pub zones_scroll_offset: f32,
    
    /// Milestones that have been achieved (by ID)
    pub achieved_milestones: Vec<String>,
    
    /// Currently selected entity
    #[serde(skip)]
    pub selection: Selection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    None,
    Zone(usize),
    Agent(u64),
}

impl Default for Selection {
    fn default() -> Self {
        Self::None
    }
}

impl GameState {
    /// Create a new game state with default values
    pub fn new(
        config: GameConfig, 
        zone_templates: Vec<ZoneTemplate>, 
        milestones: Vec<Milestone>,
        assets: crate::assets::GameAssets
    ) -> Self {
        let resources = Resources::new(
            config.starting_resources.materials,
            config.starting_resources.maintenance,
            config.starting_resources.attractiveness,
            config.starting_resources.stability,
        );
        
        Self {
            config,
            zone_templates,
            milestones,
            assets,
            tech_tree: crate::data::default_tech_tree(),
            resources,
            population: PopulationPressure::default(),
            zones: Vec::new(),
            world_map: crate::simulation::map::WorldMap::default(),
            camera: crate::simulation::camera::Camera2D::new(),
            agents: Vec::new(),
            log: GameLog::new(100),
            game_time_hours: 0.0,
            game_hour: 8.0, // Start at 8 AM
            show_tech_tree: false,
            show_build_menu: false,
            zones_scroll_offset: 0.0,
            achieved_milestones: Vec::new(),
            selection: Selection::None,
        }
    }

    /// Get a zone template by ID
    pub fn get_template(&self, id: &str) -> Option<&ZoneTemplate> {
        self.zone_templates.iter().find(|t| t.id == id)
    }

    /// Add a new zone from template (starts DORMANT - player must restore it)
    pub fn add_zone(&mut self, template_id: &str) -> bool {
        if self.get_template(template_id).is_some() {
            self.zones.push(Zone::new(template_id));
            true
        } else {
            false
        }
    }

    /// Calculate effective population using config K value
    pub fn effective_population(&self) -> f32 {
        crate::economy::effective_population(
            self.population.value(),
            self.config.population_k,
        )
    }

    /// Calculate total output from all zones
    pub fn calculate_total_output(&self) -> f32 {
        let mut total = 0.0;
        for zone in &self.zones {
            if let Some(template) = self.get_template(&zone.template_id) {
                let throughput = zone.calculate_throughput(template);
                let output = crate::economy::calculate_output(throughput, &self.resources);
                total += output;
            }
        }
        total
    }

    /// Calculate total maintenance cost
    pub fn calculate_maintenance_cost(&self) -> f32 {
        crate::economy::maintenance_cost(
            self.effective_population(),
            self.config.maintenance_cost_coefficient,
        )
    }
    /// Calculate total housing capacity from active zones
    pub fn calculate_housing_capacity(&self) -> f32 {
        let mut total = 0.0;
        for zone in &self.zones {
            if !zone.dormant {
                if let Some(template) = self.zone_templates.iter().find(|t| t.id == zone.template_id) {
                    // Capacity scales with condition? Yes, ruined houses hold fewer people.
                    total += template.population.capacity * zone.condition;
                }
            }
        }
        total
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(
            GameConfig::default(),
            Vec::new(),
            Vec::new(),
            crate::assets::GameAssets::default(),
        )
    }
}
