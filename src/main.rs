pub mod blockchain;
pub mod network;

use std::sync::{Arc, Mutex};
use blockchain::core::BlockChain;

use axum::{
    routing::{get},
    Json,
    Router
};

use crate::network::chain::get_chain;
use crate::network::faucet::faucet;
use crate::network::mine::mine;
use crate::network::transaction::{get_balance, send_transaction};

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(BlockChain::new()));

    let app: Router<()> = Router::new()
        .route("/", get(|| async { Json("Hello, world!") }))
        .route("/chain", get(get_chain))
        .route("/transactions/send", get(send_transaction))
        .route("/transactions/balance", get(get_balance))
        .route("/faucet", get(faucet))
        .route("/mine", get(mine))
        .with_state(blockchain);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
