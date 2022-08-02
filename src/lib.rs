//! # Slack Morphism for Rust
//!
//! Slack Morphism is a modern client library for Slack Web/Events API and Block Kit.
//!
//! ## Slack Web API client
//!
//! ### Create a client instance:
//! ```ignore
//! use slack_morphism::prelude::*;
//! use slack_morphism_hyper::*;
//!
//! let client = SlackClient::new(SlackClientHyperConnector::new());
//!
//! ```
//!
//! ### Make Web API methods calls
//!
//! For most of Slack Web API methods (except for OAuth methods, Incoming Webhooks and event replies)
//! you need a Slack token to make a call.
//! For simple bots you can have it in your config files, or you can obtain
//! workspace tokens using Slack OAuth.
//!
//! In the example below, we’re using a hardcoded Slack token, but don’t do that for your production bots and apps.
//! You should securely and properly store all of Slack tokens.
//!
//! ```ignore
//!
//! use slack_morphism::prelude::*;
//! use slack_morphism_hyper::*;
//! use slack_morphism_models::*;
//!
//!
//!# async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!
//! let client = SlackClient::new(SlackClientHyperConnector::new());
//!
//! // Create our Slack API token
//! let token_value: SlackApiTokenValue = "xoxb-89.....".into();
//! let token: SlackApiToken = SlackApiToken::new(token_value);
//!
//! // Create a Slack session with this token
//! // A session is just a lightweight wrapper around your token
//! // not to specify it all the time for series of calls.
//! let session = client.open_session(&token);
//!
//! // Make your first API call (which is `api.test` here)
//! let test: SlackApiTestResponse = session
//!         .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
//!         .await?;
//!
//! // Send a simple text message
//! let post_chat_req =
//!     SlackApiChatPostMessageRequest::new("#general".into(),
//!            SlackMessageContent::new().with_text("Hey there!".into())
//!     );
//!
//! let post_chat_resp = session.chat_post_message(&post_chat_req).await?;
//!
//!# Ok(())
//!# }
//!
//! ```
//!
//! ## Events API and OAuth support
//!
//! The library provides route implementation in `SlackClientEventsListener` based on Hyper/Tokio for:
//! - Push Events
//! - Interaction Events
//! - Command Events
//! - OAuth v2 redirects and client functions
//!
//! You can chain all of the routes using `chain_service_routes_fn` from the library.
//!
//! Also the library provides Slack events signature verifier (`SlackEventSignatureVerifier`)
//! (which is already integrated in the routes implementation for you).
//! All you need is provide your client id and secret configuration to route implementation.
//!
//! ## Socket Mode support
//!
//! The library provides Socket Mode support additionally Events API leveraging Web-sockets
//! in cases you don't want/need to expose publicly available HTTP endpoint.
//!
//! # Docs and examples
//!
//! Please follow to the official website: https://slack-rust.abdolence.dev
//! Examples available at: https://github.com/abdolence/slack-morphism-rust/tree/master/src/hyper/examples
//!

#![allow(clippy::new_without_default)]

pub use client::*;
pub use scroller::*;
pub use socket_mode::*;
pub use token::*;

mod models;
pub use models::*;

pub mod api;
mod client;
pub mod errors;
pub mod listener;
mod ratectl;
mod scroller;
pub mod signature_verifier;
mod socket_mode;
mod token;

#[cfg(feature = "hyper")]
pub mod hyper_tokio;

// #[cfg(feature = "axum")]
// pub mod axum_support;

pub mod prelude;
