//! Procedural generation for region maps

use super::{Biome, TownNode, Route, RegionMap, ResourcePotentials};

/// Configuration for map generation
#[derive(Debug, Clone)]
pub struct GenerationConfig {
    /// Random seed
    pub seed: u64,
    /// Number of nodes to generate
    pub node_count: usize,
    /// Minimum distance between nodes (0-1)
    pub min_node_distance: f32,
    /// Margin from map edges (0-1)
    pub edge_margin: f32,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            node_count: 8,
            min_node_distance: 0.15,
            edge_margin: 0.1,
        }
    }
}

/// Simple pseudo-random number generator using seed
struct SeededRng {
    state: u64,
}

impl SeededRng {
    fn new(seed: u64) -> Self {
        Self { state: seed.wrapping_add(1) }
    }
    
    fn next(&mut self) -> f32 {
        // Simple LCG
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.state >> 16) & 0x7FFF) as f32 / 32767.0
    }
    
    fn range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next() * (max - min)
    }
    
    fn range_int(&mut self, min: usize, max: usize) -> usize {
        min + (self.next() * (max - min) as f32) as usize
    }
}

/// Generate a procedural region map
pub fn generate_region(config: &GenerationConfig) -> RegionMap {
    let mut rng = SeededRng::new(config.seed);
    let mut map = RegionMap::new(config.seed);
    
    // Generate node positions using Poisson disk-like sampling
    let mut positions: Vec<[f32; 2]> = Vec::new();
    
    // Always place starting town near center
    let start_x = rng.range(0.45, 0.55);
    let start_y = rng.range(0.45, 0.55);
    positions.push([start_x, start_y]);
    
    // Try to place remaining nodes
    let max_attempts = 100;
    while positions.len() < config.node_count {
        let mut best_pos: Option<[f32; 2]> = None;
        let mut best_dist = 0.0f32;
        
        for _ in 0..max_attempts {
            let x = rng.range(config.edge_margin, 1.0 - config.edge_margin);
            let y = rng.range(config.edge_margin, 1.0 - config.edge_margin);
            
            // Find minimum distance to existing nodes
            let min_dist = positions.iter()
                .map(|p| ((p[0] - x).powi(2) + (p[1] - y).powi(2)).sqrt())
                .fold(f32::MAX, f32::min);
            
            if min_dist >= config.min_node_distance && min_dist > best_dist {
                best_pos = Some([x, y]);
                best_dist = min_dist;
            }
        }
        
        if let Some(pos) = best_pos {
            positions.push(pos);
        } else {
            break; // Can't fit more nodes
        }
    }
    
    // Generate nodes with biomes and names
    let town_names = [
        "Willowdale", "Stonehaven", "Pine Valley", "Dust Bowl",
        "Harbor Point", "Eagle Peak", "Riverside", "Green Meadow",
        "Crystal Lake", "Shadow Glen", "Boulder Ridge", "Sunny Fields",
    ];
    
    for (i, pos) in positions.iter().enumerate() {
        // Determine biome based on position
        let biome = position_to_biome(pos[0], pos[1], &mut rng);
        
        // Pick name
        let name = if i == 0 {
            "Quiteville" // Starting town
        } else if i < town_names.len() {
            town_names[i]
        } else {
            "Unknown Town"
        };
        
        let mut node = TownNode::new(i as u32, name, pos[0], pos[1], biome);
        
        // Set resource potentials based on biome
        node.resource_potentials = ResourcePotentials::new(
            biome.wood_multiplier() * rng.range(0.8, 1.2),
            biome.stone_multiplier() * rng.range(0.8, 1.2),
            biome.food_multiplier() * rng.range(0.8, 1.2),
            biome.trade_multiplier() * rng.range(0.8, 1.2),
        );
        
        // Starting town is settled and capital
        if i == 0 {
            node.settled = true;
            node.is_capital = true;
        }
        
        map.nodes.push(node);
    }
    
    // Generate routes using Delaunay-like connectivity
    // Connect each node to its nearest neighbors
    for i in 0..map.nodes.len() {
        let pos_i = map.nodes[i].pos();
        
        // Find 2-3 nearest neighbors
        let mut distances: Vec<(usize, f32)> = map.nodes.iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(j, n)| (j, pos_i.distance(n.pos())))
            .collect();
        
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        let neighbor_count = rng.range_int(2, 4).min(distances.len());
        
        for (j, _) in distances.iter().take(neighbor_count) {
            // Only add route if it doesn't exist
            let from = i as u32;
            let to = *j as u32;
            let (from, to) = if from < to { (from, to) } else { (to, from) };
            
            if !map.routes.iter().any(|r| r.from == from && r.to == to) {
                let mut route = Route::new(from, to);
                
                // Starting town routes are discovered
                if from == 0 || to == 0 {
                    route.discovered = true;
                }
                
                map.routes.push(route);
            }
        }
    }
    
    map.active_town_id = Some(0);
    map
}

/// Determine biome based on position (creates region-like clusters)
fn position_to_biome(x: f32, y: f32, rng: &mut SeededRng) -> Biome {
    // Use position to create natural biome regions
    // This creates a simple biome pattern
    
    let noise = rng.next() * 0.3; // Add randomness
    
    // Coast on edges
    if x < 0.15 || x > 0.85 || y > 0.85 {
        if rng.next() > 0.3 {
            return Biome::Coast;
        }
    }
    
    // Mountains in upper region
    if y < 0.3 + noise {
        if rng.next() > 0.4 {
            return Biome::Mountains;
        }
    }
    
    // Desert in lower-left
    if x < 0.4 && y > 0.5 + noise {
        if rng.next() > 0.4 {
            return Biome::Desert;
        }
    }
    
    // Forest in upper-left
    if x < 0.5 && y < 0.5 {
        if rng.next() > 0.5 {
            return Biome::Forest;
        }
    }
    
    // Default to plains
    Biome::Plains
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generation() {
        let config = GenerationConfig {
            seed: 12345,
            node_count: 6,
            ..Default::default()
        };
        
        let map = generate_region(&config);
        
        assert!(map.nodes.len() >= 1); // At least starting town
        assert!(!map.routes.is_empty());
        assert!(map.active_town_id.is_some());
    }
}
