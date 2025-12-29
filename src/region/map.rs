//! Region map data structures

use serde::{Deserialize, Serialize};
use macroquad::prelude::*;

/// A node on the region map (town site or point of interest)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownNode {
    /// Unique identifier
    pub id: u32,
    /// Display name
    pub name: String,
    /// Position on world map (0-1 normalized)
    pub position: [f32; 2],
    /// Biome type
    pub biome: super::Biome,
    /// Whether this town has been settled
    pub settled: bool,
    /// Whether this is the current capital
    pub is_capital: bool,
    /// Resource potentials for this location
    pub resource_potentials: ResourcePotentials,
    /// Whether this is a wonder construction site
    pub is_wonder_site: bool,
    /// Active wonder construction (if any)
    pub wonder_site: Option<crate::narrative::WonderSite>,
}

impl TownNode {
    pub fn new(id: u32, name: &str, x: f32, y: f32, biome: super::Biome) -> Self {
        Self {
            id,
            name: name.to_string(),
            position: [x, y],
            biome,
            settled: false,
            is_capital: false,
            resource_potentials: ResourcePotentials::default(),
            is_wonder_site: false,
            wonder_site: None,
        }
    }
    
    /// Get position as Vec2
    pub fn pos(&self) -> Vec2 {
        vec2(self.position[0], self.position[1])
    }
}

/// Resource gathering bonuses for a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourcePotentials {
    /// Wood gathering multiplier
    pub wood: f32,
    /// Stone gathering multiplier  
    pub stone: f32,
    /// Food production multiplier
    pub food: f32,
    /// Trade income multiplier
    pub trade: f32,
}

impl ResourcePotentials {
    pub fn new(wood: f32, stone: f32, food: f32, trade: f32) -> Self {
        Self { wood, stone, food, trade }
    }
}

/// A route between two towns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    /// Source node ID
    pub from: u32,
    /// Destination node ID
    pub to: u32,
    /// Whether this route has been discovered/built
    pub discovered: bool,
    /// Road quality (affects travel speed) 0.0 = trail, 1.0 = paved
    pub quality: f32,
    /// Base travel time in game days
    pub base_travel_time: f32,
}

impl Route {
    pub fn new(from: u32, to: u32) -> Self {
        Self {
            from,
            to,
            discovered: false,
            quality: 0.0,
            base_travel_time: 3.0, // 3 days by default
        }
    }
    
    /// Get actual travel time based on road quality
    pub fn travel_time(&self) -> f32 {
        // Better roads = faster travel
        self.base_travel_time * (1.0 - self.quality * 0.5)
    }
}

/// The region/world map containing all town nodes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegionMap {
    /// All town nodes
    pub nodes: Vec<TownNode>,
    /// All routes between nodes
    pub routes: Vec<Route>,
    /// Currently selected node (for UI)
    #[serde(skip)]
    pub selected_node: Option<u32>,
    /// ID of the active town the player is currently in
    pub active_town_id: Option<u32>,
    /// Seed used for procedural generation
    pub seed: u64,
}

impl RegionMap {
    pub fn new(seed: u64) -> Self {
        Self {
            nodes: Vec::new(),
            routes: Vec::new(),
            selected_node: None,
            active_town_id: None,
            seed,
        }
    }
    
