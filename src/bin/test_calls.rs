use alloy::{
    eips::eip1898::BlockNumberOrTag,
    primitives::{B256, aliases},
    providers::{Provider, ProviderBuilder},
    rpc::types::{Block, TransactionReceipt},
};
use clap::Parser;
use dotenv::dotenv;
use op_alloy_network::Optimism;
use reqwest::ClientBuilder;
use std::{env, str::FromStr};
use tracing::{error, info, warn};
use tracing_subscriber;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let rpc_url = &env::var("UNICHAIN_RPC").unwrap();
    info!("Parsed RPC URL. Using: {}", rpc_url);

    // Workaround for a bug; provided at https://t.me/ethers_rs/44751
    let op_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .fetch_chain_id()
        .with_gas_estimation()
        .with_cached_nonce_management()
        .network::<Optimism>()
        .connect(rpc_url)
        .await?;

    let transaction_hash: B256 =
        B256::from_str("0x1ae319ba1a236dffe07bcaa323948c9268225f4050ab7eaf86ab5930a937f162")
            .unwrap();
    let untrusted_transaction_receipt = op_provider
        .get_transaction_by_hash(transaction_hash)
        .await
        .expect("Failed to get transaction receipt.");
    info!(
        "untrusted tx receipt is {:?}",
        untrusted_transaction_receipt
    );

    // let untrusted_block_number: BlockNumberOrTag = BlockNumberOrTag::Number(15828242); // Unichain block number

    // let untrusted_block = op_provider
    //     .get_block_by_number(untrusted_block_number)
    //     .full()
    //     .await
    //     .expect("Failed to get block by number during RPC call with error.");

    // info!("untrusted block is {:?}", untrusted_block);

    Ok(())
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
