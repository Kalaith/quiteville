//! Inter-town trade system

use serde::{Deserialize, Serialize};
use macroquad::prelude::*;

/// A resource being transported between towns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeGood {
    Materials,
    Wood,
    Stone,
    Food,
}

impl TradeGood {
    pub fn name(&self) -> &'static str {
        match self {
            TradeGood::Materials => "Materials",
            TradeGood::Wood => "Wood",
            TradeGood::Stone => "Stone",
            TradeGood::Food => "Food",
        }
    }
}

/// A trade route between two towns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRoute {
    /// Unique ID
    pub id: u32,
    /// Source town ID
    pub from_town: u32,
    /// Destination town ID
    pub to_town: u32,
    /// What resource is being shipped
    pub good: TradeGood,
    /// Amount per shipment
    pub amount_per_trip: f32,
    /// Days between shipments
    pub frequency_days: f32,
    /// Whether this route is active
    pub active: bool,
}

impl TradeRoute {
    pub fn new(id: u32, from: u32, to: u32, good: TradeGood, amount: f32) -> Self {
        Self {
            id,
            from_town: from,
            to_town: to,
            good,
            amount_per_trip: amount,
            frequency_days: 3.0, // Default: every 3 days
            active: true,
        }
    }
}

/// A caravan traveling between towns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caravan {
    /// Unique ID
    pub id: u32,
    /// Trade route this caravan is on
    pub route_id: u32,
    /// Current position (0-1, lerp between towns)
    pub progress: f32,
    /// Whether traveling to destination (true) or returning (false)
    pub outbound: bool,
    /// Cargo being carried
    pub cargo: TradeGood,
    /// Amount of cargo
    pub cargo_amount: f32,
}

impl Caravan {
    pub fn new(id: u32, route_id: u32, cargo: TradeGood, amount: f32) -> Self {
        Self {
            id,
            route_id,
            progress: 0.0,
            outbound: true,
            cargo,
            cargo_amount: amount,
        }
    }
    
    /// Update caravan position
    /// Returns (arrived_at_destination, returned_home)
    pub fn update(&mut self, travel_time: f32, delta_days: f32) -> (bool, bool) {
        let speed = 1.0 / travel_time; // Full trip in travel_time days
        self.progress += speed * delta_days;
        
        if self.progress >= 1.0 {
            self.progress = 0.0;
            
            if self.outbound {
                // Arrived at destination
                self.outbound = false;
                self.cargo_amount = 0.0; // Unloaded
                return (true, false);
            } else {
                // Returned home
                self.outbound = true;
                return (false, true);
            }
        }
        
        (false, false)
    }
    
    /// Get visual position between two points
    pub fn get_visual_position(&self, from: Vec2, to: Vec2) -> Vec2 {
        let (start, end) = if self.outbound {
            (from, to)
        } else {
            (to, from)
        };
        
        start.lerp(end, self.progress)
    }
}

/// Manages all trade routes and caravans
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TradeManager {
    pub routes: Vec<TradeRoute>,
    pub caravans: Vec<Caravan>,
    next_route_id: u32,
    next_caravan_id: u32,
}

impl TradeManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new trade route
    pub fn add_route(&mut self, from: u32, to: u32, good: TradeGood, amount: f32) -> u32 {
        let id = self.next_route_id;
        self.next_route_id += 1;
        self.routes.push(TradeRoute::new(id, from, to, good, amount));
        id
    }
    
    /// Spawn a caravan for a route
    pub fn spawn_caravan(&mut self, route_id: u32) {
        if let Some(route) = self.routes.iter().find(|r| r.id == route_id) {
            let id = self.next_caravan_id;
            self.next_caravan_id += 1;
            self.caravans.push(Caravan::new(
                id, 
                route_id, 
                route.good, 
                route.amount_per_trip
            ));
        }
    }
    
    /// Get all routes from a specific town
    pub fn routes_from(&self, town_id: u32) -> Vec<&TradeRoute> {
        self.routes.iter()
            .filter(|r| r.from_town == town_id)
            .collect()
    }
    
    /// Get all routes to a specific town
    pub fn routes_to(&self, town_id: u32) -> Vec<&TradeRoute> {
        self.routes.iter()
            .filter(|r| r.to_town == town_id)
            .collect()
    }
    
    /// Count active caravans
    pub fn active_caravan_count(&self) -> usize {
        self.caravans.len()
    }
}
