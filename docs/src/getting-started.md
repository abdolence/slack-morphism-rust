## Getting Started

Cargo.toml dependencies example:

```toml
[dependencies]
slack-morphism="0.3"
slack-morphism-models="0.3"
```

All imports you need:

```rust,noplaypen
use slack_morphism::prelude::*;
```

or more granularly:
```rust,noplaypen
use slack_morphism::*; // access to network/client functions
use slack_morphism::api::*; // Slack Web API methods (chat, users, views, etc)
use slack_morphism::listener::*; // Slack Events API listener (routes) implementation
use slack_morphism_models::*; // common Slack models like SlackUser, etc and macros
use slack_morphism_models::blocks::*; // Slack Block Kit models
use slack_morphism_models::events::*; // Slack Events Models
```
