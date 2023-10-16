use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use manifest_common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    project: Project,
    #[serde(default)]
    sprites: HashMap<String, String>,
    #[serde(default)]
    assets: Assets,
    #[serde(default)]
    meta: Metadata,
}

impl Manifest {
    pub fn parse(input: &str) -> Result<Manifest, String> {
        match toml::from_str(input) {
            Ok(m) => Ok(m),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    #[serde(default)]
    extensions: Vec<Extension>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Assets {
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
