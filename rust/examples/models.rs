mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_stt_client::{get_client, get_models};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let client = get_client(host, tls_options).await?;

    let models = get_models(client, None).await?;
    for model in models.model {
        println!("{:?}", model);
    }
    Ok(())
}
