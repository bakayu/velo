//! # Domain
//!
//! Contains the core business logic and types for the application.
//! This module enforces invariants (like valid email formats) through the type system.

mod new_subscriber;
mod subscriber_email;
mod subscriber_name;

pub use new_subscriber::NewSubscriber;
pub use subscriber_email::SubscriberEmail;
pub use subscriber_name::SubscriberName;
