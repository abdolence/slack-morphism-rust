# Intro and motivation

Slack Morphism is a modern client library for Slack Web/Events API/Sockets Mode and Block Kit.

## Type-safety 
All of the models, API and Block Kit support in Slack Morphism are well-typed.

## Easy to use
The library depends only on familiar for Rust developers principles and libraries like Serde, futures, hyper.

## Async
Using latest Rust async/await language features and libraries, the library provides access to all of the functions 
in asynchronous manner.

## Modular design

Base crate to support frameworks-agnostic client and that doesn't have any dependency to any HTTP/async library itself, and you can implement binding to any library you want.
Includes also all type/models definitions that used for Slack Web/Events APIs.

This library provided the following features:
- `hyper`: Slack client support/binding for Hyper/Tokio/Tungstenite.
- `axum`:  Slack client support/binding for [axum framework](https://github.com/tokio-rs/axum) support (WIP, will be available in next releases).
