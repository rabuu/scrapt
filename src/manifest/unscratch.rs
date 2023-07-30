use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    project: Project,
    #[serde(default)]
    assets: Assets,
    #[serde(default)]
    meta: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    #[serde(default)]
    extensions: Vec<Extension>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assets {
    #[serde(default = "default_assets_dir")]
    directory: String,
    #[serde(default = "default_auto_renaming")]
    auto_renaming: bool,
}

impl Default for Assets {
    fn default() -> Self {
        Self {
            directory: default_assets_dir(),
            auto_renaming: default_auto_renaming(),
        }
    }
}

fn default_assets_dir() -> String {
    String::from("assets")
}

fn default_auto_renaming() -> bool {
    true
}
