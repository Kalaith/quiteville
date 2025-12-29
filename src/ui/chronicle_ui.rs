use macroquad::prelude::*;
use crate::data::GameState;
use crate::PlayerAction;
use crate::ui::colors;
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
    let ach_total = crate::data::AchievementManager::total();
    draw_text(&format!("🏆 {}/{}", ach_count, ach_total), x + w - 350.0, y + 40.0, 20.0, YELLOW);
    
    // Tab-like sections
    let content_y = y + 60.0;
    let content_h = h - 130.0;
    
    // Left half: Dynasty info (Towns, Heroes, Ancestors)
    let left_w = w * 0.55;
    draw_dynasty_section(state, x + 10.0, content_y, left_w, content_h, &mut action);
    
    // Right half: Achievements & Stats
    let right_x = x + left_w + 20.0;
    let right_w = w - left_w - 40.0;
    draw_achievements_section(state, right_x, content_y, right_w, content_h / 2.0 - 10.0);
    draw_stats_section(state, right_x, content_y + content_h / 2.0, right_w, content_h / 2.0 - 10.0);
    
    // Close Button
    let btn_w = 120.0;
    let btn_h = 35.0;
    let btn_x = x + (w - btn_w) / 2.0;
    let btn_y = y + h - 50.0;
    
    if theme::draw_button(btn_x, btn_y, btn_w, btn_h, "Close") {
        return Some(PlayerAction::ToggleChronicle); 
    }
    
    action
}

fn draw_dynasty_section(state: &GameState, x: f32, y: f32, w: f32, h: f32, action: &mut Option<PlayerAction>) {
    // Mini stats row
    draw_text(&format!("Towns: {}", state.dynasty.past_towns.len() + 1), x + 5.0, y + 15.0, 14.0, WHITE);
    draw_text(&format!("Heroes: {}", state.dynasty.hall_of_heroes.len()), x + 100.0, y + 15.0, 14.0, WHITE);
    draw_text(&format!("Ancestors: {}", state.dynasty.ancestors.len()), x + 200.0, y + 15.0, 14.0, WHITE);
    
    let col_w = (w - 20.0) / 3.0;
    let list_y = y + 30.0;
    let list_h = h - 40.0;
    
    // Column 1: Past Towns
    draw_text("Past Towns", x, list_y - 5.0, 16.0, LIGHTGRAY);
    draw_rectangle_lines(x, list_y, col_w, list_h, 1.0, GRAY);
    
    let mut item_y = list_y + 8.0;
    if state.dynasty.past_towns.is_empty() {
        draw_text("No past towns.", x + 5.0, item_y + 15.0, 12.0, GRAY);
    } else {
        for town in &state.dynasty.past_towns {
             draw_town_entry(town, x + 5.0, item_y);
             item_y += 35.0;
             if item_y > list_y + list_h - 20.0 { break; }
        }
    }
    
    // Column 2: Heroes
    let col2_x = x + col_w + 10.0;
    draw_text("Hall of Heroes", col2_x, list_y - 5.0, 16.0, LIGHTGRAY);
    draw_rectangle_lines(col2_x, list_y, col_w, list_h, 1.0, GRAY);
    
    let mut hero_y = list_y + 8.0;
    if state.dynasty.hall_of_heroes.is_empty() {
        draw_text("No heroes yet.", col2_x + 5.0, hero_y + 15.0, 12.0, GRAY);
    } else {
        for hero in &state.dynasty.hall_of_heroes {
            draw_text(&format!("★ {}", hero.name), col2_x + 5.0, hero_y + 14.0, 14.0, GOLD);
            
            let retire_btn_x = col2_x + col_w - 55.0;
            if theme::draw_button(retire_btn_x, hero_y + 2.0, 50.0, 20.0, "Retire") {
                *action = Some(PlayerAction::RetireHero(hero.name.clone()));
            }
            
            hero_y += 30.0;
            if hero_y > list_y + list_h - 25.0 { break; }
        }
    }
    
    // Column 3: Ancestors
    let col3_x = col2_x + col_w + 10.0;
    draw_text("Ancestors", col3_x, list_y - 5.0, 16.0, LIGHTGRAY);
    draw_rectangle_lines(col3_x, list_y, col_w, list_h, 1.0, GRAY);
    
    let mut anc_y = list_y + 8.0;
    if state.dynasty.ancestors.is_empty() {
        draw_text("Retire heroes", col3_x + 5.0, anc_y + 15.0, 12.0, GRAY);
        draw_text("for buffs.", col3_x + 5.0, anc_y + 30.0, 12.0, GRAY);
    } else {
        for ancestor in &state.dynasty.ancestors {
            draw_text(&ancestor.hero.name, col3_x + 5.0, anc_y + 14.0, 13.0, PURPLE);
            draw_text(ancestor.buff.name(), col3_x + 10.0, anc_y + 28.0, 11.0, GREEN);
            
            anc_y += 40.0;
            if anc_y > list_y + list_h - 35.0 { break; }
        }
    }
}

fn draw_achievements_section(state: &GameState, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.1, 0.15, 0.8));
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    draw_text("🏆 Achievements", x + 10.0, y + 20.0, 18.0, YELLOW);
    
    let unlocked = state.achievements.unlocked_list();
    let mut ax = x + 10.0;
    let mut ay = y + 40.0;
    let badge_w = 80.0;
    
    if unlocked.is_empty() {
        draw_text("No achievements yet.", x + 10.0, ay, 14.0, GRAY);
        draw_text("Keep playing to unlock!", x + 10.0, ay + 18.0, 12.0, DARKGRAY);
    } else {
        for achievement in unlocked.iter().take(12) {
            // Draw achievement badge
            draw_rectangle(ax, ay, badge_w - 5.0, 35.0, Color::new(0.2, 0.3, 0.2, 0.8));
            draw_text(achievement.icon(), ax + 5.0, ay + 22.0, 18.0, WHITE);
            
            // Truncate name if too long
            let name = achievement.name();
            let display_name = if name.len() > 10 { &name[..9] } else { name };
            draw_text(display_name, ax + 25.0, ay + 15.0, 10.0, WHITE);
            
            ax += badge_w;
            if ax + badge_w > x + w - 5.0 {
                ax = x + 10.0;
                ay += 40.0;
                if ay > y + h - 20.0 { break; }
            }
        }
    }
}

fn draw_stats_section(state: &GameState, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.12, 0.1, 0.8));
    draw_rectangle_lines(x, y, w, h, 1.0, GRAY);
    
    draw_text("📊 Lifetime Stats", x + 10.0, y + 20.0, 18.0, colors::ACCENT);
    
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
