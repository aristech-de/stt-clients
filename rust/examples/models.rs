use std::error::Error;

use aristech_stt_client::{get_models, SttClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let mut client = SttClientBuilder::new().build().await?;

    let response = get_models(&mut client, None).await?;
    for model in response.model {
        println!("{:#?}", model);
    }
    Ok(())
}
