use std::{error::Error, fs::read_to_string};

use aristech_stt_client::{
    get_account_info, get_client, get_locales, get_models,
    stt_service::{
        streaming_recognition_request, GrammarType, ModelType, RecognitionConfig, RecognitionSpec,
        StreamingRecognitionRequest,
    },
    Auth, SttClient, TlsOptions,
};
use chrono::DateTime;
use clap::{command, Parser, Subcommand};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use inquire::Select;
use regex::Regex;

use colored::Colorize;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::codegen::CompressionEncoding;

#[derive(Parser)]
#[command(author, about = "Aristech STT client")]
struct Cli {
    #[arg(long)]
    host: Option<String>,
    #[arg(long)]
    tls: Option<bool>,
    #[arg(short, long)]
    token: Option<String>,
    #[arg(short, long)]
    secret: Option<String>,
    #[arg(short, long)]
    root_cert: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Print the version.", short_flag = 'V', long_flag = "version")]
    Version,
    #[command(about = "List models and nlp functions", long_flag = "list")]
    List,
    #[command(about = "List locales (deprecated)", long_flag = "locales")]
    Locales,
    #[command(about = "Live recognition", short_flag = 'l', long_flag = "live")]
    Live {
        #[arg(long)]
        locale: Option<String>,
        #[arg(short, long)]
        model: Option<String>,
        #[arg(short, long)]
        grammar: Option<String>,
        #[arg(short, long, default_value = "true")]
        partial: Option<bool>,
        #[arg(short, long, default_value = "false")]
        single: Option<bool>,
    },
    #[command(about = "File recognition", short_flag = 'f', long_flag = "file")]
    File {
        #[arg(long)]
        locale: Option<String>,
        #[arg(short, long)]
        model: Option<String>,
        #[arg(short, long)]
        grammar: Option<String>,
        #[arg(short, long, default_value = "false")]
        partial: Option<bool>,
        #[arg(short, long, default_value = "true")]
        single: Option<bool>,
        path: String,
    },
    #[command(about = "Print account infos", long_flag = "account")]
    Account,
}

