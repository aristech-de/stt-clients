use std::error::Error;

use aristech_stt_client::{
    get_client,
    stt_service::{
        streaming_recognition_request, RecognitionConfig, RecognitionSpec,
        StreamingRecognitionRequest,
    },
    Auth, SttClient, TlsOptions,
};
use tonic::codegen::CompressionEncoding;

pub async fn streaming_recognize(
    mut client: SttClient,
    audio_path: &str,
    locale: &str,
) -> Result<(), Box<dyn Error>> {
    let wav_reader = hound::WavReader::open(audio_path)?;
    let sample_rate_hertz = wav_reader.spec().sample_rate as i64;
    println!("Sample rate: {}", sample_rate_hertz);

    let config = RecognitionConfig {
        specification: Some(RecognitionSpec {
            audio_encoding: 1,
            sample_rate_hertz,
            locale: locale.to_string(),
            graph: "".to_string(),
            grammar: "".to_string(),
            partial_results: true,
            single_utterance: false,
            normalization: None,
            phones: false,
            model: "".to_string(),
            endpointing: None,
            vad: None,
        }),
    };
    let initial_request = StreamingRecognitionRequest {
        streaming_request: Some(streaming_recognition_request::StreamingRequest::Config(
            config,
        )),
    };
    let audio_content = std::fs::read(audio_path)?;
    // Remove the header of the wav file
    let audio_content = &audio_content[44..];
    let audio_request = StreamingRecognitionRequest {
        streaming_request: Some(
            streaming_recognition_request::StreamingRequest::AudioContent(audio_content.to_vec()),
        ),
    };

    // Create an tokio_stream where the first item is the initial request
    let input_stream = tokio_stream::iter(vec![initial_request, audio_request]);

    let mut stream = client.streaming_recognize(input_stream).await?.into_inner();

    while let Some(response) = stream.message().await? {
        for chunk in response.chunks {
            let response_type = match chunk.end_of_utterance {
                true => "Final",
                false => "Partial",
            };
            for alternative in chunk.alternatives {
                println!(
                    "ClientID {}, Locale: {}, {}: {}",
                    response.client_id, response.language, response_type, alternative.text
                );
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host = std::env::var("HOST")?;
    let token = std::env::var("TOKEN")?;
    let secret = std::env::var("SECRET")?;
    // For self-signed certificates we would need to read the certificate file into a string
    // and set ca_certificate to Some(ca_certificate_string)
    let root_cert = match std::env::var("ROOT_CERT") {
        Ok(root_cert) => {
            let root_cert = std::fs::read_to_string(root_cert)?;
            Some(root_cert)
        }
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

    let file = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: file <path to audio file>");
        std::process::exit(1);
    });
    let file = file.as_str();
    // Get file relative to manifest directory
    let file = std::path::Path::new(file).to_str().unwrap();

    streaming_recognize(client, file, "en").await?;

    Ok(())
}
