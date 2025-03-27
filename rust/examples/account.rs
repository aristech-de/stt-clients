use std::error::Error;

use aristech_stt_client::{get_account_info, SttClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv()?;

    let mut client = SttClientBuilder::new().build().await?;

    let info = get_account_info(&mut client, None).await?;
    println!("{:#?}", info);

    Ok(())
}