pub async fn streaming_recognize(
    mut client: SttClient,
    audio_path: &str,
    locale: &str,
    model: &str,
    grammar: &str,
    partial_results: bool,
    single_utterance: bool,
) -> Result<(), Box<dyn Error>> {
    let wav_reader = hound::WavReader::open(audio_path)?;
    let sample_rate_hertz = wav_reader.spec().sample_rate as i64;
    println!("Samplerate: {}", sample_rate_hertz);

    // If grammar starts with <type>:@path (e.g. `jsgf:@/path/to/grammar.gram`), then load the grammar from the file
    let grammar_file_prefixes = vec!["jsgf:@", "kws:@", "json:@"];
    let grammar = if grammar_file_prefixes
        .iter()
        .any(|prefix| grammar.starts_with(prefix))
    {
        // Get grammar type by splitting the string at the first :
        let grammar_type = grammar.split_at(grammar.find(':').unwrap()).0;
        let grammar_path = grammar.split_at(grammar.find('@').unwrap() + 1).1;
        let grammar_content = std::fs::read_to_string(grammar_path)?;
        format!("{}:{}", grammar_type, grammar_content)
    } else {
        // If the grammar simply start with an @, then load the grammar from the file
        if grammar.starts_with('@') {
            let grammar_path = grammar.split_at(grammar.find('@').unwrap() + 1).1;
            std::fs::read_to_string(grammar_path)?
        } else {
            grammar.to_string()
        }
    };

    let config = RecognitionConfig {
        specification: Some(RecognitionSpec {
            audio_encoding: 1,
            sample_rate_hertz,
            locale: locale.to_string(),
            graph: "".to_string(),
            grammar: grammar.to_string(),
            partial_results,
            single_utterance,
            normalization: None,
            phones: false,
            model: model.to_string(),
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
        println!(
            "Client ID: {}, Language: {}",
            response.client_id, response.language
        );
        for chunk in response.chunks {
            let response_type = match chunk.end_of_utterance {
                true => "Final",
                false => "Partial",
            };
            for alternative in chunk.alternatives {
                println!("{}: {}", response_type, alternative.text,);
                if chunk.r#final {
                    println!("Tagged: {}", alternative.tagged_text);
                    println!("Slotted: {}", alternative.slotted_text);
                    println!("NLP: {}", alternative.nlp_text);
                }
            }
        }
    }
    Ok(())
}

async fn select_model(client: SttClient, grammar: Option<&str>) -> Result<String, Box<dyn Error>> {
    let response = get_models(client).await?;
    let mut grammar_type = None;

    if grammar.is_some() {
        let grammar_is_jsgf = grammar
            .map(|grammar| grammar.starts_with("jsgf:"))
            .unwrap_or(false);
        if grammar_is_jsgf {
            grammar_type = Some(GrammarType::Jsgf);
        }
        if grammar_type.is_none() {
            let grammar_is_kws = grammar
                .map(|grammar| grammar.starts_with("kws:"))
                .unwrap_or(false);
            if grammar_is_kws {
                grammar_type = Some(GrammarType::Kws);
            }
            if grammar_type.is_none() {
                grammar_type = Some(GrammarType::PhraseList);
            }
        }
    }

    let mut options = Vec::new();
    response.model.iter().for_each(|model| {
        // For free text recognition, don't show grammar models
        if grammar_type.is_none() && model.r#type() != ModelType::GrammarStt {
            options.push(model.id.clone());
        } else if grammar_type.is_some() {
            let grammar_type = grammar_type.unwrap();
            // Check if the model grammar types contains the grammar type
            for model_grammar_type in model.grammar_type().into_iter() {
                if model_grammar_type == grammar_type {
                    options.push(model.id.clone());
                    break;
                }
            }
        }
    });
    let model = Select::new("Select a model:", options).prompt()?;
    Ok(model)
}

