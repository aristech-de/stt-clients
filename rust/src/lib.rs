//! # Aristech STT-Client
//! The Aristech STT-Client is a client library for the Aristech STT-Server.

#![warn(missing_docs)]

/// The stt_service module contains types and functions generated from the Aristech STT proto file.
pub mod stt_service {
    #![allow(missing_docs)]
    tonic::include_proto!("ari.stt.v1");
}

use std::error::Error;

use stt_service::streaming_recognition_request::StreamingRequest;
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use stt_service::stt_service_client::SttServiceClient;
use stt_service::{
    streaming_recognition_request, AccountInfoRequest, AccountInfoResponse, LocalesRequest,
    LocalesResponse, ModelsRequest, ModelsResponse, NlpFunctionsRequest, NlpFunctionsResponse,
    NlpProcessRequest, NlpProcessResponse, RecognitionConfig, RecognitionSpec,
    StreamingRecognitionRequest, StreamingRecognitionResponse,
};

/// The Auth struct holds the token and secret needed to authenticate with the server.
#[derive(Clone)]
pub struct Auth {
    /// The token to authenticate with the server
    pub token: String,
    /// The secret to authenticate with the server
    pub secret: String,
}

impl Auth {
    /// Creates a new Auth struct with the given token and secret.
    pub fn new(token: &str, secret: &str) -> Self {
        Self {
            token: token.to_string(),
            secret: secret.to_string(),
        }
    }
}

/// The AuthInterceptor struct is used to intercept requests to the server and add the authentication headers.
pub struct AuthInterceptor {
    /// The authentication data to add to the headers
    auth: Option<Auth>,
}

impl AuthInterceptor {
    /// Creates a new AuthInterceptor with the given authentication data.
    fn new(auth: Option<Auth>) -> Self {
        Self { auth }
    }
}
impl Interceptor for AuthInterceptor {
    /// Adds the authentication data to the headers of the request.
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        if let Some(auth) = &self.auth {
            let mut request = request;
            request
                .metadata_mut()
                .insert("token", auth.token.parse().unwrap());
            request
                .metadata_mut()
                .insert("secret", auth.secret.parse().unwrap());
            Ok(request)
        } else {
            Ok(request)
        }
    }
}

/// The SttClient type is a type alias for the SttServiceClient with the AuthInterceptor.
pub type SttClient = SttServiceClient<InterceptedService<Channel, AuthInterceptor>>;

/// The TlsOptions struct holds the tls options needed to communicate with the server.
#[derive(Clone, Default)]
pub struct TlsOptions {
    /// The authentication data to authenticate with the server
    pub auth: Option<Auth>,
    /// The root certificate to verify the server's certificate
    /// This is usually only needed when the server uses a self-signed certificate
    pub ca_certificate: Option<String>,
}

impl TlsOptions {
    /// Creates a new TlsOptions struct with the given authentication data and root certificate.
    pub fn new(auth: Option<Auth>, ca_certificate: Option<String>) -> Self {
        Self {
            auth,
            ca_certificate,
        }
    }
}

/// Creates a new [SttClient] to communicate with the server.
///
/// # Arguments
/// * `host` - The host to connect to (might include the port number e.g. "https://stt.example.com:9424"). Note that the protocol must be included in the host.
/// * `tls_options` - The tls options to use when connecting to the server. If None is given, the connection will be unencrypted and unauthenticated (the server must also be configured to communicate without encryption in this case).
pub async fn get_client(
    host: String,
    tls_options: Option<TlsOptions>,
) -> Result<SttClient, Box<dyn Error>> {
    match tls_options {
        Some(tls_options) => {
            let tls = match tls_options.ca_certificate {
                Some(ca_certificate) => {
                    let ca_certificate = Certificate::from_pem(ca_certificate);
                    ClientTlsConfig::new().ca_certificate(ca_certificate)
                }
                None => ClientTlsConfig::new(),
            };
            let channel = Channel::from_shared(host)?
                .tls_config(tls)?
                .connect()
                .await?;
            let client: SttServiceClient<InterceptedService<Channel, AuthInterceptor>> =
                SttServiceClient::with_interceptor(channel, AuthInterceptor::new(tls_options.auth));
            Ok(client)
        }
        None => {
            let channel = Channel::from_shared(host)?.connect().await?;
            let client: SttServiceClient<InterceptedService<Channel, AuthInterceptor>> =
                SttServiceClient::with_interceptor(channel, AuthInterceptor::new(None));
            Ok(client)
        }
    }
}

/// Gets the list of available models from the server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server. If None is given, the default request will be used.
///
/// # Example
///
/// ```no_run
/// use aristech_stt_client::{get_client, TlsOptions, get_models};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let response = get_models(&mut client, None).await?;
///     for model in response.model {
///         println!("{:?}", model);
///     }
///     Ok(())
/// }
/// ```
pub async fn get_models(
    client: &mut SttClient,
    request: Option<ModelsRequest>,
) -> Result<ModelsResponse, Box<dyn Error>> {
    let req = request.unwrap_or(ModelsRequest::default());
    let request = tonic::Request::new(req);
    let response = client.models(request).await?;
    Ok(response.get_ref().to_owned())
}

