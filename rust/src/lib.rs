pub mod stt_service {
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

#[derive(Clone)]
pub struct Auth {
    pub token: String,
    pub secret: String,
}

impl Auth {
    pub fn new(token: &str, secret: &str) -> Self {
        Self {
            token: token.to_string(),
            secret: secret.to_string(),
        }
    }
}

pub struct AuthInterceptor {
    auth: Option<Auth>,
}

impl AuthInterceptor {
    fn new(auth: Option<Auth>) -> Self {
        Self { auth }
    }
}
impl Interceptor for AuthInterceptor {
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

pub type SttClient = SttServiceClient<InterceptedService<Channel, AuthInterceptor>>;

#[derive(Clone)]
pub struct TlsOptions {
    pub auth: Option<Auth>,
    pub ca_certificate: Option<String>,
}

impl TlsOptions {
    pub fn new(auth: Option<Auth>, ca_certificate: Option<String>) -> Self {
        Self {
            auth,
            ca_certificate,
        }
    }
}

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
                    // .domain_name("localhost".to_string())
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

pub async fn get_models(
    mut client: SttClient,
    request: Option<ModelsRequest>,
) -> Result<ModelsResponse, Box<dyn Error>> {
    let req = request.unwrap_or(ModelsRequest::default());
    let request = tonic::Request::new(req);
    let response = client.models(request).await?;
    Ok(response.get_ref().to_owned())
}

pub async fn get_locales(
    mut client: SttClient,
    request: Option<LocalesRequest>,
) -> Result<LocalesResponse, Box<dyn Error>> {
    let req = request.unwrap_or(LocalesRequest::default());
    let request = tonic::Request::new(req);
    #[allow(deprecated)]
    let response = client.locales(request).await?;
    Ok(response.get_ref().to_owned())
}

pub async fn get_account_info(
    mut client: SttClient,
    request: Option<AccountInfoRequest>,
) -> Result<AccountInfoResponse, Box<dyn Error>> {
    let req = request.unwrap_or(AccountInfoRequest::default());
    let request = tonic::Request::new(req);
    let response = client.account_info(request).await?;
    Ok(response.get_ref().to_owned())
}

pub async fn get_nlp_functions(
    mut client: SttClient,
    request: Option<NlpFunctionsRequest>,
) -> Result<NlpFunctionsResponse, Box<dyn Error>> {
    let req = request.unwrap_or(NlpFunctionsRequest::default());
    let request = tonic::Request::new(req);
    let response = client.nlp_functions(request).await?;
    Ok(response.get_ref().to_owned())
}

pub async fn nlp_process(
    mut client: SttClient,
    request: NlpProcessRequest,
) -> Result<NlpProcessResponse, Box<dyn Error>> {
    let request = tonic::Request::new(request);
    let response = client.nlp_process(request).await?;
    Ok(response.get_ref().to_owned())
}

/// Performs speech-to-text recognition on a wav file
pub async fn recognize_file(
    mut client: SttClient,
    file_path: &str,
    spec: Option<RecognitionSpec>,
) -> Result<Vec<StreamingRecognitionResponse>, Box<dyn Error>> {
    let mut responses = Vec::new();
    // Read the file with hound::WavReader
    let wav_reader = hound::WavReader::open(file_path)?;
    let sample_rate_hertz = wav_reader.spec().sample_rate as i64;
    let initial_request = StreamingRecognitionRequest {
        streaming_request: Some(StreamingRequest::Config(RecognitionConfig {
            specification: Some(RecognitionSpec {
                sample_rate_hertz,        // Set sample_rate_hertz from the WAV file
                partial_results: false,   // We don't want partial results for files
                locale: "en".to_string(), // Set locale to English
                ..spec.unwrap_or_default()
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
