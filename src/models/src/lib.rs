//! # Slack Morphism models for Rust
//! This library contains pure models definitions to use with client and async module implementation.
//! Please follow to the official website: https://slack-rust.abdolence.dev for details.

// These warnings disabled mostly since it is important to replicate original models from Slack as is
// (not for 100% cases though).
// For example: some of the original models contains more fields then recommended for one model.
#![allow(clippy::large_enum_variant, clippy::too_many_arguments)]

mod common;

pub mod apps;
pub mod blocks;
pub mod events;
pub mod files;
mod messages;
pub mod socket_mode;

pub use apps::*;
pub use common::*;
pub use files::*;
pub use messages::*;
