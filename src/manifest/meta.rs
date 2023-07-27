use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub semver: Version,
    pub vm: Version,
    pub agent: String,
}
