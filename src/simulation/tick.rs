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
    
    // Update season and weather
    let season_changed = state.season_state.update(total_hours);
    if season_changed {
        let season_name = state.season_state.season.name().to_string();
        state.log.add(
            state.game_time_hours,
            format!("{} has arrived!", season_name),
            crate::narrative::LogCategory::Event,
        );
        // Record in chronicle
        state.town_chronicle.record(
            state.game_time_hours,
            crate::narrative::ChronicleEventType::SeasonChanged { season: season_name },
        );
    }
    
    // Apply seasonal morale bonus to agents
    let morale_change = state.season_state.season.morale_bonus() * total_hours / 24.0;
    for agent in &mut state.agents {
        agent.spirit = (agent.spirit + morale_change).clamp(0.0, 1.0);
    }
    
    // Update caravans (uses Caravan::update)
    let days_elapsed = total_hours / 24.0;
    for caravan in &mut state.trade_manager.caravans {
        // Assume 2-day travel time for now
        let (arrived, returned) = caravan.update(2.0, days_elapsed);
        if arrived {
            // Caravan delivered goods - could add resources here
        }
        if returned {
            // Caravan returned home - could respawn it
        }
    }
    
    // Update town proxies (uses TownProxyManager methods)
    state.town_proxies.update_all(days_elapsed);
    
    // Check for proxy crises (uses crisis_count, all, get)
    let crisis_count = state.town_proxies.crisis_count();
    if crisis_count > 0 {
        // Could add notification about towns in crisis
        for proxy in state.town_proxies.all() {
            if proxy.needs_attention() {
                let _status = proxy.status();
                // Could log or notify player
            }
        }
    }
    
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
    // Add base capacity of 2.0 for "Campsite" so players aren't soft-locked if they restore non-housing first.
    let housing_capacity = state.calculate_housing_capacity() + bonuses.housing_flat + 2.0;
    
    // Population grows based on attractiveness and capacity
    // Boost growth based on active zones and attractiveness
    // Tech bonus to attractiveness applied here? Or to resource?
    // Let's apply to resource delta actually, so it persists.
    
    let growth_bonus = active_zones as f32 * 0.5;
    state.population.tick(
        state.resources.attractiveness * (1.0 + growth_bonus), 
        housing_capacity,
        game_minutes  // Use game time, not real time
    );
    
    // Get seasonal and weather modifiers (uses Season and Weather methods)
    let season = state.season_state.season;
    let weather = state.season_state.weather;
    let farm_mult = season.farm_growth_multiplier();
    let move_mult = season.movement_multiplier() * (1.0 - weather.movement_penalty());
    let _weather_visibility = weather.visibility_reduction();
    let _waters_crops = weather.waters_crops();
    let building_damage = weather.building_damage_chance();
    
    // Apply random building damage during storms
    if building_damage > 0.0 && rand::gen_range(0.0, 1.0) < building_damage * game_minutes / 60.0 {
        for zone in &mut state.zones {
            if !zone.dormant {
                zone.condition = (zone.condition - 0.01).max(0.5);
            }
        }
    }
    
    // Calculate and apply resource changes (batched)
    let mut total_output = crate::data::ResourceDelta::default();
    let mut total_upkeep = crate::data::ResourceDelta::default();
    
    // PASSIVE GATHERING:
    // 1. Base passive gain = 10.0 per game day (Buffed to prevent sticking)
    // 2. Population gain = 0.2 * sqrt(pop) per day (Diminishing returns)
    // Apply seasonal farm multiplier to production
    
    // We need RATE per minute. Day = 1440 minutes.
    let base_rate_per_min = (10.0 / 1440.0) * bonuses.production_multi * farm_mult;
    
    // Population gain: Diminishing returns using SQRT
    // Movement multiplier affects gathering efficiency
    let pop_rate_per_min = ((0.2 * state.population.value().sqrt()) / 1440.0) * bonuses.production_multi * move_mult;
    
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
    // Use round() to avoid flickering at integer boundaries
    let target_agents = (state.population.value().round() as usize).min(50);
    
    // Spawn (uses Agent::with_job and with_home builder methods)
    while state.agents.len() < target_agents {
        // Spawn at a random location (ideally at a house, but random for now)
        let id = rand::rand() as u64;
        let x = rand::gen_range(500.0, 800.0);
        let y = rand::gen_range(500.0, 800.0);
        let home_pos = macroquad::prelude::vec2(x + rand::gen_range(-50.0, 50.0), y + rand::gen_range(-50.0, 50.0));
        let job = crate::simulation::agents::Job::Laborer;
        
        let agent = crate::simulation::agents::Agent::new(id, macroquad::prelude::vec2(x, y))
            .with_job(job)
            .with_home(home_pos);
        state.agents.push(agent);
    }
    
    // Despawn (if population drops)
    while state.agents.len() > target_agents {
        state.agents.pop();
    }
    
    // Update game hour (24-hour cycle, 1 game minute = 1 real second)
    // So 1 real minute = 1 game hour, 24 real minutes = 1 game day
    state.game_hour = (state.game_hour + game_minutes / 60.0) % 24.0;
    
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
    let mut construction_sites = Vec::new();
    
    for (zone_idx, zone) in state.zones.iter().enumerate() {
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
            
            // Check for construction sites
            if zone.is_under_construction() {
                construction_sites.push((pos, zone_idx));
                continue;
            }
            
            if zone.dormant {
                continue;
            }
            
            match template.category {
                crate::data::ZoneCategory::Market => markets.push(pos),
                crate::data::ZoneCategory::Infrastructure => workshops.push(pos),
                crate::data::ZoneCategory::Cultural => parks.push(pos),
                _ => {},
            }
        }
    }
    
    let world_info = crate::simulation::agents::WorldInfo {
        markets,
        workshops,
        parks,
        construction_sites,
        game_hour: state.game_hour,
    };
    for agent in &mut state.agents {
        agent.update(agent_delta, &world_info);
    }

    // Apply population-based maintenance cost
    let maint_cost = state.calculate_maintenance_cost() * game_minutes;
    net_delta.maintenance -= maint_cost;
    
    // Add floating text for significant material changes (uses add_gain/add_loss)
    if net_delta.materials.abs() > 0.5 {
        // Find a zone to spawn the text near (first active zone)
        let spawn_pos = if let Some(zone) = state.zones.iter().find(|z| !z.dormant) {
            if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
                if let Some(rect) = template.map_rect {
                    macroquad::prelude::vec2(
                        (rect.x as f32 + rect.w as f32 / 2.0) * crate::ui::map_renderer::TILE_SIZE,
                        (rect.y as f32 + rect.h as f32 / 2.0) * crate::ui::map_renderer::TILE_SIZE
                    )
                } else {
                    macroquad::prelude::vec2(500.0, 300.0)
                }
            } else {
                macroquad::prelude::vec2(500.0, 300.0)
            }
        } else {
            macroquad::prelude::vec2(500.0, 300.0)
        };
        
        if net_delta.materials > 0.0 {
            state.floating_texts.add_gain(net_delta.materials, "Materials", spawn_pos);
        } else {
            state.floating_texts.add_loss(net_delta.materials.abs(), "Materials", spawn_pos);
        }
    }
    
    // PARTICLE SYSTEM UPDATE
    state.particle_system.update(game_minutes * 60.0 * agent_delta); // Approximate sync with frame time

    // TUTORIAL UPDATE
    // Build context for tutorial triggers
    let active_zones = state.zones.iter().filter(|z| !z.dormant).count();
    let tutorial_ctx = crate::narrative::tutorial::TutorialContext {
        zones_active: active_zones,
        agent_count: state.agents.len(),
        materials: state.resources.materials,
        game_hour: state.game_hour,
        day: (state.game_time_hours / 24.0) as u32 + 1,
    };
    state.tutorial.update(tutorial_ctx, game_minutes);

    // Weather particles
    let weather = state.season_state.weather;
    let particle_count = match weather {
        crate::simulation::seasons::Weather::Rain => 2,
        crate::simulation::seasons::Weather::Storm => 5,
        crate::simulation::seasons::Weather::Snow => 2,
        _ => 0,
    };

    if particle_count > 0 {
        use crate::ui::particles::ParticleType;
        let screen_w = macroquad::window::screen_width();
        let screen_h = macroquad::window::screen_height();
        
        // Spawn weather particles in "world" space relative to camera?
        // Actually weather usually falls across the screen.
        // But our particle system is world-space.
        // We should spawn them around the camera view.
        let cam_center = state.camera.target;
        let spawn_w = screen_w / state.camera.zoom;
        let spawn_h = screen_h / state.camera.zoom;
        
        for _ in 0..particle_count {
             let x = cam_center.x - spawn_w/2.0 + rand::gen_range(0.0, spawn_w);
             let y = cam_center.y - spawn_h/2.0 + rand::gen_range(0.0, spawn_h);
             let pos = macroquad::prelude::vec2(x, y);
             
             let (vel, color, life, p_type) = match weather {
                 crate::simulation::seasons::Weather::Snow => (
                     macroquad::prelude::vec2(rand::gen_range(-10.0, 10.0), 30.0),
                     macroquad::prelude::WHITE,
                     4.0,
                     ParticleType::Snow
                 ),
                 _ => ( // Rain/Storm
                     macroquad::prelude::vec2(-5.0, 200.0),
                     macroquad::prelude::Color::new(0.6, 0.6, 1.0, 0.6),
                     1.5,
                     ParticleType::Rain
                 ),
             };
             
             state.particle_system.spawn(pos, vel, life, 3.0, color, p_type);
        }
    }

    // Chimney Smoke (only in winter)
    // Only spawn occasionally
    if state.season_state.season == crate::simulation::seasons::Season::Winter && rand::gen_range(0.0, 1.0) < 0.1 {
        use crate::ui::particles::ParticleType;
        // Find active houses
         for zone in &state.zones {
             if !zone.dormant && zone.activity > 0.0 {
                 if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
                     if template.category == crate::data::ZoneCategory::Residential {
                         if let Some(rect) = template.map_rect {
                             let tile_size = crate::ui::map_renderer::TILE_SIZE;
                             // Randomly pick a spot on roof properly
                             let center_x = (rect.x as f32 + rect.w as f32 * 0.5) * tile_size;
                             let center_y = (rect.y as f32 + rect.h as f32 * 0.2) * tile_size; // Top of building
                             
                             state.particle_system.spawn(
                                 macroquad::prelude::vec2(center_x, center_y),
                                 macroquad::prelude::vec2(rand::gen_range(-5.0, 5.0), rand::gen_range(-20.0, -10.0)),
                                 rand::gen_range(2.0, 4.0),
                                 rand::gen_range(4.0, 8.0),
                                 macroquad::prelude::Color::new(0.8, 0.8, 0.8, 0.4),
                                 ParticleType::Smoke
                             );
                         }
                     }
                 }
             }
         }
    }

    // Update stats and check achievements
    update_stats_and_achievements(state, net_delta.materials.max(0.0));
    
    state.resources.apply_delta(&net_delta);
}

