# Intro and motivation

## Type-safety 
All of the models, API and Block Kit support in Slack Morphism are well-typed.

## Easy to use
The library depends only on familiar for Rust developers principles and libraries like Serde, futures, hyper.

## Async
Using latest Rust async/await language features and libraries, the library provides access to all of the functions 
in asynchronous manner.

## Frameworks-agnostic inner design

This library provided in multiple modules:
- `slack-morphism-models`, gives you access to all type/models definitions that used for Slack Web/Events APIs.
- `slack-morphism`, base module to support frameworks-agnostic client, that doesn't have any dependency to any HTTP/async library itself and you can implement binding to any library you want.
- `slack-morphism-hyper`, Slack client support/binding for Hyper/Tokio.
