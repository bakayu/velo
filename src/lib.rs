//! # Velo Library
//!
//! This library provides the core functionality for the Velo newsletter application.
//!
//! ## Modules
//!
//! - [`configuration`]: Handles reading and parsing application configuration from files and environment variables.
//! - [`domain`]: Contains the business logic and type definitions (e.g., `SubscriberEmail`, `SubscriberName`) that enforce domain invariants.
//! - [`routes`]: Defines the HTTP route handlers for the application endpoints.
//! - [`startup`]: Contains logic to bootstrap the application server and database connection.
//! - [`telemetry`]: Provides infrastructure for structured logging and distributed tracing.

pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;
