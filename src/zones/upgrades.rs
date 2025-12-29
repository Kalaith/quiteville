//! Building upgrade system

use crate::data::GameState;

/// Check if a zone can be upgraded
pub fn can_upgrade(state: &GameState, zone_idx: usize) -> Option<&str> {
    let zone = state.zones.get(zone_idx)?;
    let template = state.zone_templates.iter().find(|t| t.id == zone.template_id)?;
    
    // Zone must be complete (not under construction) and in good condition
    if zone.is_under_construction() || zone.condition < 0.8 {
        return None;
    }
    
    // Check if template has an upgrade path
    template.upgrade_to.as_deref()
}

/// Check if player can afford the upgrade
pub fn can_afford_upgrade(state: &GameState, zone_idx: usize) -> bool {
    let zone = match state.zones.get(zone_idx) {
        Some(z) => z,
        None => return false,
    };
    
    let template = match state.zone_templates.iter().find(|t| t.id == zone.template_id) {
        Some(t) => t,
        None => return false,
    };
    
    // Check if we have the target template
    let target_id = match &template.upgrade_to {
        Some(id) => id,
        None => return false,
    };
    
    let target = match state.zone_templates.iter().find(|t| &t.id == target_id) {
        Some(t) => t,
        None => return false,
    };
    
    // Check if we have required tech
    if let Some(ref tech_id) = target.locked_by_tech {
        let tech_unlocked = state.tech_tree.iter()
            .any(|t| &t.id == tech_id && t.unlocked);
        if !tech_unlocked {
            return false;
        }
    }
    
    // Check if we can afford it
    state.resources.materials >= target.construction_cost
}

/// Apply an upgrade to a zone (changes its template)
/// Returns the old template ID if successful
pub fn apply_upgrade(state: &mut GameState, zone_idx: usize) -> Option<String> {
    // Get upgrade target
    let (old_id, new_id, cost) = {
        let zone = state.zones.get(zone_idx)?;
        let template = state.zone_templates.iter().find(|t| t.id == zone.template_id)?;
        let target_id = template.upgrade_to.as_ref()?;
        let target = state.zone_templates.iter().find(|t| &t.id == target_id)?;
        
        (zone.template_id.clone(), target_id.clone(), target.construction_cost)
    };
    
    // Check affordability
    if state.resources.materials < cost {
        return None;
    }
    
    // Deduct cost
    state.resources.materials -= cost;
    
    // Update zone template
    if let Some(zone) = state.zones.get_mut(zone_idx) {
        zone.template_id = new_id;
        zone.condition = 1.0; // Freshly upgraded
    }
    
    // Log the upgrade
    let new_name = state.zone_templates.iter()
        .find(|t| t.id == state.zones[zone_idx].template_id)
        .map(|t| t.name.as_str())
        .unwrap_or("Unknown");
    
    state.log.add(
        state.game_time_hours,
        format!("Upgraded to {}!", new_name),
        crate::narrative::LogCategory::Zone,
    );
    
    Some(old_id)
}
