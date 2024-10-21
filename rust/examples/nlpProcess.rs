use std::error::Error;

use aristech_stt_client::{
    get_client, nlp_process,
    stt_service::{NlpFunctionSpec, NlpProcessRequest, NlpSpec},
    Auth, TlsOptions,
};

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
    .await?;

    let server_config = std::env::var("NLP_SERVER_CONFIG").unwrap_or("default".to_string());
    let functions = std::env::var("NLP_PIPELINE").unwrap_or("spellcheck-de".to_string());
    // Split the functions by comma
    let functions = functions
        .split(',')
        .map(|s| NlpFunctionSpec {
            id: s.to_string(),
            ..NlpFunctionSpec::default()
        })
        .collect::<Vec<NlpFunctionSpec>>();

    let request = NlpProcessRequest {
        text: "thanks for choosing aristech".to_string(),
        nlp: Some(NlpSpec {
            server_config,
            functions,
            ..NlpSpec::default()
        }),
    };

    let function_infos = nlp_process(client, request).await?;
    println!("{:#?}", function_infos);

    Ok(())
}
