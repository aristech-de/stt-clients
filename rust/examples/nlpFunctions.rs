use std::error::Error;

use aristech_stt_client::{get_client, get_nlp_functions, Auth, TlsOptions};

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

    let function_infos = get_nlp_functions(client, None).await?;
    println!("{:#?}", function_infos);

    Ok(())
}
