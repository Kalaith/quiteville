//! Data module - Core data structures and JSON config types
//!
//! All game balance numbers are defined in JSON, not hardcoded here.

mod config;
mod zone_template;
mod milestone;
mod state;
mod tech;

pub use config::*;
pub use zone_template::*;
pub use milestone::*;
pub use state::*;
pub use tech::*;

