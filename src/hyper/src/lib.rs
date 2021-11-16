#![allow(clippy::new_without_default)]

pub use crate::connector::SlackClientHyperConnector;
pub use crate::connector::SlackClientHyperHttpsConnector;
use slack_morphism::SlackClient;

pub mod connector;
pub mod listener;
pub mod scroller_ext;
mod socket_mode;

pub type SlackHyperClient = SlackClient<SlackClientHyperHttpsConnector>;

pub use listener::chain_service_routes_fn;
pub use listener::SlackClientEventsHyperListener;
pub use scroller_ext::SlackApiResponseScrollerExt;
pub use socket_mode::*;
