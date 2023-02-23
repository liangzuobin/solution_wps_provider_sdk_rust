use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize)]
pub struct Permission {
    usr_id: String,
    read: bool,
    update: bool,
    download: bool,
    rename: bool,
    history: bool,
    copy: bool,
    print: bool,
    save_as: bool,
    comment: bool,
}
