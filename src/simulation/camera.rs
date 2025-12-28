use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Camera2D {
    pub target: Vec2,
    pub zoom: f32,
    pub offset: Vec2,
    
    // Drag state
    pub drag_start: Option<Vec2>,
    pub cam_start: Vec2,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            target: vec2(0.0, 0.0),
            zoom: 1.0,
            offset: vec2(0.0, 0.0),
            drag_start: None,
            cam_start: vec2(0.0, 0.0),
        }
    }
}

impl Camera2D {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let local = point - center;
        (local / self.zoom) + self.target
    }
    
    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, point: Vec2) -> Vec2 {
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let local = point - self.target;
        (local * self.zoom) + center
    }
    
    /// Handle Input (WASD + Mouse)
    pub fn update(&mut self, delta: f32) {
        let speed = 500.0 / self.zoom;
        
        // Keyboard Pan
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { self.target.y -= speed * delta; }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { self.target.y += speed * delta; }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { self.target.x -= speed * delta; }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { self.target.x += speed * delta; }
        
        // Mouse Drag (Reliable Origin-based)
        if is_mouse_button_pressed(MouseButton::Left) {
            self.drag_start = Some(mouse_position().into());
            self.cam_start = self.target;
        }
        
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(start) = self.drag_start {
                let current: Vec2 = mouse_position().into();
                let screen_delta = current - start;
                // Convert screen delta to world delta
                let world_delta = screen_delta / self.zoom;
                
                // Move target opposite to drag to pull map
                self.target = self.cam_start - world_delta;
            }
        }
        
        if is_mouse_button_released(MouseButton::Left) {
            self.drag_start = None;
        }
        
        // Mouse Wheel Zoom
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            // Smooth zoom around center? 
            // For now just basic zoom
            let zoom_factor = if wheel_y > 0.0 { 1.1 } else { 0.9 };
            self.zoom *= zoom_factor;
        }
        
        // Keyboard Zoom
        if is_key_down(KeyCode::Equal) { self.zoom *= 1.0 + delta; }
        if is_key_down(KeyCode::Minus) { self.zoom *= 1.0 - delta; }
        
        // Clamp zoom (Wider range: 0.1 to 10.0)
        self.zoom = self.zoom.clamp(0.1, 10.0);
    }
}