/// Gets the list of available locales from the server.
/// This function is deprecated. Please use the get_models instead and check which locales are available for each model.
#[deprecated(
    since = "2.0.0",
    note = "Please use the get_models instead and check which locales are available for each model."
)]
pub async fn get_locales(
    client: &mut SttClient,
    request: Option<LocalesRequest>,
) -> Result<LocalesResponse, Box<dyn Error>> {
    let req = request.unwrap_or(LocalesRequest::default());
    let request = tonic::Request::new(req);
    #[allow(deprecated)]
    let response = client.locales(request).await?;
    Ok(response.get_ref().to_owned())
}

/// Gets the account information from the server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server. If None is given, the default request will be used.
///
/// # Example
///
/// ```no_run
/// use aristech_stt_client::{get_client, TlsOptions, get_account_info};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let response = get_account_info(&mut client, None).await?;
///     println!("{:#?}", response);
///     Ok(())
/// }
/// ```
pub async fn get_account_info(
    client: &mut SttClient,
    request: Option<AccountInfoRequest>,
) -> Result<AccountInfoResponse, Box<dyn Error>> {
    let req = request.unwrap_or(AccountInfoRequest::default());
    let request = tonic::Request::new(req);
    let response = client.account_info(request).await?;
    Ok(response.get_ref().to_owned())
}

/// Gets the list of available NLP functions for each configured NLP-Server.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server. If None is given, the default request will be used.
///
/// # Example
///
/// ```no_run
/// use aristech_stt_client::{get_client, TlsOptions, get_nlp_functions};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let response = get_nlp_functions(&mut client, None).await?;
///     println!("{:#?}", response);
///     Ok(())
/// }
/// ```
pub async fn get_nlp_functions(
    client: &mut SttClient,
    request: Option<NlpFunctionsRequest>,
) -> Result<NlpFunctionsResponse, Box<dyn Error>> {
    let req = request.unwrap_or(NlpFunctionsRequest::default());
    let request = tonic::Request::new(req);
    let response = client.nlp_functions(request).await?;
    Ok(response.get_ref().to_owned())
}

/// Processes the given text with a given NLP pipeline using the STT-Server as proxy.
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `request` - The request to send to the server.
///
/// # Example
///
/// ```no_run
/// use aristech_stt_client::{
///     get_client, TlsOptions, nlp_process,
///     stt_service::{NlpFunctionSpec, NlpProcessRequest, NlpSpec},
/// };
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let response = nlp_process(&mut client, NlpProcessRequest {
///         text: "hello world".to_string(),
///         nlp: Some(NlpSpec {{
///             server_config: "default".to_string(),
///             functions: vec![NlpFunctionSpec {
///                 id: "spellcheck-de".to_string(),
///                 ..NlpFunctionSpec::default()
///             }],
///            ..NlpSpec::default()
///         }}),
///     }).await?;
///     println!("{:#?}", response);
///     Ok(())
/// }
/// ```
pub async fn nlp_process(
    client: &mut SttClient,
    request: NlpProcessRequest,
) -> Result<NlpProcessResponse, Box<dyn Error>> {
    let request = tonic::Request::new(request);
    let response = client.nlp_process(request).await?;
    Ok(response.get_ref().to_owned())
}

/// Performs speech recognition on a wav file
///
/// # Arguments
/// * `client` - The client to use to communicate with the server.
/// * `file_path` - The path to the wav file to recognize.
/// * `config` - The recognition configuration to use. If None is given, the default configuration with locale "en" and the sample rate from the wav file will be used.
///
/// # Example
///
/// ```no_run
/// use aristech_stt_client::{get_client, TlsOptions, recognize_file};
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut client = get_client("https://tts.example.com".to_string(), Some(TlsOptions::default())).await?;
///     let results = recognize_file(&mut client, "my-audio.wav", None).await?;
///     for result in results {
///         println!(
///             "{}",
///             result
///                 .chunks
///                 .get(0)
///                 .unwrap()
///                 .alternatives
///                 .get(0)
///                 .unwrap()
///                 .text
///         );
///     }
///     Ok(())
/// }
/// ```
pub async fn recognize_file(
    client: &mut SttClient,
    file_path: &str,
    config: Option<RecognitionConfig>,
) -> Result<Vec<StreamingRecognitionResponse>, Box<dyn Error>> {
    let mut responses = Vec::new();
    // Read the file with hound::WavReader
    let wav_reader = hound::WavReader::open(file_path)?;
    let sample_rate_hertz = wav_reader.spec().sample_rate as i64;
    let spec = config.unwrap_or_default().specification.unwrap_or_default();
    let initial_request = StreamingRecognitionRequest {
        streaming_request: Some(StreamingRequest::Config(RecognitionConfig {
            specification: Some(RecognitionSpec {
                sample_rate_hertz,        // Set sample_rate_hertz from the WAV file
                partial_results: false,   // We don't want partial results for files
                locale: "en".to_string(), // Set locale to English
                ..spec
            }),
            // At the moment there is nothing besides the specification in the config so we can use the default
            ..RecognitionConfig::default()
        })),
    };

    let audio_content = std::fs::read(file_path)?;
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
        responses.push(response);
    }
    Ok(responses)
}
