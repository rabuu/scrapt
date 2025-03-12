use serde::{Deserialize, Serialize};

use crate::extension::Extension;
use crate::metadata::Metadata;
use crate::monitor::Monitor;
use crate::target::Target;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub targets: Vec<Target>,
    pub monitors: Vec<Monitor>,
    pub extensions: Vec<Extension>,
    pub meta: Metadata,
}

impl Project {
    pub fn builder(stage: Target) -> builder::ProjectBuilder {
        builder::ProjectBuilder::new(stage)
    }

    // TODO: error
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
        targets: Vec<Target>,
        monitors: Vec<Monitor>,
        extensions: Vec<Extension>,
        meta: Option<Metadata>,
    }

    impl ProjectBuilder {
        pub fn new(stage: Target) -> ProjectBuilder {
            debug_assert!(stage.is_stage);

            ProjectBuilder {
                targets: vec![stage],
                monitors: Vec::new(),
                extensions: Vec::new(),
                meta: None,
            }
        }

        pub fn add_sprite(mut self, sprite: Target) -> ProjectBuilder {
            debug_assert!(!sprite.is_stage);

            self.targets.push(sprite);
            self
        }

        pub fn add_monitor(mut self, monitor: Monitor) -> ProjectBuilder {
            self.monitors.push(monitor);
            self
        }

        pub fn add_extension(mut self, extension: Extension) -> ProjectBuilder {
            self.extensions.push(extension);
            self
        }

        pub fn metadata(mut self, metadata: Metadata) -> ProjectBuilder {
            self.meta = Some(metadata);
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
