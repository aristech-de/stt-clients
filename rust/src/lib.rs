pub mod stt_service {
  tonic::include_proto!("ari.stt.v1");
}

use std::error::Error;

use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

use stt_service::stt_service_client::SttServiceClient;
use stt_service::{AccountInfoResponse, LocalesResponse, ModelsResponse};

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

pub async fn get_models(mut client: SttClient) -> Result<ModelsResponse, Box<dyn Error>> {
  let request = tonic::Request::new(stt_service::ModelsRequest {});
  let response = client.models(request).await?;
  Ok(response.get_ref().to_owned())
}

pub async fn get_locales(mut client: SttClient) -> Result<LocalesResponse, Box<dyn Error>> {
  let request = tonic::Request::new(stt_service::LocalesRequest {});
  #[allow(deprecated)]
  let response = client.locales(request).await?;
  Ok(response.get_ref().to_owned())
}

pub async fn get_account_info(
  mut client: SttClient,
) -> Result<AccountInfoResponse, Box<dyn Error>> {
  let request = tonic::Request::new(stt_service::AccountInfoRequest {});
  let response = client.account_info(request).await?;
  Ok(response.get_ref().to_owned())
}
