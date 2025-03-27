use std::error::Error;

use aristech_stt_client::{
    nlp_process,
    stt_service::{NlpFunctionSpec, NlpProcessRequest, NlpSpec},
    SttClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let mut client = SttClientBuilder::new().build().await?;

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

    let function_infos = nlp_process(&mut client, request).await?;
    println!("{:#?}", function_infos);

    Ok(())
}