    /// Create a starter region with initial towns
    pub fn generate_starter(seed: u64) -> Self {
        use super::Biome;
        
        let mut map = Self::new(seed);
        
        // Starting town (center)
        let mut start = TownNode::new(0, "Quiteville", 0.5, 0.5, Biome::Plains);
        start.settled = true;
        start.is_capital = true;
        start.resource_potentials = ResourcePotentials::new(1.0, 1.0, 1.2, 1.0);
        map.nodes.push(start);
        
        // Nearby expansion sites
        map.nodes.push({
            let mut n = TownNode::new(1, "Pine Ridge", 0.3, 0.35, Biome::Forest);
            n.resource_potentials = ResourcePotentials::new(2.0, 0.5, 0.8, 0.5);
            n
        });
        
        map.nodes.push({
            let mut n = TownNode::new(2, "Stone's End", 0.7, 0.4, Biome::Mountains);
            n.resource_potentials = ResourcePotentials::new(0.5, 2.5, 0.3, 0.5);
            n
        });
        
        map.nodes.push({
            let mut n = TownNode::new(3, "Harbor Town", 0.6, 0.7, Biome::Coast);
            n.resource_potentials = ResourcePotentials::new(0.3, 0.5, 1.5, 2.0);
            n
        });
        
        map.nodes.push({
            let mut n = TownNode::new(4, "Dusty Flats", 0.25, 0.65, Biome::Desert);
            n.resource_potentials = ResourcePotentials::new(0.2, 1.5, 0.4, 0.8);
            n
        });
        
        // Wonder Sites (special locations for mega-projects)
        map.nodes.push({
            let mut n = TownNode::new(5, "Mystic Ruins", 0.15, 0.2, Biome::Forest);
            n.is_wonder_site = true;
            n.resource_potentials = ResourcePotentials::new(0.5, 0.5, 0.5, 0.5);
            n
        });
        
        map.nodes.push({
            let mut n = TownNode::new(6, "Ironpeak", 0.85, 0.25, Biome::Mountains);
            n.is_wonder_site = true;
            n.resource_potentials = ResourcePotentials::new(0.3, 2.0, 0.2, 0.3);
            n
        });
        
        map.nodes.push({
            let mut n = TownNode::new(7, "Celestial Summit", 0.5, 0.1, Biome::Mountains);
            n.is_wonder_site = true;
            n.resource_potentials = ResourcePotentials::new(0.1, 0.1, 0.1, 0.1);
            n
        });
        
        // Tundra biome town
        map.nodes.push({
            let mut n = TownNode::new(8, "Frostholm", 0.1, 0.15, Biome::Tundra);
            n.resource_potentials = ResourcePotentials::new(0.3, 0.8, 0.3, 1.5);
            n
        });
        
        // Swamp biome town
        map.nodes.push({
            let mut n = TownNode::new(9, "Marshwood", 0.8, 0.8, Biome::Swamp);
            n.resource_potentials = ResourcePotentials::new(0.8, 0.2, 1.5, 0.4);
            n
        });
        
        // Routes (all initially undiscovered except to first neighbor)
        let mut route_start = Route::new(0, 1);
        route_start.discovered = true;
        map.routes.push(route_start);
        
        map.routes.push(Route::new(0, 2));
        map.routes.push(Route::new(0, 3));
        map.routes.push(Route::new(1, 4));
        map.routes.push(Route::new(2, 3));
        
        // Routes to wonder sites
        map.routes.push(Route::new(1, 5)); // Pine Ridge to Mystic Ruins
        map.routes.push(Route::new(2, 6)); // Stone's End to Ironpeak
        map.routes.push(Route::new(0, 7)); // Quiteville to Celestial Summit
        
        // Routes to new biome towns
        map.routes.push(Route::new(5, 8)); // Mystic Ruins to Frostholm
        map.routes.push(Route::new(3, 9)); // Harbor Town to Marshwood
        
        map.active_town_id = Some(0);
        
        map
    }
    
    /// Get a node by ID
    pub fn get_node(&self, id: u32) -> Option<&TownNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
    
    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: u32) -> Option<&mut TownNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }
    
    /// Get routes connected to a node
    pub fn routes_from(&self, node_id: u32) -> Vec<&Route> {
        self.routes.iter()
            .filter(|r| r.from == node_id || r.to == node_id)
            .collect()
    }
    
    /// Get the active town node
    pub fn active_town(&self) -> Option<&TownNode> {
        self.active_town_id.and_then(|id| self.get_node(id))
    }
    
    /// Count settled towns
    pub fn settled_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.settled).count()
    }
    
    /// Generate a procedural region using the generation module
    pub fn generate_procedural(seed: u64, node_count: usize) -> Self {
        let config = super::GenerationConfig {
            seed,
            node_count,
            ..Default::default()
        };
        super::generate_region(&config)
    }
}
