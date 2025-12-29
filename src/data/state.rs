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
    
    /// Season and weather state
    #[serde(default)]
    pub season_state: crate::simulation::seasons::SeasonState,
    
    // UI State
    #[serde(skip)]
    pub show_tech_tree: bool,
    #[serde(skip)]
    pub show_build_menu: bool,
    #[serde(skip)]
    pub show_chronicle: bool,
    #[serde(skip)]
    pub zones_scroll_offset: f32,
    
    /// Milestones that have been achieved (by ID)
    pub achieved_milestones: Vec<String>,
    
    /// Currently selected entity
    #[serde(skip)]
    pub selection: Selection,
    
    /// Town history chronicle
    #[serde(default)]
    pub town_chronicle: crate::narrative::TownChronicle,

    // === Phase 4: Legacy & Dynasty ===
    
    /// Global dynasty record
    #[serde(default)]
    pub dynasty: crate::narrative::Dynasty,
    
    // === Phase 3: Regional Expansion ===
    
    /// Scene manager for view switching
    #[serde(skip)]
    pub scene_manager: crate::scene::SceneManager,
    
    /// Region/world map with all towns
    #[serde(default)]
    pub region_map: crate::region::RegionMap,
    
    /// Manager for archived (inactive) towns
    #[serde(default)]
    pub town_proxies: crate::region::TownProxyManager,
    
    /// Trade routes and caravans
    #[serde(default)]
    pub trade_manager: crate::region::TradeManager,
    
    /// Floating text notifications
    #[serde(skip)]
    pub floating_texts: crate::ui::floating_text::FloatingTextManager,

    // === Phase 5: Visuals ===
    
    /// Particle System (Weather, Smoke, FX)
    #[serde(skip)]
    pub particle_system: crate::ui::particles::ParticleSystem,
    
    // === Phase 5: Narrative ===
    #[serde(default)]
    pub tutorial: crate::narrative::tutorial::TutorialManager,
    
    // === Phase 6: Achievements & Stats ===
    
    /// Achievement tracking
    #[serde(default)]
    pub achievements: super::achievements::AchievementManager,
    
    /// Lifetime game statistics
    #[serde(default)]
    pub stats: super::achievements::GameStats,
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
            season_state: crate::simulation::seasons::SeasonState::default(),
            show_tech_tree: false,
            show_build_menu: false,
            show_chronicle: false,
            zones_scroll_offset: 0.0,
            achieved_milestones: Vec::new(),
            selection: Selection::None,
            town_chronicle: crate::narrative::TownChronicle::new(200),
            dynasty: crate::narrative::Dynasty::new(),
            // Phase 3 - Use procedural generation (uses generate_region from generation.rs)
            scene_manager: crate::scene::SceneManager::new(),
            region_map: crate::region::RegionMap::generate_procedural(12345, 6),
            town_proxies: crate::region::TownProxyManager::new(),
            trade_manager: crate::region::TradeManager::new(),
            floating_texts: crate::ui::floating_text::FloatingTextManager::new(),
            particle_system: crate::ui::particles::ParticleSystem::new(2000),
            tutorial: crate::narrative::tutorial::TutorialManager::new(),
            achievements: super::achievements::AchievementManager::new(),
            stats: super::achievements::GameStats::new(),
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
    
    /// Archive the current town as a proxy (uses TownProxy::from_town_state, TownProxyManager::set)
    pub fn archive_current_town(&mut self) {
        if let Some(town_id) = self.region_map.active_town_id {
            // Calculate net output based on current zones
            let net_materials = self.resources.materials * 0.1; // Simplified
            let net_food = self.resources.attractiveness * 0.05;
            
            let proxy = crate::region::TownProxy::from_town_state(
                town_id,
                self.agents.len() as u32,
                net_materials,
                net_food,
                0.0, // wood
                0.0, // stone
            );
            
            self.town_proxies.set(proxy);
            
            // Record in Dynasty
            let town_name = self.region_map.get_node(town_id).map(|n| n.name.clone()).unwrap_or("Unknown".to_string());
            let record = crate::narrative::TownRecord {
                name: town_name,
                timestamp: self.game_time_hours,
                population: self.agents.len() as u32,
                outcome: "Archived".to_string(),
                chronicle: self.town_chronicle.clone(),
            };
            self.dynasty.add_town_record(record);
            
            // Award Legacy Points (simplified for now)
            self.dynasty.add_legacy_points(10 + (self.agents.len() as u32 / 10));
            
            // Clear current chronicle for next town
            self.town_chronicle = crate::narrative::TownChronicle::new(200);
        }
    }
    
    /// Restore a town from proxy (uses TownProxyManager::get, remove)
    pub fn restore_town(&mut self, town_id: u32) -> bool {
        // Check if town exists in proxies
        if let Some(proxy) = self.town_proxies.get(town_id) {
            // Get accumulated resources before removing
            let _accumulated_materials = proxy.stockpile_materials;
            let _accumulated_food = proxy.stockpile_food;
        }
        
        // Remove from proxies
        if self.town_proxies.remove(town_id).is_some() {
            self.region_map.active_town_id = Some(town_id);
            return true;
        }
        
        false
    }
    
    /// Settle a new town (uses RegionMap::get_node_mut)
    pub fn settle_town(&mut self, town_id: u32) -> bool {
        if let Some(node) = self.region_map.get_node_mut(town_id) {
            if !node.settled {
                node.settled = true;
                self.log.add(
                    self.game_time_hours,
                    format!("Settled new town: {}", node.name),
                    crate::narrative::LogCategory::Event,
                );
                
                // Record in chronicle
                self.town_chronicle.record(
                    self.game_time_hours,
                    crate::narrative::ChronicleEventType::MilestoneAchieved { 
                        milestone_name: format!("Settled {}", node.name) 
                    },
                );
                return true;
            }
        }
        false
    }
    
    /// Switch to static starter map instead of procedural (uses generate_starter)
    pub fn use_static_map(&mut self, seed: u64) {
        self.region_map = crate::region::RegionMap::generate_starter(seed);
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
