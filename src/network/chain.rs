
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::blockchain::core::BlockChain;
use crate::network::types::AppState;

pub async fn get_chain(State(blockchain): State<AppState>) -> (StatusCode, Json<BlockChain>){
    let blockchain = blockchain
        .lock()
        .expect("Failed to lock blockchain");

    (StatusCode::OK, Json(blockchain.clone()))
}