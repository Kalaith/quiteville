//! Simulation module - The idle loop and time progression

pub use tick::{simulate_ticks, TickTimer};

pub mod agents;
pub mod camera;
pub mod map;
pub mod seasons;
pub mod tick;
pub mod traits;
