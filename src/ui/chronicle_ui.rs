use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;
use crate::ui::theme::colors;
use crate::ui::theme;
use crate::narrative::TownRecord;

pub fn draw_chronicle_ui(state: &GameState, x: f32, y: f32, w: f32, h: f32) -> Option<PlayerAction> {
    theme::draw_panel(x, y, w, h);
    let mut action: Option<PlayerAction> = None;
    
    // Header
    theme::draw_header("The Chronicle", x + 20.0, y + 20.0);
    draw_text(&format!("Legacy: {}", state.dynasty.legacy_points), x + w - 200.0, y + 40.0, 26.0, GOLD);
    
    // Achievement count badge
    let ach_count = state.achievements.count();
    let ach_total = state.achievements.total();
    draw_text(&format!("Ach: {}/{}", ach_count, ach_total), x + w - 350.0, y + 40.0, 20.0, YELLOW);
    
    // Tab-like sections
    let content_y = y + 60.0;
    let content_h = h - 130.0;
    
    // Left half: Dynasty info (Towns, Heroes, Ancestors)
    let left_w = w * 0.55;
    draw_dynasty_section(state, x + 10.0, content_y, left_w, content_h, &mut action);
    
    // Right half: Achievements & Stats
    let right_x = x + left_w + 20.0;
    let right_w = w - left_w - 40.0;
    
    // Achievements section (top half of right side)
    draw_achievements_section(state, right_x, content_y, right_w, content_h * 0.45);
    
    // Stats section (bottom half of right side)
    draw_stats_section(state, right_x, content_y + content_h * 0.48, right_w, content_h * 0.52);
    
    // Close button - draw last to be on top, and make it larger for better click target
    let close_btn_w = 50.0;
    let close_btn_h = 35.0;
    let close_btn_x = x + w - close_btn_w - 10.0;
    let close_btn_y = y + 10.0;
    if theme::draw_button(close_btn_x, close_btn_y, close_btn_w, close_btn_h, "Close") {
        action = Some(PlayerAction::ToggleChronicle);
    }
    
    action
}

fn draw_dynasty_section(state: &GameState, x: f32, y: f32, w: f32, h: f32, action: &mut Option<PlayerAction>) {
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.1, 0.15, 0.8));
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    // Section header
    draw_text("Dynasty", x + 10.0, y + 20.0, 18.0, colors::ACCENT);
    
    let mut sy = y + 45.0;
    let line_h = 22.0;
    
    // Past towns
    draw_text(&format!("Past Towns: {}", state.dynasty.past_towns.len()), x + 10.0, sy, 14.0, WHITE);
    sy += line_h;
    
    // Hall of Heroes (recent)
    draw_text("Hall of Heroes:", x + 10.0, sy, 15.0, GOLD);
    sy += line_h;
    
    for hero in state.dynasty.hall_of_heroes.iter().take(5) {
        let text = format!("  {} - {}", hero.name, hero.description);
        draw_text(&text, x + 10.0, sy, 13.0, LIGHTGRAY);
        sy += line_h - 4.0;
    }
    if state.dynasty.hall_of_heroes.is_empty() {
        draw_text("  No heroes immortalized yet.", x + 10.0, sy, 12.0, GRAY);
        sy += line_h;
    }
    
    sy += 10.0;
    
    // Ancestors
    draw_text("Active Ancestors:", x + 10.0, sy, 15.0, colors::SECONDARY);
    sy += line_h;
    
    for ancestor in state.dynasty.ancestors.iter().take(4) {
        let buff_name = match ancestor.buff {
            crate::narrative::AncestorBuff::ProductionBoost => "Prod",
            crate::narrative::AncestorBuff::MoraleBoost => "Morale",
            crate::narrative::AncestorBuff::LuckBoost => "Luck",
            crate::narrative::AncestorBuff::GrowthBoost => "Growth",
        };
        let text = format!("  {} (+{})", ancestor.hero.name, buff_name);
        draw_text(&text, x + 10.0, sy, 12.0, LIGHTGRAY);
        sy += line_h - 4.0;
    }
    if state.dynasty.ancestors.is_empty() {
        draw_text("  No ancestors watching over yet.", x + 10.0, sy, 12.0, GRAY);
        sy += line_h;
    }
    
    sy += 15.0;
    
    // Wonders completed
    draw_text("Completed Wonders:", x + 10.0, sy, 15.0, YELLOW);
    sy += line_h;
    
    for wonder in state.dynasty.completed_wonders.iter().take(3) {
        draw_text(&format!("  {}", wonder.name()), x + 10.0, sy, 12.0, LIGHTGRAY);
        sy += line_h - 4.0;
    }
    if state.dynasty.completed_wonders.is_empty() {
        draw_text("  No wonders completed yet.", x + 10.0, sy, 12.0, GRAY);
    }
    
    // Retire hero button (if there's a top agent eligible)
    if !state.agents.is_empty() {
        let hero_btn_y = y + h - 45.0;
        if theme::draw_button(x + 10.0, hero_btn_y, 120.0, 30.0, "Retire Hero") {
            // Find top agent by spirit
            if let Some(agent) = state.agents.iter().max_by(|a, b| 
                a.spirit.partial_cmp(&b.spirit).unwrap_or(std::cmp::Ordering::Equal)
            ) {
                *action = Some(PlayerAction::RetireHero(agent.name.clone()));
            }
        }
    }
}

