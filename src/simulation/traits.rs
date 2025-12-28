//! Agent traits for personality and emergent storytelling

use serde::{Deserialize, Serialize};
use macroquad::rand;

/// Personality traits that affect agent behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trait {
    // Work-related
    Hardworking,    // +20% work speed
    Lazy,           // -15% work speed
    NightOwl,       // Works better at night
    EarlyBird,      // Works better in morning
    
    // Social
    Charismatic,    // +Opinion gain from interactions
    Loner,          // Needs less social, dislikes crowds
    Gossip,         // Spreads memories faster
    
    // Needs
    Glutton,        // Eats 2x as much
    Frugal,         // Eats 0.5x as much
    Energetic,      // Needs less sleep
    Sleepyhead,     // Needs more sleep
    
    // Special
    Optimist,       // Spirit decays slower
    Pessimist,      // Spirit decays faster
    Tough,          // Less affected by bad events
    Sensitive,      // More affected by all events
}

impl Trait {
    /// Display name for UI
    pub fn name(&self) -> &'static str {
        match self {
            Trait::Hardworking => "Hardworking",
            Trait::Lazy => "Lazy",
            Trait::NightOwl => "Night Owl",
            Trait::EarlyBird => "Early Bird",
            Trait::Charismatic => "Charismatic",
            Trait::Loner => "Loner",
            Trait::Gossip => "Gossip",
            Trait::Glutton => "Glutton",
            Trait::Frugal => "Frugal",
            Trait::Energetic => "Energetic",
            Trait::Sleepyhead => "Sleepyhead",
            Trait::Optimist => "Optimist",
            Trait::Pessimist => "Pessimist",
            Trait::Tough => "Tough",
            Trait::Sensitive => "Sensitive",
        }
    }
    
    /// Description for tooltips
    pub fn description(&self) -> &'static str {
        match self {
            Trait::Hardworking => "Works 20% faster",
            Trait::Lazy => "Works 15% slower",
            Trait::NightOwl => "More productive at night",
            Trait::EarlyBird => "More productive in morning",
            Trait::Charismatic => "Gains more opinion from social interactions",
            Trait::Loner => "Needs less social, but dislikes crowds",
            Trait::Gossip => "Spreads news and rumors quickly",
            Trait::Glutton => "Eats twice as much food",
            Trait::Frugal => "Eats half as much food",
            Trait::Energetic => "Needs less sleep",
            Trait::Sleepyhead => "Needs more sleep",
            Trait::Optimist => "Morale decreases slower",
            Trait::Pessimist => "Morale decreases faster",
            Trait::Tough => "Less affected by negative events",
            Trait::Sensitive => "Strongly affected by all events",
        }
    }
    
    /// Work speed multiplier
    pub fn work_speed_modifier(&self) -> f32 {
        match self {
            Trait::Hardworking => 1.2,
            Trait::Lazy => 0.85,
            _ => 1.0,
        }
    }
    
    /// Hunger decay multiplier
    pub fn hunger_decay_modifier(&self) -> f32 {
        match self {
            Trait::Glutton => 2.0,
            Trait::Frugal => 0.5,
            _ => 1.0,
        }
    }
    
    /// Energy decay multiplier
    pub fn energy_decay_modifier(&self) -> f32 {
        match self {
            Trait::Energetic => 0.7,
            Trait::Sleepyhead => 1.4,
            _ => 1.0,
        }
    }
    
    /// Spirit decay multiplier
    pub fn spirit_decay_modifier(&self) -> f32 {
        match self {
            Trait::Optimist => 0.5,
            Trait::Pessimist => 1.5,
            _ => 1.0,
        }
    }
    
    /// Social need decay multiplier
    pub fn social_decay_modifier(&self) -> f32 {
        match self {
            Trait::Loner => 0.5,
            _ => 1.0,
        }
    }
}

/// Generate random traits for a new agent (1-3 traits)
pub fn generate_random_traits() -> Vec<Trait> {
    let all_traits = [
        Trait::Hardworking, Trait::Lazy, Trait::NightOwl, Trait::EarlyBird,
        Trait::Charismatic, Trait::Loner, Trait::Gossip,
        Trait::Glutton, Trait::Frugal, Trait::Energetic, Trait::Sleepyhead,
        Trait::Optimist, Trait::Pessimist, Trait::Tough, Trait::Sensitive,
    ];
    
    let count = rand::gen_range(1u32, 4) as usize;
    let mut traits = Vec::with_capacity(count);
    
    for _ in 0..count {
        let idx = rand::gen_range(0, all_traits.len());
        let t = all_traits[idx];
        // Avoid duplicates and conflicting traits
        if !traits.contains(&t) && !conflicts_with(&traits, t) {
            traits.push(t);
        }
    }
    
    traits
}

/// Check if a trait conflicts with existing traits
fn conflicts_with(existing: &[Trait], new: Trait) -> bool {
    for t in existing {
        let conflict = match (t, new) {
            (Trait::Hardworking, Trait::Lazy) | (Trait::Lazy, Trait::Hardworking) => true,
            (Trait::NightOwl, Trait::EarlyBird) | (Trait::EarlyBird, Trait::NightOwl) => true,
            (Trait::Glutton, Trait::Frugal) | (Trait::Frugal, Trait::Glutton) => true,
            (Trait::Energetic, Trait::Sleepyhead) | (Trait::Sleepyhead, Trait::Energetic) => true,
            (Trait::Optimist, Trait::Pessimist) | (Trait::Pessimist, Trait::Optimist) => true,
            (Trait::Tough, Trait::Sensitive) | (Trait::Sensitive, Trait::Tough) => true,
            _ => false,
        };
        if conflict { return true; }
    }
    false
}
