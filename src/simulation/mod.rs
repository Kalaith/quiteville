//! Simulation module - The idle loop and time progression

pub mod tick;

pub use tick::{TickTimer, TimeTracker, process_offline_time, simulate_ticks};

