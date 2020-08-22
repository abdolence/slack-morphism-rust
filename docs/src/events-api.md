 ## Events API and OAuth support

 The library provides route implementation in `SlackClientEventsListener` based on Hyper/Tokio for:
 - Push Events
 - Interaction Events
 - Command Events
 - OAuth v2 redirects and client functions

 You can chain all of the routes using `chain_service_routes_fn` from the library.

 Also the library provides Slack events signature verifier (`SlackEventSignatureVerifier`)
 (which is already integrated in the routes implementation for you).
 All you need is provide your client id and secret configuration to route implementation.

 Look at the examples/test_server sources for the details.
 