mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_stt_client::{
    get_client, nlp_process,
    stt_service::{NlpFunctionSpec, NlpProcessRequest, NlpSpec},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let client = get_client(host, tls_options).await?;

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
