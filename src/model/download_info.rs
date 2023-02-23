use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::model::digest_type::DigestType;
use std::collections::HashMap;

#[derive(Builder, Serialize, Deserialize)]
pub struct DownloadInfo {
    url: String,
    headers: HashMap<String, String>,
    digest_type: DigestType,
    digest_value: String,
}

impl DownloadInfo {}
