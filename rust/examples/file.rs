// Usage: cargo run --example file [<path_to_wav_file>]
use std::error::Error;

use aristech_stt_client::{
    recognize_file,
    stt_service::{RecognitionConfig, RecognitionSpec},
    SttClientBuilder,
};
use tonic::codegen::CompressionEncoding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let mut client = SttClientBuilder::new()
        .build()
        .await?
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);

    let file_path = std::env::args().nth(1).unwrap_or_else(|| {
        // Use test.wav from the repository root as default
        // Get the path to the rust directory
        let project_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = std::path::Path::new(project_dir)
            .join("..")
            .join("test.wav")
            .to_str()
            .unwrap()
            .to_string();
        file_path
    });
    // Assure the file ends with .wav
    assert!(
        file_path.ends_with(".wav"),
        "recognize_file only supports .wav files"
    );
    // Assure the file exists
    assert!(
        std::path::Path::new(&file_path).exists(),
        "File does not exist"
    );
    let file_path = file_path.as_str();

    // Get file relative to manifest directory
    let file_path = std::path::Path::new(file_path).to_str().unwrap();
    // The recognize_file function accepts a wav file and will set the sample rate for you
    let results = recognize_file(
        &mut client,
        file_path,
        Some(RecognitionConfig {
            specification: Some(RecognitionSpec {
                model: std::env::var("MODEL").unwrap_or_else(|_| "".to_string()),
                ..RecognitionSpec::default()
            }),
        }),
    )
    .await?;

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
