use scratch_common_types::Extension;
use serde::{Deserialize, Serialize};

pub use meta::Metadata;
pub use target::{Asset, Target};

mod block;
mod common;
mod meta;
mod monitor;
mod target;

pub type Version = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub targets: Vec<target::Target>,
    pub monitors: Vec<monitor::Monitor>,
    pub extensions: Vec<Extension>,
    pub meta: meta::Metadata,
}

impl Project {
    pub fn builder(stage: target::Target) -> builder::ProjectBuilder {
        builder::ProjectBuilder::new(stage)
    }

    pub fn parse(input: &str) -> Result<Project, String> {
        match serde_json::from_str(input) {
            Ok(m) => Ok(m),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub mod builder {
    use super::*;

    pub struct ProjectBuilder {
        targets: Vec<target::Target>,
        monitors: Vec<monitor::Monitor>,
        extensions: Vec<Extension>,
        meta: Option<meta::Metadata>,
    }

    impl ProjectBuilder {
        pub fn new(stage: target::Target) -> ProjectBuilder {
            debug_assert!(stage.is_stage);

            ProjectBuilder {
                targets: vec![stage],
                monitors: Vec::new(),
                extensions: Vec::new(),
                meta: None,
            }
        }

        pub fn add_sprite(mut self, sprite: target::Target) -> ProjectBuilder {
            debug_assert!(!sprite.is_stage);

            self.targets.push(sprite);
            self
        }

        pub fn add_monitor(mut self, monitor: monitor::Monitor) -> ProjectBuilder {
            self.monitors.push(monitor);
            self
        }

        pub fn add_extension(mut self, extension: Extension) -> ProjectBuilder {
            self.extensions.push(extension);
            self
        }

        pub fn meta(mut self, meta: meta::Metadata) -> ProjectBuilder {
            self.meta = Some(meta);
            self
        }

        pub fn build(self) -> Project {
            Project {
                targets: self.targets,
                monitors: self.monitors,
                extensions: self.extensions,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
}
