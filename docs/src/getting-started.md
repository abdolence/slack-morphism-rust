## Getting Started

Cargo.toml dependencies example:

```toml
[dependencies]
slack-morphism="0.26"
slack-morphism-models="0.26"
slack-morphism-hyper="0.26"
```

All imports you need:

```rust,noplaypen
use slack_morphism::prelude::*;
use slack_morphism_hyper::*;
```

or more granularly:
```rust,noplaypen
use slack_morphism::*; // access to network/client functions
use slack_morphism::api::*; // Slack Web API methods (chat, users, views, etc)
use slack_morphism::listener::*; // Slack Events API listener (routes) implementation
use slack_morphism_models::*; // common Slack models like SlackUser, etc and macros
use slack_morphism_models::blocks::*; // Slack Block Kit models
use slack_morphism_models::events::*; // Slack Events Models

use slack_morphism_hyper::*; // Hyper/Tokio client implementation
```

## Ready to use examples
- Slack Web API client and Block kit example
- Events API server example
- Slack Web API client with Socket Mode

You can find them on [github](https://github.com/abdolence/slack-morphism-rust/tree/master/src/hyper/examples)
