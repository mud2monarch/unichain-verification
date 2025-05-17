use alloy::{
    consensus::transaction, eips::eip1898::BlockNumberOrTag,
    primitives::{aliases, B256, utils::keccak256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{Block, TransactionReceipt}
};
use clap::Parser;
use dotenv::dotenv;
use op_alloy_network::Optimism;
use reqwest::ClientBuilder;
use std::{env, str::FromStr};
use tracing::{error, info, warn};
use tracing_subscriber;
use url::Url;
use alloy_rlp::encode;
use hex;

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
    let untrusted_alloy_transaction = op_provider
        .get_transaction_by_hash(transaction_hash)
        .await
        .expect("Failed to get transaction receipt in RPC call.")
        .expect(&format!("Transaction receipt is None for transaction hash {}. Can't continue without a valid transaction.", transaction_hash));

    info!("untrusted tx is {:?}", untrusted_alloy_transaction);

    // TODO: This code is giving a hash mismatch :(
    // let (untrusted_transaction, _) = untrusted_alloy_transaction.inner.inner.into_parts();

    // info!("tx is {:?}", untrusted_transaction);

    // let untrusted_raw_rlp_bytes = encode(&untrusted_transaction);
    // info!("untrusted rlp is {:?}", untrusted_raw_rlp_bytes);
    // info!("hex-printed rlp is 0x{}", hex::encode(untrusted_raw_rlp_bytes.clone())); 

    // let untrusted_hash = keccak256(untrusted_raw_rlp_bytes.clone());
    // info!("untrusted hash is 0x{:x}", untrusted_hash);

    //     // 1. Allocate a new buffer and prepend the type byte
    // let mut tx_bytes = Vec::with_capacity(1 + untrusted_raw_rlp_bytes.len());
    // tx_bytes.push(0x02);                                // type-byte for EIP-1559
    // tx_bytes.extend_from_slice(&untrusted_raw_rlp_bytes);

    // // 2. Hash it
    // let derived = keccak256(&tx_bytes);
    // info!("derived hash = 0x{:x}", derived);

    // // 3. (optional) compare with RPC / wallet hash
    // assert_eq!(derived, transaction_hash, "hash mismatch!");

    Ok(())
}
