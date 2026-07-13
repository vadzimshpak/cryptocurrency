use std::sync::{Arc, Mutex};

use crate::blockchain::core::BlockChain;

pub type AppState = Arc<Mutex<BlockChain>>;