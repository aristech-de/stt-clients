use std::error::Error;

use aristech_stt_client::{get_nlp_functions, SttClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let mut client = SttClientBuilder::new().build().await?;

    let function_infos = get_nlp_functions(&mut client, None).await?;
    println!("{:#?}", function_infos);

    Ok(())
}
