 # Events API and OAuth for Hyper

 The library provides two different ways to work with Slack Events API:
 - Using [pure Hyper-based solution](./events-api-hyper.md)
 - Using more [high-level solution for axum web framework](./events-api-axum.md)

## Testing with ngrok
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
cargo run --example events_api_server --all-features
```

## Slack Signature Verifier
 The library provides Slack events signature verifier (`SlackEventSignatureVerifier`),
 which is already integrated in the OAuth routes implementation for you, and you don't need to use it directly.
 All you need is provide your client id and secret configuration to route implementation.
 Look at the [complete example here](https://github.com/abdolence/slack-morphism-rust/tree/master/src/hyper/examples/events_api_server.rs).

 In case you're embedding the library into your own Web/routes-framework, you can use it separately.
