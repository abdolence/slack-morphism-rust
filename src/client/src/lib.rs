//! # Slack Morphism for Rust
//!
//! Slack Morphism is a modern client library for Slack Web/Events API and Block Kit.
//!
//! ## Intro and motivation
//!
//! - *Type-safety*: All of the models, API and Block Kit support in Slack Morphism is well-typed.
//! - *Easy to use*: The library depends only on familiar for Rust developers principles and libraries like Serde, hyper, futures.
//! - *Async*: Using latest Rust async/.await language features and libraries, the library provides access to all of the functions in asynchronous manner
//!
//! ## Getting Started
//!
//! Cargo.toml dependencies example:
//!
//! ```toml
//!
//! [dependencies]
//! slack-morphism="0.1"
//! slack-morphism-models="0.1"
//!
//! ```
//!
//! All imports you need:
//!
//! ```rust
//! use slack_morphism::*; // access to network/client functions
//! use slack_morphism::api::*; // Slack Web API methods (chat, users, views, etc)
//! use slack_morphism::listener::*; // Slack Events API listener (routes) implementation
//! use slack_morphism_models::*; // common models and Block Kit macros
//! ```
//!
//! ## Slack Web API client
//!
//! ### Create a client instance:
//! ```rust
//! use slack_morphism::*;
//!
//! let client = SlackClient::new();
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
//! ```rust
//!
//! use slack_morphism::*;
//! use slack_morphism::api::*;
//!
//! let client = SlackClient::new();
//!
//! // Create our Slack API token
//! let token_value: SlackApiTokenValue = "xoxb-89.....".into();
//! let token: SlackApiToken = SlackApiToken::new(token_value);
//!
//! // Create a Slack session with this token
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
//! ```
//!
//! ### Pagination support
//! Some Web API methods defines cursors and pagination, to give you an ability to load a lot of data continually
//! (using batching and continually making many requests).
//!
//! Examples: `conversations.history`, `conversations.list`, `users.list`, ...
//!
//! To help with those methods Slack Morphism provides additional a “scroller” implementation,
//! which deal with all scrolling/batching requests for you.
//!
//! For example for `users.list`:
//!
//! ```rust
//!
//! use slack_morphism::*;
//! use slack_morphism::api::*;
//! use slack_morphism_models::*;
//! use std::time::Duration;
//!
//! let client = SlackClient::new();
//! let token_value: SlackApiTokenValue = "xoxb-89.....".into();
//! let token: SlackApiToken = SlackApiToken::new(token_value);
//! let session = client.open_session(&token);
//!
//! // Create a first request and specify a batch limit:
//! let scroller_req: SlackApiUsersListRequest = SlackApiUsersListRequest::new().with_limit(5);
//!
//! // Create a scroller from this request
//! let scroller = scroller_req.scroller();
//!
//! // Option 1: Create a Rust Futures Stream from this scroller and use it
//! use futures::stream::BoxStream;
//! use futures::TryStreamExt;
//!
//! let mut items_stream = scroller.to_items_stream(&session);
//! while let Some(items) = items_stream.try_next().await? {
//!     println!("users batch: {:#?}", items);
//! }
//!
//! // Option 2: Collect all of the data in a vector (which internally uses the same approach above)
//! let collected_members: Vec<SlackUser> = scroller
//!     .collect_items_stream(&session, Duration::from_millis(1000))
//!     .await?;
//!
//! ```
//!
//! ### Block Kit support
//!
//! To support Slack Block Kit rich messages and views, the library provides:
//! - Well-typed models
//! - Macros to help build blocks, block elements
//!
//! Let’s take some very simple block example:
//!
//! ```json
//! {
//!   "blocks": [
//!       {
//!         "type": "section",
//!         "text": {
//!             "type": "mrkdwn",
//!             "text": "A message *with some bold text* and _some italicized text_."
//!         }
//!       }
//!   ]
//! }
//! ```
//!
//! Now, let’s look at how it looks with type-safe code using Slack Morphism Blocks macro support:
//!
//! ```rust
//! use slack_morphism_models::*;
//!
//! slack_blocks![
//!  some_into(
//!     SlackSectionBlock::new()
//!         .with_text(md!("A message *with some bold text* and _some italicized text_."))
//!  )
//! ]
//! ```
//!
//! Let’s look at another more complex example for welcoming message:
//!
//! ```rust
//!
//! use slack_morphism::*;
//! use slack_morphism::api::*;
//! use slack_morphism_models::*;
//! use std::time::Duration;
//! use rsb_derive::Builder;
//!
//! let client = SlackClient::new();
//! let token_value: SlackApiTokenValue = "xoxb-89.....".into();
//! let token: SlackApiToken = SlackApiToken::new(token_value);
//! let session = client.open_session(&token);
//!
//! // Our welcome message params as a struct
//! #[derive(Debug, Clone, Builder)]
//! pub struct WelcomeMessageTemplateParams {
//!     pub user_id: SlackUserId,
//! }
//!
//! // Define our welcome message template using block macros, a trait and models from the library
//! impl SlackMessageTemplate for WelcomeMessageTemplateParams {
//!     fn render_template(&self) -> SlackMessageContent {
//!         SlackMessageContent::new()
//!             .with_text(format!("Hey {}", self.user_id.to_slack_format()))
//!             .with_blocks(slack_blocks![
//!                 some_into(
//!                     SlackSectionBlock::new()
//!                         .with_text(md!("Hey {}", self.user_id.to_slack_format()))
//!                 ),
//!                 some_into(SlackDividerBlock::new())
//!                 some_into(SlackImageBlock::new(
//!                     "https://www.gstatic.com/webp/gallery3/2_webp_ll.png".into(),
//!                     "Test Image".into()
//!                 )),
//!                 some_into(SlackActionsBlock::new(slack_blocks![some_into(
//!                     SlackBlockButtonElement::new(
//!                         "simple-message-button".into(),
//!                         pt!("Simple button text")
//!                     )
//!                 )]))
//!             ])
//!     }
//! }
//!
//! // Use it
//! let message = WelcomeMessageTemplateParams::new("some-slack-uid".into());
//!
//! let post_chat_req =
//!     SlackApiChatPostMessageRequest::new("#general".into(), message.render_template());
//!
//! ```
//! Look other examples in examples/templates.rs
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
//! Look at the examples/test_server sources for the details.
//!
//!
//! ## Limitations
//!
//! Slack Morphism doesn't provide:
//! - RTM API (the usage of which is slowly declining in favour of Events API)
//! - Legacy Web/Events API methods and models (like Slack Message attachments, which should be replaced with Slack Blocks)
//!

pub mod api;
mod client;
pub mod errors;
pub mod listener;
mod scroller;
mod token;

pub use client::*;
pub use scroller::*;
pub use token::*;
