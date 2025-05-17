use dotenv::dotenv;
use std::env;

use reqwest::Client;
use std::env;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error> {
    // Load .env file
    dotenv().ok();

    // Get RPC URL from environment
    let rpc_url = env::var("UNICHAIN_RPC").expect("UNICHAIN_RPC must be set in .env file");

    // Validate the URL
    let _url = Url::parse(&rpc_url).expect("UNICHAIN_RPC must be a valid URL");

    // Create a reqwest client
    let client = Client::builder().build()?;

    Ok(client)
}
