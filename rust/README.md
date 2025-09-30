# Aristech STT-Client for Rust

This is the Rust client implementation for the Aristech STT-Server.

## Installation

To use the client in your project, add it to your `Cargo.toml` or use `cargo` to add it:

```sh
cargo add aristech-stt-client
```

## Usage

```rust
use aristech_stt_client::{SttClientBuilder, recognize_file};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Creating a client like this will attempt to parse the API key from the environment variable `ARISTECH_STT_API_KEY`.
    // If the environment variable is not set or invalid, it will fall back to default values.
    let mut client = SttClientBuilder::new()
        .build()
        .await?;

    // To manually specify the API key and catch invalid API keys, use the default builder and the `api_key` method.
    // let mut client = SttClientBuilder::default() // <- won't attempt to parse the API key from the environment variable
    //     .api_key("at-abc123...")? // <- will return an error if the API key is invalid
    //     .build()
    //     .await?;

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


To run the examples, use `cargo`. For example:

```sh
cargo run --example live
```

### API Key

If you didn't get an API key but a token, secret and host instead, you can simply convert those values with our [API key generator](https://www.aristech.de/api-key-generator/?type=stt).

<details>

<summary>Alternatively you can still provide the connection options manually.</summary>

```rust
use aristech_stt_client::{SttClientBuilder, Auth};

let mut client = SttClientBuilder::default()
    .host("https://stt.example.com:443")?
    .auth(Some(Auth {
        token: "your-token".to_string(),
        secret: "your-secret".to_string(),
    }))
    .build()
    .await?;
```

## Build

To build the library, run:

```bash
cargo build
```