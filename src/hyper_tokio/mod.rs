//! # Slack Morphism models for Rust
//! This library contains the client implementation for Hyper/Tokio.
//! Please follow to the official website: https://slack-rust.abdolence.dev for details.

#![allow(clippy::new_without_default)]

pub use crate::hyper_tokio::connector::SlackClientHyperConnector;
pub use crate::hyper_tokio::connector::SlackClientHyperHttpsConnector;
use crate::SlackClient;

use crate::*;

pub mod connector;
pub mod hyper_errors;
pub(crate) mod hyper_ext;
pub mod listener;
mod ratectl;
pub mod scroller_ext;
mod socket_mode;

use crate::listener::SlackClientEventsListenerEnvironment;
pub use listener::chain_service_routes_fn;
pub use listener::SlackClientEventsHyperListener;
pub use scroller_ext::SlackApiResponseScrollerExt;
pub use socket_mode::*;

pub type SlackHyperClient = SlackClient<SlackClientHyperHttpsConnector>;
pub type SlackHyperListenerEnvironment =
    SlackClientEventsListenerEnvironment<SlackClientHyperHttpsConnector>;
