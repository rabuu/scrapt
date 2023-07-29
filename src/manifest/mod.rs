use serde::{Deserialize, Serialize};

pub mod common;
pub mod extension;
pub mod meta;
pub mod monitor;
pub mod target;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub targets: Vec<target::Target>,
    pub monitors: Vec<monitor::Monitor>,
    pub extensions: Vec<extension::Extension>,
    pub meta: meta::Metadata,
}
