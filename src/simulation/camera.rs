use macroquad::prelude::*;

// Wrapper around toolkit Camera2D to maintain API compatibility
#[derive(Debug, Clone)]
pub struct Camera2D {
    pub target: Vec2,
    pub zoom: f32,
    pub drag_start: Option<Vec2>,
    inner: macroquad_toolkit::camera::Camera2D,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            target: vec2(0.0, 0.0),
            zoom: 1.0,
            drag_start: None,
            inner: macroquad_toolkit::camera::Camera2D::new(vec2(0.0, 0.0), 1.0),
        }
    }

    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        self.inner.screen_to_world(point)
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, point: Vec2) -> Vec2 {
        self.inner.world_to_screen(point)
    }

    /// Handle Input (WASD + Mouse)
    pub fn update(&mut self, delta: f32, input_captured: bool) {
        // Sync public fields to inner camera
        self.inner.target = self.target;
        self.inner.zoom = self.zoom;

        // Update drag_start tracking for external access
        if is_mouse_button_pressed(MouseButton::Left) && !input_captured {
            self.drag_start = Some(mouse_position().into());
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.drag_start = None;
        }

        // Delegate to toolkit camera
        self.inner.update(delta, input_captured);

        // Sync back from inner camera
        self.target = self.inner.target;
        self.zoom = self.inner.zoom;
    }
}
