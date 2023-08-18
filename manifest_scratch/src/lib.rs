use serde::{Deserialize, Serialize};

use manifest_common::*;

mod common;
mod monitor;
mod target;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub targets: Vec<target::Target>,
    pub monitors: Vec<monitor::Monitor>,
    pub extensions: Vec<Extension>,
    pub meta: Metadata,
}

impl Manifest {
    pub fn parse(input: &str) -> Option<Manifest> {
        serde_json::from_str(input).ok()?
    }
}
