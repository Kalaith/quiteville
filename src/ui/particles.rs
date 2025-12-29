use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ParticleType {
    Smoke,
    Rain,
    Snow,
    Dust,
    Heart,
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32, // Remaining time in seconds
    pub max_lifetime: f32,
    pub size: f32,
    pub color: Color,
    pub particle_type: ParticleType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleSystem {
    #[serde(skip)]
    pub particles: Vec<Particle>,
    #[serde(skip)]
    pub max_particles: usize,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    pub fn spawn(&mut self, pos: Vec2, velocity: Vec2, lifetime: f32, size: f32, color: Color, p_type: ParticleType) {
        if self.particles.len() >= self.max_particles {
            // Remove oldest if full (simple ring buffer approximation by removing index 0)
            // For better performance in large systems, use a ring buffer or index tracking, 
            // but for <1000 particles, Vec::remove(0) is acceptable if not per-frame spam.
            // Actually, randomly replacing one or just ignoring new ones might be better for performance?
            // Let's just ignore for now to avoid O(N) shift.
            return;
        }

        self.particles.push(Particle {
            pos,
            velocity,
            lifetime,
            max_lifetime: lifetime,
            size,
            color,
            particle_type: p_type,
        });
    }

    pub fn update(&mut self, delta: f32) {
        // Use retain to filter out dead particles in one pass
        self.particles.retain_mut(|p| {
            p.lifetime -= delta;
            if p.lifetime <= 0.0 {
                return false;
            }

            // Update position
            p.pos += p.velocity * delta;

            // Type specific updates
            match p.particle_type {
                ParticleType::Smoke => {
                    // Smoke rises and expands
                    p.velocity.y -= 10.0 * delta; // Buoyancy
                    p.size += 5.0 * delta; // Expand
                    p.color.a = (p.lifetime / p.max_lifetime).powf(0.5); // Fade out
                },
                ParticleType::Rain => {
                    // Rain falls straight
                },
                ParticleType::Snow => {
                    // Snow drifts
                    p.velocity.x += macroquad::rand::gen_range(-10.0, 10.0) * delta;
                },
                ParticleType::Dust => {
                    p.color.a = p.lifetime / p.max_lifetime;
                },
                ParticleType::Heart => {
                    p.velocity.y -= 20.0 * delta; // Float up
                    p.color.a = p.lifetime / p.max_lifetime;
                },
            }

            true
        });
    }

    pub fn draw(&self, camera: &crate::simulation::camera::Camera2D) {
        for p in &self.particles {
            let screen_pos = camera.world_to_screen(p.pos);
            let size = p.size * camera.zoom;
            
            // Culling
            if screen_pos.x + size < 0.0 || screen_pos.x > screen_width() ||
               screen_pos.y + size < 0.0 || screen_pos.y > screen_height() {
                continue;
            }

            match p.particle_type {
                ParticleType::Rain => {
                     draw_line(screen_pos.x, screen_pos.y, screen_pos.x - 2.0 * camera.zoom, screen_pos.y + 10.0 * camera.zoom, 1.0, p.color);
                },
                ParticleType::Heart => {
                    // Draw a simple heart shape or circle
                    draw_text("♥", screen_pos.x, screen_pos.y, size, p.color);
                },
                _ => {
                    draw_rectangle(screen_pos.x, screen_pos.y, size, size, p.color);
                }
            }
        }
    }
}
