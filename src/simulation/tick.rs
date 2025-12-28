//! Game tick system - Time management and simulation stepping

use serde::{Deserialize, Serialize};
use macroquad::rand;

// Helper struct for aggregating tech effects
#[derive(Default)]
struct TechBonuses {
    production_multi: f32,
    maintenance_factor: f32,
    attractiveness_flat: f32,
    housing_flat: f32,
}
// Manually impl default to set multipliers to 1.0
impl TechBonuses {
    fn default() -> Self {
        Self {
            production_multi: 1.0,
            maintenance_factor: 1.0,
            attractiveness_flat: 0.0,
            housing_flat: 0.0,
        }
    }
}

/// Manages game tick timing
/// 
/// Separates frame time (fast, visual updates) from game ticks (slow, logic updates).
/// This allows the simulation to run at a consistent rate regardless of framerate.
pub struct TickTimer {
    /// Accumulated time since last game tick
    accumulated: f32,
    /// Time between game ticks in seconds
    tick_rate: f32,
}

impl TickTimer {
    pub fn new(tick_rate: f32) -> Self {
        Self {
            accumulated: 0.0,
            tick_rate,
        }
    }

    /// Update timer with frame delta, returns number of game ticks to process
    /// 
    /// Call this every frame with get_frame_time(). It accumulates time and
    /// returns how many full ticks should be processed this frame.
    pub fn update(&mut self, delta_time: f32) -> u32 {
        self.accumulated += delta_time;
        let ticks = (self.accumulated / self.tick_rate) as u32;
        self.accumulated -= ticks as f32 * self.tick_rate;
        ticks
    }

    /// Get the configured tick rate
    pub fn tick_rate(&self) -> f32 {
        self.tick_rate
    }
}

/// Tracks time spent in game and offline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTracker {
    /// Total game time in hours (including offline)
    pub total_hours: f32,
    
    /// Last save timestamp (Unix seconds)
    pub last_save_time: u64,
    
    /// Session start timestamp (Unix seconds)  
    pub session_start_time: u64,
}

impl Default for TimeTracker {
    fn default() -> Self {
        Self {
            total_hours: 0.0,
            last_save_time: 0,
            session_start_time: 0,
        }
    }
}

impl TimeTracker {
    /// Create a new time tracker with current timestamp
    pub fn new(current_time: u64) -> Self {
        Self {
            total_hours: 0.0,
            last_save_time: current_time,
            session_start_time: current_time,
        }
    }

    /// Calculate hours spent offline since last save
    pub fn calculate_offline_hours(&self, current_time: u64, cap_hours: f32) -> f32 {
        if current_time <= self.last_save_time {
            return 0.0;
        }
        
        let offline_seconds = (current_time - self.last_save_time) as f32;
        let offline_hours = offline_seconds / 3600.0;
        
        // Apply soft cap from config
        offline_hours.min(cap_hours)
    }

    /// Update last save time
    pub fn mark_saved(&mut self, current_time: u64) {
        self.last_save_time = current_time;
    }

    /// Add played time
    pub fn add_time(&mut self, hours: f32) {
        self.total_hours += hours;
    }
}

/// Process offline progression
/// 
/// Called when loading a save to apply accumulated gains while away.
/// Uses logarithmic scaling to prevent AFK abuse.
pub fn process_offline_time(
    state: &mut crate::data::GameState,
    offline_hours: f32,
) {
    if offline_hours <= 0.0 {
        return;
    }

    // Calculate base output at current state
    let base_output = state.calculate_total_output();
    
    // Apply logarithmic scaling (from formulas doc)
    // OfflineGain = Output × log(TimeAway + 1)
    let offline_gain = crate::economy::offline_gain(base_output, offline_hours);
    
    // Apply gains to resources (simplified: add to attractiveness/materials)
    state.resources.materials += offline_gain * 0.5;
    state.resources.attractiveness += offline_gain * 0.2;
    
    // Population also grew while away (but saturated)
    let pop_growth = offline_hours * 0.1 * state.resources.attractiveness;
    state.population.apply_delta(pop_growth);
    
    // Log the offline progress
    state.log.add(
        state.game_time_hours,
        format!(
            "While you were away ({:.1}h): The town quietly grew. (+{:.2} resources)",
            offline_hours, offline_gain
        ),
        crate::narrative::LogCategory::System,
    );
    
    // Update game time
    state.game_time_hours += offline_hours;
}

