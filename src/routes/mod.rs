//! # Routes
//!
//! Aggregates all application route handlers.

pub mod health_check;
pub mod subscribe;
pub use health_check::*;
pub use subscribe::*;
