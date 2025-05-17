use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::types::{Block, TransactionReceipt},
};
use clap::Parser;
use dotenv::dotenv;
use op_alloy_network::Optimism;
use reqwest::ClientBuilder;
use serde_json;
use std::env;
use tracing::{error, info, warn};
use tracing_subscriber;
use url::Url;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let rpc_url = &env::var("BASE_RPC").unwrap();
    info!("Parsed RPC URL. Using: {}", rpc_url);
    let provider = ProviderBuilder::new()
        .network::<Optimism>()
        .connect("https://base-rpc.publicnode.com")
        .await?;

    let transaction_hash: String =
        "0x1ae319ba1a236dffe07bcaa323948c9268225f4050ab7eaf86ab5930a937f162".to_string();
    // let untrusted_block_number: u64 = 15828242; // Unichain block number
    let untrusted_block_number: u64 = 30217764; // Base block number

    let request = client.request(
        "eth_getBlockByNumber",
        (format!("0x{:x}", untrusted_block_number), false),
    );
    let untrusted_block: Block = request
        .await
        .expect("Failed to get block by number during RPC call with error.");
    let untrusted_transaction_root = untrusted_block.header.inner.transactions_root;
    info!(
        "Untrusted transaction root: {:?}",
        untrusted_transaction_root
    );
    // let untrusted_block: Block = request.await.expect("Failed to get block by number during RPC call with error.");
    // let raw_response: serde_json::Value = request.await.expect("Failed to get block by number during RPC call with error.");

    // info!("Raw response: {:?}", raw_response);
    // apparently alloy needs us to pass `true` to get all transactions and calculate the transaction root
    // let request = client.request("eth_getBlockByNumber", (format!("0x{:x}", untrusted_block_number), true));

    // info!("Block object: {:?}", untrusted_block);

    // info!("Transaction root: {:?}", untrusted_block.header.inner.transactions_root);

    // TODO: calcualte the transaction root, I guess I can't get the root from the RPC call..
    // let untrusted_transaction_root: String = untrusted_block

    // let request = client.request("eth_getBlockByNumber", (untrusted_block_number, true));
    // let untrusted_block = request.await.unwrap();
}
