mod utils;
use utils::get_tls_options;

use aristech_stt_client::{
    get_client,
    stt_service::{
        streaming_recognition_request, RecognitionConfig, RecognitionSpec,
        StreamingRecognitionRequest,
    },
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::codegen::CompressionEncoding;

fn f32_to_pcm16(data: &[f32]) -> Vec<u8> {
    let mut pcm16 = Vec::with_capacity(data.len() * 2);
    for sample in data {
        let sample = (sample * std::i16::MAX as f32) as i16;
        pcm16.extend_from_slice(&sample.to_le_bytes());
    }
    pcm16
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Clear the terminal
    print!("{}[2J", 27 as char);
    // Move the cursor to the top left
    print!("{}[1;1H", 27 as char);

    // We simply use the default input device provided by the OS
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no input device available");

    // Sending audio with a sample rate higher than 16_000 will not improve the recognition accuracy
    // but will increase the latency because most models are at most trained with 16_000 Hz.
    // The default sample rate could be 44_100 however. Therefore it would be better to request a
    // sample rate of 16_000. However on MacOS cpal sometimes panics when not using the native sample rate of the device.
    /*
    let stream_config = cpal::SupportedStreamConfig::new(
        1,
        cpal::SampleRate(16_000),
        cpal::SupportedBufferSize::Range {
            min: 512,
            max: 2048,
        },
        cpal::SampleFormat::F32,
    );
    let sample_rate = stream_config.sample_rate().0 as i64;
    let config = stream_config.config();
     */
    let stream_config = device
        .default_input_config()
        .expect("Could not get default input config");
    let sample_rate = stream_config.sample_rate().0 as i64;
    let config = stream_config.into();

    println!(
        "Using input device \"{}\" @ {} Hz",
        device.name().unwrap(),
        sample_rate
    );

    // We create a channel which can be used to create a input_stream of SpeechRecognitionRequests
    let (tx, rx) = mpsc::channel::<StreamingRecognitionRequest>(2000);
    let input_stream = ReceiverStream::new(rx);
    let initial_request = StreamingRecognitionRequest {
        streaming_request: Some(streaming_recognition_request::StreamingRequest::Config(
            RecognitionConfig {
                specification: Some(RecognitionSpec {
                    audio_encoding: 0,
                    sample_rate_hertz: sample_rate,
                    locale: "en".to_string(),
                    graph: "".to_string(),
                    grammar: "".to_string(),
                    partial_results: true,
                    single_utterance: false,
                    normalization: None,
                    phones: false,
                    model: std::env::var("MODEL").unwrap_or_else(|_| "".to_string()),
                    endpointing: None,
                    vad: None,
                    prompt: "".to_string(),
                }),
            },
        )),
    };
    // The initial request tells the server what kind of audio we are sending
    tx.send(initial_request).await?;

    let stream = device
        .build_input_stream(
            &config,
            move |data: &[f32], _| {
                // Create a new audio request and send it to the server
                let audio_request = StreamingRecognitionRequest {
                    streaming_request: Some(
                        streaming_recognition_request::StreamingRequest::AudioContent(
                            f32_to_pcm16(data),
                        ),
                    ),
                };
                tx.blocking_send(audio_request).unwrap();
            },
            |_| {
                panic!("An error occured");
            },
            None,
        )
        .unwrap();
    // Start the stream
    stream.play().unwrap();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options)
        .await?
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);

    // With the input_stream we are finally able to start the streaming recognition
    let mut response_stream = client.streaming_recognize(input_stream).await?.into_inner();

    // We keep track of the number of printed lines to be able to rewrite the last line whe partial results are received
    let mut final_lines = 1;

    // Wait for the results
    while let Some(response) = response_stream.message().await? {
        // Clear the last line completely
        print!("{}[2K", 27 as char);
        // Move cursor to the beginning of the line
        print!("{}[{};1H", 27 as char, final_lines + 1);
        // Chunks was originally intended to also store previous final results
        // but for bandwith optimization reasons the server essentially only
        // sends the last partial or final result. It can be assumed that response.chunk.len() will always be 1.
        // Therefore the client has to keep track of the previous final results.
        for chunk in response.chunks {
            // When end of utterance was detected, this result is final and the speech recognition will start
            // from scratch with the next chunk.
            let response_type = match chunk.end_of_utterance {
                true => "Final:  ",
                false => "Partial:",
            };
            // For now, the server only sends one alternative per chunk.
            // This might change in the future.
            for alternative in chunk.alternatives {
                if chunk.end_of_utterance {
                    final_lines += 1;
                }
                println!("{} {}", response_type, alternative.text);
            }
        }
    }
    Ok(())
}
