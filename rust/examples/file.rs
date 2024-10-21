// Usage: cargo run --example file [<path_to_wav_file>]

use std::error::Error;

use aristech_stt_client::{
    get_client, recognize_file, stt_service::RecognitionSpec, Auth, TlsOptions,
};
use tonic::codegen::CompressionEncoding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let host = std::env::var("HOST")?;
    let token = std::env::var("TOKEN")?;
    let secret = std::env::var("SECRET")?;
    // For self-signed certificates we would need to read the certificate file into a string
    // and set ca_certificate to Some(ca_certificate_string)
    let root_cert = match std::env::var("ROOT_CERT") {
        Ok(root_cert) => match root_cert.is_empty() {
            true => None,
            false => {
                let root_cert = std::fs::read_to_string(root_cert)?;
                Some(root_cert)
            }
        },
        Err(_) => None,
    };
    let client = get_client(
        host,
        Some(TlsOptions {
            ca_certificate: root_cert,
            auth: Some(Auth { token, secret }),
        }),
    )
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
    let results = recognize_file(
        client,
        file_path,
        Some(RecognitionSpec {
            model: std::env::var("MODEL").unwrap_or_else(|_| "".to_string()),
            ..RecognitionSpec::default()
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
