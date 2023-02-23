use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    avatra: String,
}