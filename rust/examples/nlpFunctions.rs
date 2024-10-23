mod utils;
use std::error::Error;
use utils::get_tls_options;

use aristech_stt_client::{get_client, get_nlp_functions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let host = std::env::var("HOST")?;
    let tls_options = get_tls_options()?;
    let mut client = get_client(host, tls_options).await?;

    let function_infos = get_nlp_functions(&mut client, None).await?;
    println!("{:#?}", function_infos);

    Ok(())
}
