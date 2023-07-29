use serde::{Deserialize, Serialize};

use super::common::*;

pub mod common;
pub mod monitor;
pub mod target;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub targets: Vec<target::Target>,
    pub monitors: Vec<monitor::Monitor>,
    pub extensions: Vec<Extension>,
    pub meta: Metadata,
}
