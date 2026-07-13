use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;

use serde::Deserialize;

use crate::blockchain::core::BlockChain;
use crate::network::types::AppState;

#[derive(Deserialize)]
pub struct FaucetRequest {
    pub address: String
}

pub async fn faucet(
    State(blockchain): State<AppState>,
    Query(params): Query<FaucetRequest>,
) -> (StatusCode, Json<BlockChain>) {
    let mut blockchain = blockchain
        .lock()
        .expect("Failed to lock blockchain");

    blockchain.inner_transaction("core0000".to_string(), params.address, 100);


    (StatusCode::OK, Json(blockchain.clone()))
}