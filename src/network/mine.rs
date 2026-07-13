use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::blockchain::core::BlockChain;
use crate::network::types::AppState;

pub async fn mine(State(blockchain): State<AppState>) -> (StatusCode, Json<BlockChain>) {
    let mut blockchain = blockchain
        .lock()
        .expect("Failed to lock blockchain");

    let new_proof = blockchain.proof_of_work(blockchain.last_block().proof);
    let last_hash = blockchain.last_block().hash();

    blockchain.new_block(new_proof, last_hash);

    (StatusCode::OK, Json(blockchain.clone()))
}