fn f32_to_pcm16(data: &[f32]) -> Vec<u8> {
    let mut pcm16 = Vec::with_capacity(data.len() * 2);
    for sample in data {
        let sample = (sample * std::i16::MAX as f32) as i16;
        pcm16.extend_from_slice(&sample.to_le_bytes());
    }
    pcm16
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let env_ca_certificate = std::env::var("STT_CA_CERTIFICATE").ok();
    let ca_certificate = match cli.root_cert {
        Some(root_cert) => {
            // If root_cert is an empty string return None
            if root_cert.is_empty() {
                None
            } else {
                Some(read_to_string(root_cert).expect("Failed to read root certificate file"))
            }
        }
        None => match env_ca_certificate {
            Some(root_cert) => {
                // If root_cert is an empty string return None
                if root_cert.is_empty() {
                    None
                } else {
                    Some(read_to_string(root_cert).expect("Failed to read root certificate file from environment variable STT_CA_CERTIFICATE"))
                }
            }
            None => None,
        },
    };

    // When the host ends on 9424 it indicates that TLS should be used (Some(true)),
    // when the host ends on 9423 it indicates that TLS should not be used (Some(false)),
    // otherwise we can make no assumptions about TLS and we set host_indicates_tls to None
    let host_indicates_tls = match &cli.host {
        Some(host) => match host.ends_with("9424") {
            true => Some(true),
            false => match host.ends_with("9423") {
                true => Some(false),
                false => None,
            },
        },
        None => None,
    };

    // TLS is true if:
    // - the --tls flag is set to true
    // - a root certificate is provided
    // - no --tls flag is set and the host indicates TLS should be used
    // TODO: use_tls always seems to be true
    let use_tls = cli.tls.unwrap_or(false)
        || (ca_certificate.is_some() && ca_certificate.as_ref().unwrap().len() > 0)
        || (cli.tls.is_none() && host_indicates_tls == Some(true));
    // let use_tls = false;

    // Check if there is an environment variable STT_HOST
    let env_host = std::env::var("STT_HOST").ok();
    let mut host = match cli.host {
        Some(host) => host,
        None => match env_host {
            Some(host) => host,
            None => match use_tls {
                true => "https://localhost:9424".to_string(),
                false => "http://localhost:9423".to_string(),
            },
        },
    };
    // If host does not end on a port (regex `:[0-9]+$` doesn't match) we add the default port based on the TLS setting
    let host_does_not_end_on_port = !Regex::new(r":[0-9]+$").unwrap().is_match(&host);
    if host_does_not_end_on_port {
        host = match use_tls {
            true => format!("{}:9424", host),
            false => format!("{}:9423", host),
        };
    }

    let env_token = std::env::var("STT_TOKEN").ok();
    let env_secret = std::env::var("STT_SECRET").ok();
    let (token, secret) = match (cli.token, cli.secret) {
        (Some(token), Some(secret)) => (Some(token), Some(secret)),
        _ => {
            let env_token_is_empty = env_token.as_ref().map_or(false, |s| s.is_empty());
            let env_secret_is_empty = env_secret.as_ref().map_or(false, |s| s.is_empty());
            if env_token.is_some()
                && env_secret.is_some()
                && !env_token_is_empty
                && !env_secret_is_empty
            {
                (env_token, env_secret)
            } else {
                (None, None)
            }
        }
    };
    let tls_options = match use_tls {
        true => Some(TlsOptions {
            auth: match (token, secret) {
                (Some(token), Some(secret)) => Some(Auth { token, secret }),
                _ => None,
            },
            ca_certificate,
        }),
        false => None,
    };

    println!("Connecting to {}", host);
    println!("TLS: {:?}", use_tls);

    match &cli.command {
        Commands::Version => {
            // Get the version number from Cargo.toml
            let version = env!("CARGO_PKG_VERSION");
            println!("stt {}", version);
        }
        Commands::Account => {
            let client = get_client(host.clone(), tls_options.clone())
                .await?
                .accept_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Gzip);
            let response = get_account_info(client).await?;
            println!("Account token: {}", response.token);
            println!("Display name: {}", response.display_name);
            let blocked_str = if response.blocked {
                "Yes".red().to_string()
            } else {
                "No".green().to_string()
            };
            println!("Blocked: {}", blocked_str);
            println!("Total requests: {}", response.total_requests.to_string());
            println!("Requests in seconds: {}", response.used_seconds.to_string());
            let booked_seconds_str = if response.booked_seconds > 0 {
                response.booked_seconds.to_string().yellow().to_string()
            } else {
                "unlimited".green().to_string()
            };
            println!("Booked seconds: {}", booked_seconds_str);
            let expiry_str = match response.expiration_date > 0 {
                true => {
                    let expiry_date = DateTime::from_timestamp(response.expiration_date, 0)
                        .expect("Failed to parse expiry date");
                    let expiry_str = expiry_date.format("%Y-%m-%d %H:%M:%S").to_string();
                    let expired = expiry_date < chrono::Local::now();
                    if expired {
                        expiry_str.red().to_string()
                    } else {
                        expiry_str.green().to_string()
                    }
                }
                false => "unlimited".green().to_string(),
            };
            println!("Expiry date: {}", expiry_str);
        }
        Commands::List => {
            let client = get_client(host, tls_options)
                .await?
                .accept_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Gzip);
            let response = get_models(client).await?;
            response.model.iter().for_each(|model| {
                let version_str = match model.version.len() {
                    0 => format!("({})", "unknown version".yellow()),
                    _ => format!("({})", model.version.green()),
                };
                println!("{} {} - {}", model.id.blue(), version_str, model.name);
                if model.description.len() > 0 {
                    println!("  Description: {}", model.description);
                }
                let supported_grammars = match model.grammar_type.len() {
                    0 => "No grammar support".yellow().to_string(),
                    _ => {
                        let mut supported_grammars = Vec::with_capacity(model.grammar_type.len());
                        model.grammar_type.iter().for_each(|grammar_type| {
                            match GrammarType::try_from(*grammar_type) {
                                Ok(GrammarType::Jsgf) => supported_grammars.push("JSGF".green()),
                                Ok(GrammarType::Kws) => supported_grammars.push("KWS".green()),
                                Ok(GrammarType::PhraseList) => {
                                    supported_grammars.push("PhraseList".green())
                                }
                                _ => {}
                            };
                        });
                        supported_grammars
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(", ".normal().to_string().as_str())
                    }
                };
                println!("  Supported grammar types: {}", supported_grammars);
                if model.slots.len() > 0 {
                    let slots = model
                        .slots
                        .iter()
                        .map(|slot| slot.clone().red().to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("  Slots: {}", slots);
                }
                if model.examples.len() > 0 {
                    let slots = model
                        .examples
                        .iter()
                        .map(|example| format!("\"{}\"", example.clone()).yellow().to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("  Examples: {}", slots);
                }
                match &model.nlp {
                    Some(nlp) => {
                        let global_args = match nlp.args.is_empty() {
                            false => format!(":{}", nlp.args.clone()),
                            true => "".to_string(),
                        };
                        print!(
                            "  NLP pipeline({}{}): ",
                            nlp.server_config.blue(),
                            global_args
                        );
                        match nlp.functions.len() {
                            0 => {
                                print!("{}", "Empty".yellow().to_string());
                            }
                            _ => {
                                nlp.functions.iter().enumerate().for_each(|(i, function)| {
                                    let args = match function.args.len() {
                                        0 => "".to_string(),
                                        _ => format!(
                                            ":{}",
                                            function
                                                .args
                                                .iter()
                                                .map(|arg| arg.clone())
                                                .collect::<Vec<_>>()
                                                .join(",")
                                                .to_string()
                                        ),
                                    };
                                    if i == 0 {
                                        print!("{}{}", function.id.red().to_string(), args)
                                    } else {
                                        print!(" → {}{}", function.id.red().to_string(), args)
                                    }
                                });
                            }
                        }
                        println!();
                    }
                    None => {}
                }
                let supported_locales = match model.locale.len() {
                    0 => "No locale support".yellow().to_string(),
                    _ => {
                        let mut supported_locales = Vec::with_capacity(model.locale.len());
                        model.locale.iter().for_each(|locale| {
                            supported_locales.push(locale.to_string().green());
                        });
                        supported_locales
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(", ".normal().to_string().as_str())
                    }
                };
                println!("  Supported locales: {}", supported_locales);
            });
        }
        Commands::Locales => {
            let client = get_client(host, tls_options)
                .await?
                .accept_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Gzip);
            let response = get_locales(client).await?;
            let mut locales = Vec::new();
            response.locale.iter().for_each(|locale| {
                locales.push(locale.locale.to_string());
            });
            locales.sort();
            locales.dedup();
            locales.iter().for_each(|locale| {
                println!("{}", locale);
            });
        }
        Commands::Live {
            locale,
            model,
            grammar,
            partial,
            single,
        } => {
            let mut client = get_client(host.clone(), tls_options.clone())
                .await?
                .accept_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Gzip);
            let locale = match locale {
                Some(locale) => locale.to_string(),
                None => "".to_string(),
            };
            let grammar_str = match grammar {
                Some(grammar) => Some(grammar.as_str()),
                None => None,
            };
            let model = match model {
                Some(model) => model.to_string(),
                None => {
                    let client = get_client(host, tls_options)
                        .await?
                        .accept_compressed(CompressionEncoding::Gzip)
                        .send_compressed(CompressionEncoding::Gzip);
                    select_model(client, grammar_str).await?
                }
            };
            let grammar = match grammar {
                Some(grammar) => {
                    if grammar.starts_with('@') {
                        let grammar_path = grammar.split_at(grammar.find('@').unwrap() + 1).1;
                        std::fs::read_to_string(grammar_path)?
                    } else {
                        grammar.to_string()
                    }
                }
                None => "".to_string(),
            };
            let partial = partial.unwrap_or(true);
            let single = single.unwrap_or(false);
            // We simply use the default input device provided by the OS
            let host = cpal::default_host();
            let device = host
                .default_input_device()
                .expect("no input device available");
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
            let (tx, rx) = mpsc::channel::<StreamingRecognitionRequest>(2000);
            let input_stream = ReceiverStream::new(rx);
            let initial_request = StreamingRecognitionRequest {
                streaming_request: Some(streaming_recognition_request::StreamingRequest::Config(
                    RecognitionConfig {
                        specification: Some(RecognitionSpec {
                            audio_encoding: 0,
                            sample_rate_hertz: sample_rate,
                            locale,
                            graph: "".to_string(),
                            grammar,
                            partial_results: partial,
                            single_utterance: single,
                            normalization: None,
                            phones: true,
                            model,
                            endpointing: None,
                            vad: None,
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

            let mut response_stream = client.streaming_recognize(input_stream).await?.into_inner();

            while let Some(response) = response_stream.message().await? {
                // Chunks was originally intended to also store previous final results
                // but for bandwith optimization reasons the server essentially only
                // sends the last partial or final result. It can be assumed that response.chunk.len() will always be 1.
                // Therefore the client has to keep track of the previous final results.
                for chunk in response.chunks {
                    // When end of utterance was detected, this result is final and the speech recognition will start
                    // from scratch with the next chunk.
                    let response_type = match chunk.end_of_utterance {
                        true => "Final:  ",
                        false => "Partial: ",
                    };
                    // For now, the server only sends one alternative per chunk.
                    // This might change in the future.
                    for alternative in chunk.alternatives {
                        let text = match alternative.words.len() {
                            0 => alternative.text.black().to_string(),
                            _ => {
                                let mut text = String::new();
                                for word in alternative.words {
                                    let word_str = if word.confidence >= 0.9 {
                                        word.word.green()
                                    } else if word.confidence >= 0.5 {
                                        word.word.yellow()
                                    } else {
                                        word.word.red()
                                    };
                                    let word_str = word_str.to_string();
                                    text.push_str(word_str.as_str());
                                    text.push(' ');
                                }
                                text
                            }
                        };
                        println!("{} {}", response_type, text);
                        if chunk.end_of_utterance || chunk.r#final {
                            println!("Tagged: {}", alternative.tagged_text);
                            println!("Slotted: {}", alternative.slotted_text);
                            println!("NLP: {}", alternative.nlp_text);
                        }
                    }
                }
            }
        }
        Commands::File {
            locale,
            model,
            grammar,
            partial,
            single,
            path,
        } => {
            let client = get_client(host.clone(), tls_options.clone())
                .await?
                .accept_compressed(CompressionEncoding::Gzip)
                .send_compressed(CompressionEncoding::Gzip);
            let locale = match locale {
                Some(locale) => locale.as_str(),
                None => "",
            };
            let grammar_str = match grammar {
                Some(grammar) => Some(grammar.as_str()),
                None => None,
            };
            let model = match model {
                Some(model) => model.to_string(),
                None => {
                    let client = get_client(host, tls_options)
                        .await?
                        .accept_compressed(CompressionEncoding::Gzip)
                        .send_compressed(CompressionEncoding::Gzip);
                    select_model(client, grammar_str).await?
                }
            };
            let model = model.as_str();
            let grammar = match grammar {
                Some(grammar) => grammar.as_str(),
                None => "",
            };
            let partial = partial.unwrap_or(true);
            let single = single.unwrap_or(false);
            streaming_recognize(
                client,
                path.as_str(),
                locale,
                model,
                grammar,
                partial,
                single,
            )
            .await?;
        }
    };

    Ok(())
}
