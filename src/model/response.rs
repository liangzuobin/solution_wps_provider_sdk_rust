use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize)]
pub struct ProviderResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}
