//! Floating text for resource change notifications

use macroquad::prelude::*;

/// A floating text popup that rises and fades
#[derive(Debug, Clone)]
pub struct FloatingText {
    pub text: String,
    pub pos: Vec2,
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl FloatingText {
    pub fn new(text: String, pos: Vec2, color: Color, lifetime: f32) -> Self {
        Self {
            text,
            pos,
            color,
            lifetime,
            max_lifetime: lifetime,
        }
    }
    
    /// Update and return true if still alive
    pub fn update(&mut self, delta: f32) -> bool {
        self.lifetime -= delta;
        self.pos.y -= 30.0 * delta; // Rise upward
        self.lifetime > 0.0
    }
    
    /// Get current alpha based on remaining lifetime
    pub fn alpha(&self) -> f32 {
        (self.lifetime / self.max_lifetime).clamp(0.0, 1.0)
    }
}

/// Manages a collection of floating text popups
#[derive(Debug, Default)]
pub struct FloatingTextManager {
    texts: Vec<FloatingText>,
}

impl FloatingTextManager {
    pub fn new() -> Self {
        Self { texts: Vec::new() }
    }
    
    /// Add a new floating text
    pub fn add(&mut self, text: String, world_pos: Vec2, color: Color) {
        self.texts.push(FloatingText::new(text, world_pos, color, 2.0));
    }
    
    /// Add a resource gain notification (green)
    pub fn add_gain(&mut self, amount: f32, resource_name: &str, world_pos: Vec2) {
        if amount.abs() < 0.01 { return; }
        let text = format!("+{:.1} {}", amount, resource_name);
        self.add(text, world_pos, GREEN);
    }
    
    /// Add a resource loss notification (red)
    pub fn add_loss(&mut self, amount: f32, resource_name: &str, world_pos: Vec2) {
        if amount.abs() < 0.01 { return; }
        let text = format!("-{:.1} {}", amount.abs(), resource_name);
        self.add(text, world_pos, RED);
    }
    
    /// Update all floating texts and remove expired ones
    pub fn update(&mut self, delta: f32) {
        self.texts.retain_mut(|t| t.update(delta));
    }
    
    /// Draw all floating texts (call after camera transform)
    pub fn draw(&self, camera: &crate::simulation::camera::Camera2D) {
        for text in &self.texts {
            let screen_pos = camera.world_to_screen(text.pos);
            let alpha = text.alpha();
            let color = Color::new(text.color.r, text.color.g, text.color.b, alpha);
            
            // Shadow for readability
            draw_text(&text.text, screen_pos.x + 1.0, screen_pos.y + 1.0, 20.0, 
                Color::new(0.0, 0.0, 0.0, alpha * 0.5));
            draw_text(&text.text, screen_pos.x, screen_pos.y, 20.0, color);
        }
    }
    
    /// Get the number of active texts
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.texts.len()
    }
}
