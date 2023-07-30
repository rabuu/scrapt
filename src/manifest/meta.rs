use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default = "default_semver")]
    pub semver: Version,
    #[serde(default = "default_vm")]
    pub vm: Version,
    #[serde(default = "default_agent")]
    pub agent: String,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            semver: default_semver(),
            vm: default_vm(),
            agent: default_agent(),
        }
    }
}

fn default_semver() -> Version {
    Version::from("3.0.0")
}

fn default_vm() -> Version {
    Version::from("1.5.91")
}

fn default_agent() -> String {
    String::from("Mozilla/5 (X11; U; Linux x86_64; en-US) Gecko/2010 Firefox/115")
}
