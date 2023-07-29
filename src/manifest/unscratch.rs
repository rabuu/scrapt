use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    project: Project,
    assets: Assets,
    meta: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    extensions: Vec<Extension>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    directory: String,
    auto_renaming: bool,
}
