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
    streaming_recognition_request, AccountInfoRequest, AccountInfoResponse, ModelsRequest,
    ModelsResponse, NlpFunctionsRequest, NlpFunctionsResponse, NlpProcessRequest,
    NlpProcessResponse, RecognitionConfig, RecognitionSpec, StreamingRecognitionRequest,
    StreamingRecognitionResponse,
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

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

struct ApiKeyData {
    token: String,
    secret: String,
    host: Option<String>,
}

/// Decodes the given api key into an Auth struct.
fn decode_api_key(api_key: &str) -> Result<ApiKeyData, Box<dyn Error>> {
    // The API key is base64 url encoded and has no padding and starts with "at-"
    let api_key = api_key.trim_start_matches("at-");
    let key_data = URL_SAFE_NO_PAD.decode(api_key)?;
    let key_data = String::from_utf8(key_data)?;

    let mut token = None;
    let mut secret = None;
    let mut host = None;
    for line in key_data.lines() {
        let mut parts = line.splitn(2, ":");
        let key = match parts.next() {
            Some(key) => key.trim(),
            None => continue,
        };
        let value = match parts.next() {
            Some(value) => value.trim(),
            None => continue,
        };
        match key {
            "token" => token = Some(value.to_string()),
            "secret" => secret = Some(value.to_string()),
            "host" => {
                // If the host doesn't start with http:// or https://, add https://
                let key_host = value.to_string();
                host = match key_host.starts_with("http://") || key_host.starts_with("https://") {
                    true => Some(
                        key_host
                            .trim_end_matches('/')
                            .trim_end_matches('/')
                            .to_string(),
                    ),
                    false => Some(format!(
                        "https://{}",
                        key_host.trim_end_matches('/').trim_end_matches('/')
                    )),
                };
            }
            _ => {}
        }
    }
    match (token, secret) {
        (Some(token), Some(secret)) => Ok(ApiKeyData {
            token,
            secret,
            host,
        }),
        _ => Err("API key is missing token or secret".into()),
    }
}

/// The SttClientBuilder struct is used to build a SttClient with the given host and tls options.
#[derive(Default)]
pub struct SttClientBuilder {
    host: String,
    tls: bool,
    auth: Option<Auth>,
    ca_certificate: Option<String>,
}

impl SttClientBuilder {
    /// Creates a new SttClientBuilder and tries to parse the API key from the environment variable `ARISTECH_STT_API_KEY`.
    /// If no API key is found or the API key is invalid, the builder will be created without authentication data.
    /// Tls will be enabled if a valid API key was found or false otherwise.
    /// To catch any errors with the API key, use the `.api_key` method.
    ///
    /// If a valid API key was found, a custom root certificate can be set with the environment variable `ARISTECH_STT_CA_CERTIFICATE` as well if the server uses a self-signed certificate for example.
    /// The certificate should be the path to the certificate in PEM format. It is also possible to set the certificate with the `.ca_certificate` method.
    ///
    /// To create a client without automatically checking the environment variable, use the default constructor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aristech_stt_client::{SttClientBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SttClientBuilder::new()
    ///       .build()
    ///       .await
    ///       .unwrap();
    ///     // Use the client
    ///     // ...
    /// }
    pub fn new() -> Self {
        // Try parsing the API key from the environment `ARISTECH_STT_API_KEY`
        if let Ok(api_key) = std::env::var("ARISTECH_STT_API_KEY") {
            if let Ok(api_key_data) = decode_api_key(&api_key) {
                let ca_certificate = match std::env::var("ARISTECH_STT_CA_CERTIFICATE") {
                    Ok(ca_certificate) if !ca_certificate.is_empty() => {
                        // Try to read the certificate from the file
                        match std::fs::read_to_string(ca_certificate) {
                            Ok(ca_certificate) => Some(ca_certificate),
                            Err(_) => None,
                        }
                    }
                    _ => None,
                };
                let host = api_key_data.host.unwrap_or_default();
                return Self {
                    tls: true,
                    host,
                    auth: Some(Auth::new(&api_key_data.token, &api_key_data.secret)),
                    ca_certificate,
                };
            }
        }
        Self {
            tls: false,
            ..Default::default()
        }
    }

    /// Attempts to parse the given API key and set the authentication data for the SttClientBuilder.
    /// When using the `new` method, the builder will automatically try to parse the API key from the environment variable `ARISTECH_STT_API_KEY` but won't fail if the API key is invalid or missing.
    /// You can use this method to manually set the API key and catch any errors.
    /// Note that the host from the API key will only be used if no host was set before.
    ///
    /// # Arguments
    /// * `api_key` - The API key to use for the connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aristech_stt_client::{SttClientBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Use the default constructor to create a new SttClientBuilder without
    ///     // automatically attempting to parse the environment variable `ARISTECH_STT_API_KEY`.
    ///     let client = SttClientBuilder::default()
    ///       .api_key("at-abc123...").unwrap()
    ///       .build()
    ///       .await
    ///       .unwrap();
    ///     // Use the client
    ///     // ...
    /// }
    /// ```
    pub fn api_key(mut self, api_key: &str) -> Result<Self, Box<dyn Error>> {
        let api_key_data = decode_api_key(api_key)?;
        if let Some(host) = api_key_data.host {
            if self.host.is_empty() {
                self.host = host;
            }
        }
        self.tls = true;
        self.auth = Some(Auth::new(&api_key_data.token, &api_key_data.secret));
        Ok(self)
    }

