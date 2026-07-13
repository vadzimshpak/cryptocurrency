use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::blockchain::core::BlockChain;
use crate::network::types::AppState;

#[derive(Deserialize)]
pub struct SendTransactionRequest {
    pub from: String,
    pub to: String,
    pub signature: String,
    pub amount: u64
}

#[derive(Deserialize)]
pub struct GetBalanceRequest {
    pub address: String,
}

#[derive(Serialize)]
pub struct GetBalanceResponse {
    pub balance: u64,
}

pub async fn send_transaction(
    State(blockchain): State<AppState>,
    Query(params): Query<SendTransactionRequest>,
) -> (StatusCode, Json<BlockChain>) {
    let mut blockchain = blockchain
        .lock()
        .expect("Failed to lock blockchain");

    let sender_balance = blockchain.balance(params.from.clone());

    if sender_balance < params.amount {
        println!("Transaction failed, invalid amount");
        return (StatusCode::BAD_REQUEST, Json(blockchain.clone()))
    }

    let result = blockchain.new_transaction(params.from, params.to, params.signature, params.amount);
    if !result {
        println!("Transaction failed, invalid signature");
        return (StatusCode::BAD_REQUEST, Json(blockchain.clone()))
    }

    (StatusCode::OK, Json(blockchain.clone()))
}

pub async fn get_balance(
    State(blockchain): State<AppState>,
    Query(params): Query<GetBalanceRequest>,
) -> (StatusCode, Json<GetBalanceResponse>) {
    let blockchain = blockchain
        .lock()
        .expect("Failed to lock blockchain");

    let balance = blockchain.balance(params.address);
    (StatusCode::OK, Json(GetBalanceResponse { balance }))
}