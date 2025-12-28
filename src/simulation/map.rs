use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Grass,
    Dirt,
    Water,
    Floor,
    Wall,
    Ruins,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub kind: TileType,
    /// ID of the zone this tile belongs to, if any
    pub zone_id: Option<usize>, 
    /// Helps with rendering variations/connections
    pub variant: u8,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            kind: TileType::Grass,
            zone_id: None,
            variant: 0,
        }
    }
}

/// The game world grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
}

impl WorldMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::default(); width * height],
        }
    }
    
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y * self.width + x])
        } else {
            None
        }
    }
    
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < self.width && y < self.height {
            Some(&mut self.tiles[y * self.width + x])
        } else {
            None
        }
    }
    
    /// Set a rectangular area to a specific tile type (and optional zone)
    pub fn set_rect(&mut self, x: usize, y: usize, w: usize, h: usize, kind: TileType, zone_id: Option<usize>) {
        for dy in 0..h {
            for dx in 0..w {
                if let Some(tile) = self.get_tile_mut(x + dx, y + dy) {
                    tile.kind = kind;
                    tile.zone_id = zone_id;
                }
            }
        }
    }
}

impl Default for WorldMap {
    // Create a default small map for testing
    fn default() -> Self {
        let mut map = Self::new(50, 50);
        
        // Some decorative terrain
        map.set_rect(0, 0, 50, 50, TileType::Grass, None);
        map.set_rect(20, 0, 10, 50, TileType::Dirt, None); // Main road
        map.set_rect(0, 20, 50, 10, TileType::Dirt, None); // Cross road
        
        map
    }
}
