use alloy_rpc_client::ClientBuilder;
use clap::Parser;
use dotenv::dotenv;
use std::env;
use url::Url;
use alloy_rpc_types::TransactionReceipt;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    transaction_hash: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let rpc_url =
        Url::parse(&env::var("UNICHAIN_RPC").expect("need to set Unichain RPC in dotenv"))
            .expect("invalid RPC URL");
    
    println!("Parsed URL: {}", rpc_url);

    let client = ClientBuilder::default().http(rpc_url);

    let args = Args::parse();
    let untrusted_transaction_hash = args.transaction_hash;

    let request = client.request("eth_getTransactionReceipt", (untrusted_transaction_hash,));

    let receipt: TransactionReceipt = request.await.unwrap();
    println!("receipt: {:?}", receipt);
}
