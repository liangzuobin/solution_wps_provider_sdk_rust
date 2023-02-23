use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Builder, Serialize, Deserialize)]
pub struct File {
    id: String,
    name: String,
    version: u32,
    size: u64,
    create_time: u64,
    modify_time: u64,
    creator_id: String,
    modifier_id: String,
}

impl File {}
