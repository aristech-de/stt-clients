# Aristech STT-Client for Rust

This is the Rust client implementation for the Aristech STT-Server.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
aristech_stt_client = "1.0.0"
```

## Usage

```rust
let client = get_client(
  "https://stt.example.com",
  Some(TlsOptions {
      ca_certificate: None,
      auth: Some(Auth { token: "your-token", secret: "your-secret" }),
  }),
).await?;

let results = recognize_file(client, "path/to/audio/file.wav", None).await?;
for result in results {
    println!(
        "{}",
        result
            .chunks
            .get(0)
            .unwrap()
            .alternatives
            .get(0)
            .unwrap()
            .text
    );
}
```

There are several examples in the [examples](.) directory:

- [file.rs](examples/file.rs): Demonstrates how to perform recognition on a file.
- [live.rs](examples/live.rs): Demonstrates how to perform live recognition using the microphone.
- [models.rs](examples/models.rs): Demonstrates how to get the available models from the server.
- [nlpFunctions.rs](examples/nlpFunctions.rs): Demonstrates how to list the configured NLP-Servers and the coresponding functions.
- [nlpProcess.rs](examples/nlpProcess.rs): Demonstrates how to perform NLP processing on a text by using the STT-Server as a proxy.
- [account.rs](examples/account.rs): Demonstrates how to retrieve the account information from the server.

You can run the examples directly using `cargo` like this:

1. Create a `.env` file in the [rust](.) directory:

```sh
HOST=https://stt.example.com # Note: The protocol is required in the rust client
# The credentials are optional but probably required for most servers:
TOKEN=your-token
SECRET=your-secret

# The following are optional:
# ROOT_CERT=your-root-cert.pem # If the server uses a self-signed certificate
# SSL=true # Set to true if credentials are provided or if a ROOT_CERT is provided
# MODEL=some-available-model
# NLP_SERVER=some-config
# NLP_PIPELINE=function1,function2
```

2. Run the examples, e.g.:

```sh
cargo run --example live
```

## Build

To build the library, run:

```bash
cargo build
```