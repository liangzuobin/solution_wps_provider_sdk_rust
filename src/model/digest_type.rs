use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Clone, Serialize, Deserialize, Display)]
pub enum DigestType {
    MD5,
    SHA1,
}