/// Simulate multiple ticks at once (for catching up or fast-forward)
/// 
/// More efficient than calling game_tick() many times - batches calculations.
/// Time scale: 1 real second = 1 game minute at 1x speed
pub fn simulate_ticks(
    state: &mut crate::data::GameState,
    num_ticks: u32,
    tick_seconds: f32,
) {
    if num_ticks == 0 {
        return;
    }

    // For efficiency, we batch similar operations
    let total_seconds = num_ticks as f32 * tick_seconds;
    
    // Time scale: 1 real second = 1 game minute
    // ALL calculations now use game_minutes for consistency
    let game_minutes = total_seconds; // 1:1 real seconds to game minutes
    let total_hours = game_minutes / 60.0;
    
    // Track population before update for milestone checking
    let _pop_before = state.population.value();
    
    // Update game time
    state.game_time_hours += total_hours;
    
    // Count active zones for population growth
    let active_zones = state.zones.iter().filter(|z| !z.dormant).count();
    
    // --- TECH BONUSES ---
    let mut bonuses = TechBonuses::default();
    for tech in &state.tech_tree {
        if tech.unlocked {
            match tech.effect {
                crate::data::TechEffect::ProductionMulti(m) => bonuses.production_multi *= m,
                crate::data::TechEffect::EfficiencyMulti(m) => bonuses.maintenance_factor *= m,
                crate::data::TechEffect::AttractivenessFlat(v) => bonuses.attractiveness_flat += v,
                crate::data::TechEffect::HousingGlobal(v) => bonuses.housing_flat += v,
            }
        }
    }
    
    // Calculate total housing capacity (Base + Tech)
    // Note: calculate_housing_capacity() iterates zones. We should probably modify that function or add bonus here.
    // For now, let's just add the flat bonus * number of residential zones? Or just global flat?
    // Let's assume global flat is added to total.
    let housing_capacity = state.calculate_housing_capacity() + bonuses.housing_flat;
    
    // Population only grows if there are active zones!
    if active_zones > 0 {
        // Boost growth based on active zones and attractiveness
        // Tech bonus to attractiveness applied here? Or to resource?
        // Let's apply to resource delta actually, so it persists.
        
        let growth_bonus = active_zones as f32 * 0.5;
        state.population.tick(
            state.resources.attractiveness * (1.0 + growth_bonus), 
            housing_capacity,
            game_minutes  // Use game time, not real time
        );
    }
    
    // ...
    
    // Calculate and apply resource changes (batched)
    let mut total_output = crate::data::ResourceDelta::default();
    let mut total_upkeep = crate::data::ResourceDelta::default();
    
    // PASSIVE GATHERING:
    // 1. Base passive gain = 10.0 per game day (Buffed to prevent sticking)
    // 2. Population gain = 0.2 * sqrt(pop) per day (Diminishing returns)
    
    // We need RATE per minute. Day = 1440 minutes.
    let base_rate_per_min = (10.0 / 1440.0) * bonuses.production_multi;
    
    // Population gain: Diminishing returns using SQRT
    // This prevents massive exponential explosions at high pop
    let pop_rate_per_min = ((0.2 * state.population.value().sqrt()) / 1440.0) * bonuses.production_multi;
    
    // Add rates to accumulator
    total_output.materials += base_rate_per_min + pop_rate_per_min;
    total_output.attractiveness += bonuses.attractiveness_flat * 0.001; // Small trickle or flat boost?
    // Actually flat bonus should probably be permanent stat, but here we deal with deltas.
    // Let's just say Tech gives +Attractiveness RATE.
    
    for zone in &state.zones {
        if zone.dormant {
            continue;
        }
        
        if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
            let throughput = zone.calculate_throughput(template);
            let multiplier = crate::economy::calculate_output(throughput, &state.resources);
            
            // Accumulate scaled outputs (Applied Production Multiplier)
            total_output.materials += template.output.materials * multiplier * bonuses.production_multi;
            total_output.maintenance += template.output.maintenance * multiplier; // Maintenance output usually 0
            total_output.attractiveness += template.output.attractiveness * multiplier;
            total_output.stability += template.output.stability * multiplier;
            
            // Accumulate upkeep (these are costs, will be subtracted)
            // Apply Efficiency Multiplier to upkeep
            total_upkeep.materials += template.upkeep.materials * bonuses.maintenance_factor;
            total_upkeep.maintenance += template.upkeep.maintenance * bonuses.maintenance_factor;
            total_upkeep.attractiveness += template.upkeep.attractiveness * bonuses.maintenance_factor;
            total_upkeep.stability += template.upkeep.stability * bonuses.maintenance_factor;
        }
    }
    
    // Apply net resource changes (output - upkeep) × game time
    let mut net_delta = crate::data::ResourceDelta {
        materials: (total_output.materials - total_upkeep.materials) * game_minutes,
        maintenance: (total_output.maintenance - total_upkeep.maintenance) * game_minutes,
        attractiveness: (total_output.attractiveness - total_upkeep.attractiveness) * game_minutes,
        stability: (total_output.stability - total_upkeep.stability) * game_minutes,
    };
    
    // GLOBAL RESOURCE DECAY (Soft Cap)
    // Prevent Attractiveness/Stability from spiraling to infinity.
    // Lose 0.1% of current stockpile per game minute.
    let decay_rate = 0.001 * game_minutes;
    net_delta.attractiveness -= state.resources.attractiveness * decay_rate;
    net_delta.stability -= state.resources.stability * decay_rate;
    
    // --- AGENT SIMULATION ---
    // Target agent count based on population (capped for performance/visual clutter)
    let target_agents = (state.population.value() as usize).min(50);
    
    // Spawn
    while state.agents.len() < target_agents {
        // Spawn at a random location (ideally at a house, but random for now)
        let id = rand::rand() as u64;
        let x = rand::gen_range(500.0, 800.0);
        let y = rand::gen_range(500.0, 800.0);
        state.agents.push(crate::simulation::agents::Agent::new(id, macroquad::prelude::vec2(x, y)));
    }
    
    // Despawn (if population drops)
    while state.agents.len() > target_agents {
        state.agents.pop();
    }
    
    // Update Agents
    // We update agents in real-time delta (approx), not batched game_minutes
    // Because movement is visual.
    // However, simulate_ticks is called with game_minutes batches.
    // For movement, we should use a fixed small delta step or just use game_minutes if it represents "fast forward".
    // Actually, agent movement should be decoupled from economy ticks if economy is super fast?
    // But for MVP, let's just step them.
    // We need to pass a "Visual Delta" vs "Game Delta".
    // But `simulate_ticks` is disconnected from frame time in the arguments.
    // Let's assume 1 tick = 1 update step for agents.
    // Movement speed should be scaled appropriately.
    let agent_delta = 0.016; // Approx 60fps step
    
    // Populate World Info for Agents
    let mut markets = Vec::new();
    let mut workshops = Vec::new();
    let mut parks = Vec::new();
    
    for zone in &state.zones {
        if !zone.dormant {
            if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
                // Get Center Position
                let pos = if let Some(rect) = template.map_rect {
                    macroquad::prelude::vec2(
                        (rect.x as f32 + rect.w as f32 / 2.0) * crate::ui::map_renderer::TILE_SIZE,
                        (rect.y as f32 + rect.h as f32 / 2.0) * crate::ui::map_renderer::TILE_SIZE
                    )
                } else {
                    continue; // No physical location
                };
                
                match template.category {
                    crate::data::ZoneCategory::Market => markets.push(pos),
                    crate::data::ZoneCategory::Infrastructure => workshops.push(pos),
                    crate::data::ZoneCategory::Cultural => parks.push(pos),
                    _ => {},
                }
            }
        }
    }
    
    let mut world_info = crate::simulation::agents::WorldInfo {
        markets,
        workshops,
        parks,
    };
    for agent in &mut state.agents {
        agent.update(agent_delta, &mut world_info);
    }

    // Apply population-based maintenance cost
    let maint_cost = state.calculate_maintenance_cost() * game_minutes;
    net_delta.maintenance -= maint_cost;
    
    state.resources.apply_delta(&net_delta);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_timer_accumulation() {
        let mut timer = TickTimer::new(1.0); // 1 tick per second
        
        // Half a second - no tick yet
        assert_eq!(timer.update(0.5), 0);
        
        // Another half - now we have 1 tick
        assert_eq!(timer.update(0.5), 1);
        
        // 2.5 seconds at once - 2 ticks, 0.5 remains
        assert_eq!(timer.update(2.5), 2);
        assert_eq!(timer.update(0.5), 1);
    }

    #[test]
    fn test_offline_hours_calculation() {
        let tracker = TimeTracker {
            total_hours: 10.0,
            last_save_time: 1000,
            session_start_time: 0,
        };
        
        // 1 hour later (3600 seconds)
        let offline = tracker.calculate_offline_hours(4600, 72.0);
        assert!((offline - 1.0).abs() < 0.01);
        
        // Test cap
        let tracker2 = TimeTracker {
            total_hours: 0.0,
            last_save_time: 0,
            session_start_time: 0,
        };
        // 100 hours later, but capped at 72
        let offline = tracker2.calculate_offline_hours(360000, 72.0);
        assert_eq!(offline, 72.0);
    }
}
