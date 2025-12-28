//! Town proxy for abstract simulation of inactive towns

use serde::{Deserialize, Serialize};

/// A simplified representation of an inactive town for abstract simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownProxy {
    /// Town node ID this proxy represents
    pub town_id: u32,
    
    /// Net resource output per day (can be negative)
    pub net_materials: f32,
    pub net_food: f32,
    pub net_wood: f32,
    pub net_stone: f32,
    
    /// Current stockpile
    pub stockpile_materials: f32,
    pub stockpile_food: f32,
    pub stockpile_wood: f32,
    pub stockpile_stone: f32,
    
    /// Population count
    pub population: u32,
    
    /// Days since last full simulation
    pub days_archived: f32,
    
    /// Whether this town is in crisis (needs player attention)
    pub in_crisis: bool,
}

impl TownProxy {
    /// Create a proxy from full town data
    pub fn from_town_state(
        town_id: u32,
        population: u32,
        net_materials: f32,
        net_food: f32,
        net_wood: f32,
        net_stone: f32,
    ) -> Self {
        Self {
            town_id,
            net_materials,
            net_food,
            net_wood,
            net_stone,
            stockpile_materials: 0.0,
            stockpile_food: 0.0,
            stockpile_wood: 0.0,
            stockpile_stone: 0.0,
            population,
            days_archived: 0.0,
            in_crisis: false,
        }
    }
    
    /// Update proxy with time passage (called daily)
    pub fn update(&mut self, days: f32) {
        self.days_archived += days;
        
        // Apply net production
        self.stockpile_materials += self.net_materials * days;
        self.stockpile_food += self.net_food * days;
        self.stockpile_wood += self.net_wood * days;
        self.stockpile_stone += self.net_stone * days;
        
        // Check for crisis (stockpile went negative)
        self.in_crisis = self.stockpile_materials < -10.0 
            || self.stockpile_food < -10.0;
            
        // Clamp stockpiles (can't go too negative)
        self.stockpile_materials = self.stockpile_materials.max(-50.0);
        self.stockpile_food = self.stockpile_food.max(-50.0);
        self.stockpile_wood = self.stockpile_wood.max(-50.0);
        self.stockpile_stone = self.stockpile_stone.max(-50.0);
    }
    
    /// Check if town needs player intervention
    pub fn needs_attention(&self) -> bool {
        self.in_crisis || self.days_archived > 30.0
    }
    
    /// Get a status string
    pub fn status(&self) -> &'static str {
        if self.in_crisis {
            "Crisis!"
        } else if self.net_materials < 0.0 || self.net_food < 0.0 {
            "Struggling"
        } else {
            "Stable"
        }
    }
}

/// Collection of all archived towns
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TownProxyManager {
    /// All town proxies (excluding active town)
    proxies: Vec<TownProxy>,
}

impl TownProxyManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add or update a town proxy
    pub fn set(&mut self, proxy: TownProxy) {
        if let Some(existing) = self.proxies.iter_mut().find(|p| p.town_id == proxy.town_id) {
            *existing = proxy;
        } else {
            self.proxies.push(proxy);
        }
    }
    
    /// Remove a proxy (when activating a town)
    pub fn remove(&mut self, town_id: u32) -> Option<TownProxy> {
        if let Some(pos) = self.proxies.iter().position(|p| p.town_id == town_id) {
            Some(self.proxies.remove(pos))
        } else {
            None
        }
    }
    
    /// Get a proxy by town ID
    pub fn get(&self, town_id: u32) -> Option<&TownProxy> {
        self.proxies.iter().find(|p| p.town_id == town_id)
    }
    
    /// Update all proxies (called once per game day)
    pub fn update_all(&mut self, days: f32) {
        for proxy in &mut self.proxies {
            proxy.update(days);
        }
    }
    
    /// Get all proxies
    pub fn all(&self) -> &[TownProxy] {
        &self.proxies
    }
    
    /// Count towns in crisis
    pub fn crisis_count(&self) -> usize {
        self.proxies.iter().filter(|p| p.in_crisis).count()
    }
}