fn draw_achievements_section(state: &GameState, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.1, 0.15, 0.8));
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    draw_text("Achievements", x + 10.0, y + 20.0, 18.0, YELLOW);
    
    let unlocked = state.achievements.unlocked_list();
    let mut ax = x + 10.0;
    let mut ay = y + 40.0;
    
    if unlocked.is_empty() {
        draw_text("No achievements yet.", x + 10.0, ay, 14.0, GRAY);
        draw_text("Keep playing to unlock!", x + 10.0, ay + 18.0, 12.0, DARKGRAY);
    } else {
        for achievement in unlocked.iter().take(12) {
            // Draw achievement badge (wider to fit description)
            let badge_w = 150.0;
            draw_rectangle(ax, ay, badge_w - 5.0, 45.0, Color::new(0.2, 0.3, 0.2, 0.8));
            // draw_text(&achievement.icon, ax + 5.0, ay + 20.0, 18.0, WHITE);
            
            // Name
            draw_text(&achievement.name, ax + 28.0, ay + 15.0, 12.0, WHITE);
            
            // Description (truncated)
            let desc = &achievement.description;
            let display_desc = if desc.len() > 20 { &desc[..18] } else { desc };
            draw_text(display_desc, ax + 28.0, ay + 30.0, 9.0, GRAY);
            
            ax += badge_w;
            if ax + badge_w > x + w - 5.0 {
                ax = x + 10.0;
                ay += 50.0;
                if ay > y + h - 30.0 { break; }
            }
        }
    }
}

fn draw_stats_section(state: &GameState, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.12, 0.1, 0.8));
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    draw_text("Lifetime Stats", x + 10.0, y + 20.0, 18.0, colors::ACCENT);
    
    let stats = &state.stats;
    let col1_x = x + 10.0;
    let col2_x = x + w / 2.0;
    let mut sy = y + 40.0;
    let line_h = 18.0;
    
    // Column 1
    draw_text(&format!("Zones Restored: {}", stats.zones_restored), col1_x, sy, 13.0, WHITE);
    sy += line_h;
    draw_text(&format!("Resources: {:.0}", stats.resources_collected), col1_x, sy, 13.0, WHITE);
    sy += line_h;
    draw_text(&format!("Peak Resources: {:.0}", stats.peak_resources), col1_x, sy, 13.0, WHITE);
    sy += line_h;
    draw_text(&format!("Peak Pop: {}", stats.peak_population), col1_x, sy, 13.0, WHITE);
    sy += line_h;
    draw_text(&format!("Play Time: {:.1}h", stats.total_play_hours), col1_x, sy, 13.0, WHITE);
    
    // Column 2
    sy = y + 40.0;
    draw_text(&format!("Agents Born: {}", stats.agents_born), col2_x, sy, 13.0, WHITE);
    sy += line_h;
    draw_text(&format!("Agents Died: {}", stats.agents_died), col2_x, sy, 13.0, WHITE);
    sy += line_h;
    draw_text(&format!("Techs: {}", stats.techs_researched), col2_x, sy, 13.0, WHITE);
    sy += line_h;
    draw_text(&format!("Heroes: {}", stats.heroes_immortalized), col2_x, sy, 13.0, WHITE);
    sy += line_h;
    draw_text(&format!("Wonders: {}", stats.wonders_built), col2_x, sy, 13.0, WHITE);
}

fn draw_town_entry(town: &TownRecord, x: f32, y: f32) {
    draw_text(&town.name, x, y + 14.0, 14.0, WHITE);
    draw_text(&format!("Pop: {} • {}", town.population, town.outcome), x, y + 28.0, 11.0, LIGHTGRAY);
}