/// Update game stats and check for achievement unlocks
fn update_stats_and_achievements(state: &mut crate::data::GameState, resources_gained: f32) {
    use crate::data::{Achievement, ZoneCategory};
    use crate::simulation::seasons::Season;
    
    // Update stats
    state.stats.add_resources(resources_gained);
    state.stats.total_play_hours = state.game_time_hours;
    state.stats.update_peaks(state.resources.materials, state.population.value() as u32);
    
    // Check population achievements
    let pop = state.population.value() as u32;
    if pop >= 50 {
        state.achievements.unlock(Achievement::GrowingCommunity);
    }
    if pop >= 100 {
        state.achievements.unlock(Achievement::FirstHundred);
    }
    if pop >= 500 {
        state.achievements.unlock(Achievement::TownProper);
    }
    
    // Check resource achievements
    let mats = state.resources.materials;
    if mats >= 1000.0 {
        state.achievements.unlock(Achievement::Hoarder);
    }
    if mats >= 10000.0 {
        state.achievements.unlock(Achievement::Wealthy);
    }
    if mats >= 100000.0 {
        state.achievements.unlock(Achievement::Millionaire);
    }
    
    // Check zone-based achievements
    let active_zones = state.zones.iter().filter(|z| !z.dormant).count();
    if active_zones >= 1 {
        // Check if any is residential for FirstHouse
        for zone in &state.zones {
            if !zone.dormant {
                if let Some(template) = state.zone_templates.iter().find(|t| t.id == zone.template_id) {
                    if template.category == ZoneCategory::Residential {
                        state.achievements.unlock(Achievement::FirstHouse);
                        break;
                    }
                }
            }
        }
    }
    if active_zones >= 10 {
        state.achievements.unlock(Achievement::SettlerSpirit);
    }
    if active_zones >= 25 {
        state.achievements.unlock(Achievement::Builder);
    }
    if active_zones >= 50 {
        state.achievements.unlock(Achievement::MasterBuilder);
    }
    
    // Check utopia (stability + attractiveness > 0.8 each)
    if state.resources.stability > 0.8 && state.resources.attractiveness > 0.8 {
        state.achievements.unlock(Achievement::Utopia);
    }
    
    // Check play time achievements
    let days = state.game_time_hours / 24.0;
    if days >= 10.0 {
        state.achievements.unlock(Achievement::Dedicated);
    }
    if days >= 100.0 {
        state.achievements.unlock(Achievement::Veteran);
    }
    
    // Check winter survivor (when season changes FROM winter)
    if state.season_state.season == Season::Spring && state.season_state.day_in_season < 1.0 {
        // Just transitioned from winter
        if state.stats.winters_survived == 0 {
            state.stats.winters_survived = 1;
            state.achievements.unlock(Achievement::WinterSurvivor);
        }
    }
    
    // Check dynasty achievements
    if state.dynasty.hall_of_heroes.len() >= 1 {
        state.achievements.unlock(Achievement::Remembered);
    }
    if state.dynasty.ancestors.len() >= 10 {
        state.achievements.unlock(Achievement::AncestorWorship);
    }
    if state.dynasty.completed_wonders.len() >= 1 {
        state.achievements.unlock(Achievement::LegacyFounder);
    }
    if state.dynasty.completed_wonders.len() >= 3 {
        state.achievements.unlock(Achievement::WonderWorker);
    }
    if state.dynasty.past_towns.len() + 1 >= 5 {
        state.achievements.unlock(Achievement::DynastyRuler);
    }
    
    // Check scholar (all tech unlocked)
    let all_researched = state.tech_tree.iter().all(|t| t.unlocked);
    if all_researched && !state.tech_tree.is_empty() {
        state.achievements.unlock(Achievement::Scholar);
    }
    
    // Log newly unlocked achievements
    while let Some(achievement) = state.achievements.pop_notification() {
        state.log.add(
            state.game_time_hours,
            format!("{} Achievement Unlocked: {}!", achievement.icon(), achievement.name()),
            crate::narrative::LogCategory::Milestone,
        );
    }
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
