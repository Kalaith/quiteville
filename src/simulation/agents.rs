use macroquad::prelude::*;
use macroquad::rand;
use serde::{Deserialize, Serialize};

/// Job roles for agents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Job {
    Laborer,   // Default - general work
    Farmer,    // Works at farms
    Cook,      // Works at markets/kitchens
    Scavenger, // Works at workshops
    Builder,   // Constructs buildings
    Hauler,    // Transports resources between buildings
}

impl Default for Job {
    fn default() -> Self {
        Self::Laborer
    }
}

impl Job {
    pub fn name(&self) -> &'static str {
        match self {
            Job::Laborer => "Laborer",
            Job::Farmer => "Farmer",
            Job::Cook => "Cook",
            Job::Scavenger => "Scavenger",
            Job::Builder => "Builder",
            Job::Hauler => "Hauler",
        }
    }
}

/// Time of day for agent schedules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeOfDay {
    Morning,   // 6:00 - 9:00
    Work,      // 9:00 - 17:00
    Evening,   // 17:00 - 22:00
    Night,     // 22:00 - 6:00
}

impl TimeOfDay {
    pub fn from_hour(hour: f32) -> Self {
        let h = hour % 24.0;
        if h >= 6.0 && h < 9.0 {
            TimeOfDay::Morning
        } else if h >= 9.0 && h < 17.0 {
            TimeOfDay::Work
        } else if h >= 17.0 && h < 22.0 {
            TimeOfDay::Evening
        } else {
            TimeOfDay::Night
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AgentState {
    Idle,
    Wandering { target: Vec2 },
    Working { target: Vec2, duration: f32 },
    Shopping { target: Vec2, duration: f32 },
    Socializing { target: Vec2, duration: f32 },
    GoingHome,
    Sleeping,
    Building { target: Vec2, zone_idx: usize },
}

/// Track agent accomplishments for Hall of Heroes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentFeats {
    pub buildings_helped: u32,
    pub resources_hauled: u32,
    pub days_lived: u32,
    pub social_events: u32,
}

impl AgentFeats {
    pub fn to_strings(&self) -> Vec<String> {
        let mut feats = Vec::new();
        if self.buildings_helped > 0 {
            feats.push(format!("Helped build {} structures", self.buildings_helped));
        }
        if self.resources_hauled > 0 {
            feats.push(format!("Hauled {} resources", self.resources_hauled));
        }
        if self.days_lived >= 30 {
            feats.push(format!("Lived {} days in town", self.days_lived));
        }
        if self.social_events >= 10 {
            feats.push(format!("Attended {} social gatherings", self.social_events));
        }
        feats
    }
}

/// Random name pool for villagers
const FIRST_NAMES: &[&str] = &[
    "Ada", "Ben", "Clara", "Dex", "Ella", "Finn", "Grace", "Hugo",
    "Ivy", "Jack", "Kate", "Leo", "Mia", "Nate", "Olive", "Pete",
    "Quinn", "Rose", "Sam", "Tess", "Uma", "Vic", "Wren", "Xander",
    "Yuki", "Zoe", "Abel", "Beth", "Cora", "Dale", "Erin", "Garth"
];

const LAST_NAMES: &[&str] = &[
    "Miller", "Smith", "Baker", "Cooper", "Fisher", "Carter", "Mason",
    "Potter", "Weaver", "Hunter", "Thorne", "Brook", "Stone", "Wood",
    "Field", "Hill", "Lake", "Marsh", "Vale", "Glen", "Frost", "Bloom"
];

fn generate_random_name() -> String {
    let first = FIRST_NAMES[rand::gen_range(0, FIRST_NAMES.len())];
    let last = LAST_NAMES[rand::gen_range(0, LAST_NAMES.len())];
    format!("{} {}", first, last)
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: u64,
    pub name: String,
    pub pos: Vec2,
    pub state: AgentState,
    
    // Needs (0.0 - 1.0, where 1.0 is Full/Good)
    pub energy: f32,
    pub hunger: f32,
    pub social: f32,
    pub spirit: f32, // Hope/Morale
    
    // Job and workplace
    pub job: Job,
    pub home_pos: Vec2,
    
    // Personality / Stats
    pub speed: f32,
    pub color: [f32; 4], // RGBA
    pub traits: Vec<crate::simulation::traits::Trait>,
    
    // Accomplishments for Hall of Heroes
    pub feats: AgentFeats,
}

impl Agent {
    pub fn new(id: u64, pos: Vec2) -> Self {
        Self {
            id,
            name: generate_random_name(),
            pos,
            state: AgentState::Idle,
            energy: 1.0,
            hunger: 1.0,
            social: 1.0,
            spirit: 1.0,
            job: Job::default(),
            home_pos: pos, // Default home is spawn position
            speed: 60.0 + rand::gen_range(-15.0, 15.0),
            color: [
                rand::gen_range(0.5, 1.0),
                rand::gen_range(0.5, 1.0),
                rand::gen_range(0.5, 1.0),
                1.0
            ],
            traits: crate::simulation::traits::generate_random_traits(),
            feats: AgentFeats::default(),
        }
    }
    
    pub fn with_job(mut self, job: Job) -> Self {
        self.job = job;
        self
    }
    
    pub fn with_home(mut self, home: Vec2) -> Self {
        self.home_pos = home;
        self
    }
    
    pub fn update(&mut self, delta: f32, world: &WorldInfo) {
        let time_of_day = TimeOfDay::from_hour(world.game_hour);
        
        // Calculate trait modifiers
        let mut energy_mod = 1.0f32;
        let mut hunger_mod = 1.0f32;
        let mut social_mod = 1.0f32;
        let mut spirit_mod = 1.0f32;
        for t in &self.traits {
            energy_mod *= t.energy_decay_modifier();
            hunger_mod *= t.hunger_decay_modifier();
            social_mod *= t.social_decay_modifier();
            spirit_mod *= t.spirit_decay_modifier();
        }
        
        // Needs Decay (with trait modifiers)
        let decay_rate = 0.02 * delta;
        self.energy = (self.energy - decay_rate * 0.3 * energy_mod).max(0.0);
        self.hunger = (self.hunger - decay_rate * 0.5 * hunger_mod).max(0.0);
        self.social = (self.social - decay_rate * 0.4 * social_mod).max(0.0);
        self.spirit = (self.spirit - decay_rate * 0.1 * spirit_mod).max(0.0);
        
        match self.state {
            AgentState::Idle => {
                // Time-of-day based decision tree
                match time_of_day {
                    TimeOfDay::Night => {
                        // At night, go home and sleep
                        if self.pos.distance(self.home_pos) > 20.0 {
                            self.state = AgentState::GoingHome;
                        } else {
                            self.state = AgentState::Sleeping;
                        }
                    },
                    TimeOfDay::Morning => {
                        // Morning routine: eat if hungry
                        if self.hunger < 0.5 && !world.markets.is_empty() {
                            let target = self.find_nearest(world.markets.as_slice());
                            self.state = AgentState::Wandering { target };
                        } else if self.energy < 0.3 {
                            self.state = AgentState::Sleeping;
                        }
                    },
                    TimeOfDay::Work => {
                        // Work time: go to workplace or find work
                        if self.energy < 0.2 {
                            self.state = AgentState::GoingHome;
                        } else if self.hunger < 0.3 && !world.markets.is_empty() {
                            let target = self.find_nearest(world.markets.as_slice());
                            self.state = AgentState::Wandering { target };
                        } else if !world.workshops.is_empty() && rand::gen_range(0, 100) < 5 {
                            let target = self.find_nearest(world.workshops.as_slice());
                            self.state = AgentState::Wandering { target };
                        } else if !world.construction_sites.is_empty() && self.job == Job::Builder {
                            // Builders go to construction sites
                            let (target, zone_idx) = world.construction_sites[0];
                            self.state = AgentState::Building { target, zone_idx };
                        }
                    },
                    TimeOfDay::Evening => {
                        // Evening: socialize, eat, relax
                        if self.hunger < 0.4 && !world.markets.is_empty() {
                            let target = self.find_nearest(world.markets.as_slice());
                            self.state = AgentState::Wandering { target };
                        } else if self.social < 0.5 && !world.parks.is_empty() {
                            let target = self.find_nearest(world.parks.as_slice());
                            self.state = AgentState::Wandering { target };
                        } else if rand::gen_range(0, 100) < 3 {
                            let target = self.pick_random_target();
                            self.state = AgentState::Wandering { target };
                        }
                    },
                }
            },
            AgentState::Wandering { target } => {
                let dist = self.pos.distance(target);
                if dist < 10.0 {
                    // Arrived! Determine what we are doing based on location
                    self.state = AgentState::Idle;
                    
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
                    self.hunger = 1.0;
                    self.spirit = (self.spirit + 0.1).min(1.0);
                    self.state = AgentState::Idle;
                }
            },
            AgentState::Socializing { ref mut duration, .. } => {
                *duration -= delta;
                if *duration <= 0.0 {
                    self.social = 1.0;
                    self.spirit = (self.spirit + 0.2).min(1.0);
                    self.feats.social_events += 1; // Track social feat
                    self.state = AgentState::Idle;
                }
            },
            AgentState::Working { ref mut duration, .. } => {
                *duration -= delta;
                self.energy = (self.energy - delta * 0.1).max(0.0);
                if *duration <= 0.0 {
                    self.spirit = (self.spirit + 0.05).min(1.0);
                    self.state = AgentState::Idle;
                }
            },
            AgentState::Building { target, .. } => {
                let dist = self.pos.distance(target);
                if dist < 10.0 {
                    // At construction site - work being done in tick.rs
                    self.energy = (self.energy - delta * 0.15).max(0.0);
                    if self.energy < 0.2 {
                        self.state = AgentState::GoingHome;
                    }
                } else {
                    // Move toward site
                    let dir = (target - self.pos).normalize();
                    self.pos += dir * self.speed * delta;
                }
            },
            AgentState::GoingHome => {
                let dist = self.pos.distance(self.home_pos);
                if dist < 10.0 {
                    self.state = AgentState::Sleeping;
                } else {
                    let dir = (self.home_pos - self.pos).normalize();
                    self.pos += dir * self.speed * delta;
                }
            },
            AgentState::Sleeping => {
                self.energy = (self.energy + delta * 0.3).min(1.0);
                if self.energy >= 0.9 {
                    let time_of_day = TimeOfDay::from_hour(world.game_hour);
                    if time_of_day != TimeOfDay::Night {
                        self.state = AgentState::Idle;
                    }
                }
            },
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
    pub construction_sites: Vec<(Vec2, usize)>, // Position and zone index
    pub game_hour: f32, // 0-24 hour cycle
}

