# Aristech STT-Client for Rust

This is the Rust client implementation for the Aristech STT-Server.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aristech_stt_client = "1.0.0"
```

## Usage

There are several examples in the `examples` directory which can be run with `cargo run --example <example>`:

- [live.rs](examples/live.rs): Demonstrates how to perform live recognition using the microphone.
- [file.rs](examples/file.rs): Demonstrates how to perform recognition on a file.
- [models.rs](examples/models.rs): Demonstrates how to get the available models from the server.