//! Season and weather system

use serde::{Deserialize, Serialize};
use macroquad::rand;

/// The four seasons of the year
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    /// Get the next season in the cycle
    pub fn next(&self) -> Self {
        match self {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
            Season::Winter => Season::Spring,
        }
    }
    
    /// Display name
    pub fn name(&self) -> &'static str {
        match self {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter => "Winter",
        }
    }
    
    /// Base color tint for the season (RGBA)
    pub fn color_tint(&self) -> [f32; 4] {
        match self {
            Season::Spring => [0.4, 0.9, 0.5, 0.1],  // Light green
            Season::Summer => [1.0, 0.9, 0.6, 0.08], // Warm yellow
            Season::Autumn => [0.9, 0.6, 0.3, 0.12], // Orange brown
            Season::Winter => [0.7, 0.8, 1.0, 0.15], // Cold blue
        }
    }
    
    /// Farm growth rate multiplier
    pub fn farm_growth_multiplier(&self) -> f32 {
        match self {
            Season::Spring => 1.5,  // Best growing
            Season::Summer => 1.2,
            Season::Autumn => 0.8,  // Harvest time
            Season::Winter => 0.0,  // No farming
        }
    }
    
    /// Agent movement speed multiplier
    pub fn movement_multiplier(&self) -> f32 {
        match self {
            Season::Spring => 1.0,
            Season::Summer => 1.1,  // Faster in summer
            Season::Autumn => 0.95, // Wind slows slightly
            Season::Winter => 0.8,  // Snow slows movement
        }
    }
    
    /// Spirit/morale bonus per game day
    pub fn morale_bonus(&self) -> f32 {
        match self {
            Season::Spring => 0.05, // Hope of new growth
            Season::Summer => 0.02,
            Season::Autumn => 0.0,
            Season::Winter => -0.03, // Cold is demoralizing
        }
    }
}

impl Default for Season {
    fn default() -> Self {
        Season::Spring
    }
}

/// Weather conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rain,
    Storm,
    Snow,
    Fog,
}

impl Weather {
    pub fn name(&self) -> &'static str {
        match self {
            Weather::Sunny => "Sunny",
            Weather::Cloudy => "Cloudy",
            Weather::Rain => "Rainy",
            Weather::Storm => "Stormy",
            Weather::Snow => "Snowing",
            Weather::Fog => "Foggy",
        }
    }
    
    /// Whether this weather waters crops
    pub fn waters_crops(&self) -> bool {
        matches!(self, Weather::Rain | Weather::Storm)
    }
    
    /// Chance per hour to damage buildings (0.0 - 1.0)
    pub fn building_damage_chance(&self) -> f32 {
        match self {
            Weather::Storm => 0.02, // 2% chance per hour
            _ => 0.0,
        }
    }
    
    /// Movement speed penalty (multiplier)
    pub fn movement_penalty(&self) -> f32 {
        match self {
            Weather::Sunny => 1.0,
            Weather::Cloudy => 1.0,
            Weather::Rain => 0.9,
            Weather::Storm => 0.7,
            Weather::Snow => 0.75,
            Weather::Fog => 0.85,
        }
    }
    
    /// Visibility reduction (0.0 = clear, 1.0 = blind)
    pub fn visibility_reduction(&self) -> f32 {
        match self {
            Weather::Sunny => 0.0,
            Weather::Cloudy => 0.1,
            Weather::Rain => 0.2,
            Weather::Storm => 0.4,
            Weather::Snow => 0.3,
            Weather::Fog => 0.5,
        }
    }
}

impl Default for Weather {
    fn default() -> Self {
        Weather::Sunny
    }
}

/// Current season and weather state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonState {
    /// Current season
    pub season: Season,
    
    /// Days elapsed in current season (0-11, season changes at 12)
    pub day_in_season: f32,
    
    /// Current weather
    pub weather: Weather,
    
    /// Hours until next weather change
    pub weather_duration: f32,
    
    /// Total days elapsed (for tracking)
    pub total_days: f32,
}

impl Default for SeasonState {
    fn default() -> Self {
        Self {
            season: Season::Spring,
            day_in_season: 0.0,
            weather: Weather::Sunny,
            weather_duration: 6.0, // 6 hours
            total_days: 0.0,
        }
    }
}

impl SeasonState {
    /// Days per season (configurable)
    pub const DAYS_PER_SEASON: f32 = 12.0;
    
    /// Update season state with elapsed game hours
    /// Returns true if season changed
    pub fn update(&mut self, game_hours: f32) -> bool {
        let game_days = game_hours / 24.0;
        self.total_days += game_days;
        self.day_in_season += game_days;
        
        // Update weather duration
        self.weather_duration -= game_hours;
        if self.weather_duration <= 0.0 {
            self.roll_new_weather();
        }
        
        // Check for season change
        if self.day_in_season >= Self::DAYS_PER_SEASON {
            self.day_in_season -= Self::DAYS_PER_SEASON;
            self.season = self.season.next();
            return true;
        }
        
        false
    }
    
    /// Roll new weather based on season probabilities
    fn roll_new_weather(&mut self) {
        let roll = rand::gen_range(0.0, 1.0);
        
        self.weather = match self.season {
            Season::Spring => {
                if roll < 0.4 { Weather::Sunny }
                else if roll < 0.7 { Weather::Cloudy }
                else if roll < 0.9 { Weather::Rain }
                else { Weather::Fog }
            },
            Season::Summer => {
                if roll < 0.6 { Weather::Sunny }
                else if roll < 0.85 { Weather::Cloudy }
                else { Weather::Storm }
            },
            Season::Autumn => {
                if roll < 0.3 { Weather::Sunny }
                else if roll < 0.5 { Weather::Cloudy }
                else if roll < 0.75 { Weather::Rain }
                else if roll < 0.9 { Weather::Fog }
                else { Weather::Storm }
            },
            Season::Winter => {
                if roll < 0.3 { Weather::Sunny }
                else if roll < 0.5 { Weather::Cloudy }
                else if roll < 0.8 { Weather::Snow }
                else { Weather::Fog }
            },
        };
        
        // Set new duration (3-12 hours)
        self.weather_duration = rand::gen_range(3.0, 12.0);
    }
    
    /// Get current season display string
    pub fn display_string(&self) -> String {
        format!(
            "{}, Day {:.0} - {}",
            self.season.name(),
            self.day_in_season + 1.0,
            self.weather.name()
        )
    }
}
