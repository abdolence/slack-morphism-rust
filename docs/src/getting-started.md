## Getting Started

Cargo.toml dependencies example:

```toml
[dependencies]
slack-morphism = { version = "1.14", features = ["hyper", "axum"] }
```

All imports you need:

```rust,noplaypen
use slack_morphism::prelude::*;
```

## Ready to use examples
- Slack Web API client and Block kit example
- Events API server example using either pure hyper solution or axum
- Slack Web API client with Socket Mode

You can find them on [github](https://github.com/abdolence/slack-morphism-rust/tree/master/examples)
