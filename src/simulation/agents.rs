use macroquad::prelude::*;
use macroquad::rand;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AgentState {
    Idle,
    Wandering { target: Vec2 },
    Working { target: Vec2, duration: f32 },
    Shopping { target: Vec2, duration: f32 },
    Socializing { target: Vec2, duration: f32 },
    GoingHome,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: u64,
    pub pos: Vec2,
    pub state: AgentState,
    
    // Needs (0.0 - 1.0, where 1.0 is Full/Good)
    pub energy: f32,
    pub hunger: f32,
    pub social: f32,
    
    // Personality / Stats
    pub speed: f32,
    pub color: [f32; 4], // RGBA
}

impl Agent {
    pub fn new(id: u64, pos: Vec2) -> Self {
        Self {
            id,
            pos,
            state: AgentState::Idle,
            energy: 1.0,
            hunger: 1.0,
            social: 1.0,
            speed: 60.0 + rand::gen_range(-15.0, 15.0),
            color: [
                rand::gen_range(0.5, 1.0),
                rand::gen_range(0.5, 1.0),
                rand::gen_range(0.5, 1.0),
                1.0
            ],
        }
    }
    
    pub fn update(&mut self, delta: f32, world: &WorldInfo) {
        // Needs Decay
        let decay_rate = 0.05 * delta;
        self.energy = (self.energy - decay_rate * 0.5).max(0.0);
        self.hunger = (self.hunger - decay_rate).max(0.0);
        self.social = (self.social - decay_rate * 0.8).max(0.0);
        
        match self.state {
            AgentState::Idle => {
                // Decision Tree
                if self.energy < 0.2 {
                    self.state = AgentState::GoingHome;
                } else if self.hunger < 0.3 && !world.markets.is_empty() {
                    // Go Shopping
                    let target = self.find_nearest(world.markets.as_slice());
                    self.state = AgentState::Wandering { target };
                    // After wandering, we will switch to Shopping? 
                    // Need a way to pass "Next State". For now, logic in Wandering arrival.
                } else if self.social < 0.4 && !world.parks.is_empty() {
                    let target = self.find_nearest(world.parks.as_slice());
                    self.state = AgentState::Wandering { target };
                } else if rand::gen_range(0, 100) < 1 { // 1% chance to Work
                     if !world.workshops.is_empty() {
                         let target = self.find_nearest(world.workshops.as_slice());
                         self.state = AgentState::Wandering { target };
                     }
                } else if rand::gen_range(0, 100) < 2 {
                    // Random wander
                    let target = self.pick_random_target();
                    self.state = AgentState::Wandering { target };
                }
            },
            AgentState::Wandering { target } => {
                let dist = self.pos.distance(target);
                if dist < 10.0 {
                    // Arrived! Determine what we are doing based on location
                    self.state = AgentState::Idle; // Default
                    
                    // Check local zones
                    if self.is_at_location(target, world.markets.as_slice()) && self.hunger < 0.5 {
                        self.state = AgentState::Shopping { target, duration: 2.0 };
                    } else if self.is_at_location(target, world.parks.as_slice()) && self.social < 0.5 {
                        self.state = AgentState::Socializing { target, duration: 3.0 };
                    } else if self.is_at_location(target, world.workshops.as_slice()) {
                         self.state = AgentState::Working { target, duration: 5.0 };
                    }
                } else {
                    let dir = (target - self.pos).normalize();
                    self.pos += dir * self.speed * delta;
                }
            },
            AgentState::Shopping { ref mut duration, .. } => {
                *duration -= delta;
                if *duration <= 0.0 {
                    self.hunger = 1.0; // Fed!
                    self.state = AgentState::Idle;
                }
            },
            AgentState::Socializing { ref mut duration, .. } => {
                *duration -= delta;
                if *duration <= 0.0 {
                    self.social = 1.0; // Happy!
                    self.state = AgentState::Idle;
                }
            },
            AgentState::Working { ref mut duration, .. } => {
                *duration -= delta;
                if *duration <= 0.0 {
                    // Earned money? For now just done
                    self.state = AgentState::Idle;
                }
            },
            AgentState::GoingHome => {
                self.energy += delta * 0.5;
                if self.energy >= 1.0 {
                    self.state = AgentState::Idle;
                }
            }
        }
        
        // Bounds
        self.pos.x = self.pos.x.clamp(0.0, 50.0 * 32.0);
        self.pos.y = self.pos.y.clamp(0.0, 50.0 * 32.0);
    }
    
    fn find_nearest(&self, targets: &[Vec2]) -> Vec2 {
        let mut nearest = targets[0];
        let mut min_dist = f32::MAX;
        for &t in targets {
            let d = self.pos.distance(t);
            if d < min_dist {
                min_dist = d;
                nearest = t;
            }
        }
        nearest
    }
    
    fn is_at_location(&self, target: Vec2, list: &[Vec2]) -> bool {
        list.iter().any(|&pos| pos.distance(target) < 1.0)
    }
    
    fn pick_random_target(&self) -> Vec2 {
        vec2(
            rand::gen_range(100.0, 1500.0),
            rand::gen_range(100.0, 1500.0)
        )
    }
}

/// Context for agent decisions
pub struct WorldInfo {
    pub markets: Vec<Vec2>,
    pub workshops: Vec<Vec2>,
    pub parks: Vec<Vec2>,
}
