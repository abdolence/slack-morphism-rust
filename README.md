[![Cargo](https://img.shields.io/crates/v/slack_morphism.svg)](https://crates.io/crates/slack_morphism)
![tests and formatting](https://github.com/abdolence/slack-morphism-rust/workflows/tests%20&amp;%20formatting/badge.svg)
![security audit](https://github.com/abdolence/slack-morphism-rust/workflows/security%20audit/badge.svg)

# Slack Morphism for Rust

Slack Morphism is a modern client library for Slack Web/Events API/Socket Mode and Block Kit.

## Documentation
Please follow to the official website: https://slack-rust.abdolence.dev.

## Examples

https://github.com/abdolence/slack-morphism-rust/tree/master/examples

The examples require to work the following environment variables (from your Slack bot profile in api.slack.com):

- `SLACK_TEST_TOKEN` - for Slack client example
- `SLACK_TEST_APP_TOKEN` - for Slack client with Socket Mode example
- `SLACK_CLIENT_ID`, `SLACK_CLIENT_SECRET`, `SLACK_BOT_SCOPE`, `SLACK_REDIRECT_HOST` - for OAuth routes for Events API example
- `SLACK_SIGNING_SECRET` for all routes for Events API example

To run example use with environment variables:
```
# SLACK_... cargo run --example <client|events_api_server|axum_events_api_server|socket_mode> --all-features
```

Routes for this example are available on http://<your-host>:8080:

- /auth/install - to begin OAuth installation
- /auth/callback - a callback endpoint for Slack OAuth profile config
- /push - for Slack Push Events
- /interaction - for Slack Interaction Events
- /command - for Slack Command Events

### Testing with ngrok
For development/testing purposes you can use [ngrok](https://ngrok.com/):
```
ngrok http 8080
```
and copy the URL it gives for you to the example parameters for `SLACK_REDIRECT_HOST`.

Example testing with ngrok:
```
SLACK_CLIENT_ID=<your-client-id> \
SLACK_CLIENT_SECRET=<your-client-secret> \
SLACK_BOT_SCOPE=app_mentions:read,incoming-webhook \
SLACK_REDIRECT_HOST=https://<your-ngrok-url>.ngrok.io \
SLACK_SIGNING_SECRET=<your-signing-secret> \
cargo run --example events_api_server  --all-features
```

## Licence
Apache Software License (ASL)

## Author
Abdulla Abdurakhmanov
