# Aristech STT-Client for Rust

This is the Rust client implementation for the Aristech STT-Server.

## Installation

To use the client in your project, add it to your `Cargo.toml` or use `cargo` to add it:

```sh
cargo add aristech-stt-client
```

## Usage

```rust
use aristech_stt_client::{get_client, recognize_file, TlsOptions, Auth};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = get_client(
    "https://stt.example.com",
    Some(TlsOptions {
        ca_certificate: None,
        auth: Some(Auth { token: "your-token", secret: "your-secret" }),
    }),
    ).await?;

    let results = recognize_file(&mut client, "path/to/audio/file.wav", None).await?;
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
    Ok(())
}
```

There are several examples in the [examples](https://github.com/aristech-de/stt-clients/blob/main/rust/examples/) directory:

- [file.rs](https://github.com/aristech-de/stt-clients/blob/main/rust/examples/file.rs): Demonstrates how to perform recognition on a file.
- [live.rs](https://github.com/aristech-de/stt-clients/blob/main/rust/examples/live.rs): Demonstrates how to perform live recognition using the microphone.
- [models.rs](https://github.com/aristech-de/stt-clients/blob/main/rust/examples/models.rs): Demonstrates how to get the available models from the server.
- [nlpFunctions.rs](https://github.com/aristech-de/stt-clients/blob/main/rust/examples/nlpFunctions.rs): Demonstrates how to list the configured NLP-Servers and the coresponding functions.
- [nlpProcess.rs](https://github.com/aristech-de/stt-clients/blob/main/rust/examples/nlpProcess.rs): Demonstrates how to perform NLP processing on a text by using the STT-Server as a proxy.
- [account.rs](https://github.com/aristech-de/stt-clients/blob/main/rust/examples/account.rs): Demonstrates how to retrieve the account information from the server.

You can run the examples directly using `cargo` like this:

1. Create a `.env` file in the [rust](.) directory:

```sh
HOST=stt.example.com
# The credentials are optional but probably required for most servers:
TOKEN=your-token
SECRET=your-secret

# The following are optional:
# ROOT_CERT=your-root-cert.pem # If the server uses a self-signed certificate
# If neither credentials nor an explicit root certificate are provided,
# you can still enable SSL by setting the SSL environment variable to true:
# SSL=true
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