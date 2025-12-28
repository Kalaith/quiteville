//! Simulation module - The idle loop and time progression



pub use tick::{TickTimer, TimeTracker, process_offline_time, simulate_ticks};

pub mod tick;
pub mod map;
pub mod camera;
pub mod agents;