    /// Allows to set a custom root certificate to use for the connection and enables tls when a certificate is set to Some.
    /// This is especially useful when the server uses a self-signed certificate.
    /// The `ca_certificate` should be the content of the certificate in PEM format.
    ///
    /// # Arguments
    /// * `ca_certificate` - The root certificate to use for the connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aristech_stt_client::{SttClientBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SttClientBuilder::new()
    ///       .ca_certificate(Some(std::fs::read_to_string("path/to/certificate.pem").unwrap()))
    ///       .build()
    ///       .await
    ///       .unwrap();
    ///      // Use the client
    ///      // ...
    /// }
    /// ```
    pub fn ca_certificate(mut self, ca_certificate: Option<String>) -> Self {
        // If a ca_certificate is set, we need to use tls
        match ca_certificate {
            Some(_) => self.tls = true,
            _ => {}
        }
        self.ca_certificate = ca_certificate;
        self
    }

    /// Sets the auth options for the SttClientBuilder manually and enables tls when auth is set to Some.  
    /// **Note:** Calling `.api_key` after `.auth` will overwrite the auth data.
    ///
    /// # Arguments
    /// * `auth` - The authentication data to use for the connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aristech_stt_client::{SttClientBuilder, Auth};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SttClientBuilder::default()
    ///       .host("https://stt.example.com:9424").unwrap()
    ///       .auth(Some(Auth { token: "my-token".to_string(). secret: "my-secret".to_string() }))
    ///       .build()
    ///       .await
    ///       .unwrap();
    ///       // Use the client
    ///       // ...
    /// }
    pub fn auth(mut self, auth: Option<Auth>) -> Self {
        // If auth is set, we need to use tls
        match auth {
            Some(_) => self.tls = true,
            _ => {}
        }
        self.auth = auth;
        self
    }

    /// Sets the host for the SttClientBuilder manually and enables tls depending on the protocol of the host.  
    /// **Note:** When the API key in the environment variable ARISTECH_STT_API_KEY contains a host or when you call `.api_key` before this call, this will automatically be set to the host from the API key but you can still overwrite it with this call.
    ///
    /// # Arguments
    /// * `host` - The host to connect to (might include the port number e.g. "https://stt.example.com:9424"). Note that the protocol must be included.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aristech_stt_client::{SttClientBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SttClientBuilder::default()
    ///       .host("https://stt.example.com:9423").unwrap()
    ///       .build()
    ///       .await
    ///       .unwrap();
    ///       // Use the client
    ///       // ...
    /// }
    pub fn host(mut self, host: &str) -> Result<Self, Box<dyn Error>> {
        if host.is_empty() {
            return Err("Host cannot be empty".into());
        }
        if !host.starts_with("http://") && host.starts_with("https://") {
            return Err("Host must start with http:// or https://".into());
        }
        self.tls = host.starts_with("https://");
        self.host = host.to_string();
        Ok(self)
    }

    /// Manually enables or disables tls for the SttClientBuilder.  
    /// **Note:** The other methods will overwrite this setting depending on the given values if called after this method.
    ///
    /// # Arguments
    /// * `tls` - Whether to use tls for the connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aristech_stt_client::{SttClientBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SttClientBuilder::default()
    ///       .host("https://stt.example.com:9424").unwrap()
    ///       .tls(false) // <- This doesn't make much sense because the host obviously uses tls but it's just an example
    ///       .build()
    ///       .await
    ///       .unwrap();
    ///
    ///     // Use the client
    ///     // ...
    /// }
    /// ``````
    pub fn tls(mut self, tls: bool) -> Self {
        self.tls = tls;
        self
    }

    /// Atttempts to build the SttClient with the given options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aristech_stt_client::{SttClientBuilder};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///    let client = SttClientBuilder::new()
    ///      .build()
    ///      .await?;
    ///      // Use the client
    ///      // ...
    ///      Ok(())
    /// }
    pub async fn build(self) -> Result<SttClient, Box<dyn Error>> {
        let tls_options = match self.tls {
            true => Some(TlsOptions::new(self.auth, self.ca_certificate)),
            false => None,
        };
        get_client(self.host, tls_options).await
    }
}

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
    // Check if a schema is included in the host
    // otherwise add http if no tls options are given and https otherwise
    let host = if host.starts_with("http://") || host.starts_with("https://") {
        host
    } else {
        match tls_options {
            Some(_) => format!("https://{}", host),
            None => format!("http://{}", host),
        }
    };
    match tls_options {
        Some(tls_options) => {
            let tls = match tls_options.ca_certificate {
                Some(ca_certificate) => {
                    let ca_certificate = Certificate::from_pem(ca_certificate);
                    ClientTlsConfig::new().ca_certificate(ca_certificate)
                }
                None => ClientTlsConfig::with_native_roots(ClientTlsConfig::new()),
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
                sample_rate_hertz,      // Set sample_rate_hertz from the WAV file
                partial_results: false, // We don't want partial results for files
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
