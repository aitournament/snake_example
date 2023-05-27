# Rust Snake Example

This repo contains an example snake implementation for [Snake](https://snake.aitournament.com).

## Building

1. ensure you have a recent version of Rust [installed](https://www.rust-lang.org/tools/install)
2. install the `wasm32-unknown-unknown` target for Rust `rustup target add wasm32-unknown-unknown`
3. compile with `cargo build --release --target=wasm32-unknown-unknown`
4. The wasm file is at `target/wasm32-unknown-unknown/release/snake_example.wasm`

## SDK

For the full list of functions available in the SDK, view the [docs](https://sdk.snake.aitournament.com/snake_sdk/)