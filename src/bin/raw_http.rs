// At one point thought I would need to work around alloy

use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use tracing::{error, info, warn};
use tracing_subscriber;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let rpc_url = env::var("UNICHAIN_RPC").expect("UNICHAIN_RPC must be set in .env file");
    info!("rpc url set to {}.", rpc_url);
    let client = Client::builder().build()?;

    let transaction_hash: String =
        "0x1ae319ba1a236dffe07bcaa323948c9268225f4050ab7eaf86ab5930a937f162".to_string();

    Ok(())
}
