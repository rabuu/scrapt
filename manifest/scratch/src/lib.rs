use serde::{Deserialize, Serialize};

pub use manifest_common::{Extension, Metadata};
pub use target::{Asset, Target};

mod block;
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
    pub fn builder(stage: target::Target) -> builder::ManifestBuilder {
        builder::ManifestBuilder::new(stage)
    }

    pub fn parse(input: &str) -> Result<Manifest, String> {
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

    pub struct ManifestBuilder {
        targets: Vec<target::Target>,
        monitors: Vec<monitor::Monitor>,
        extensions: Vec<Extension>,
        meta: Option<Metadata>,
    }

    impl ManifestBuilder {
        pub fn new(stage: target::Target) -> ManifestBuilder {
            debug_assert!(stage.is_stage);

            ManifestBuilder {
                targets: vec![stage],
                monitors: Vec::new(),
                extensions: Vec::new(),
                meta: None,
            }
        }

        pub fn add_sprite(mut self, sprite: target::Target) -> ManifestBuilder {
            debug_assert!(!sprite.is_stage);

            self.targets.push(sprite);
            self
        }

        pub fn add_monitor(mut self, monitor: monitor::Monitor) -> ManifestBuilder {
            self.monitors.push(monitor);
            self
        }

        pub fn add_extension(mut self, extension: Extension) -> ManifestBuilder {
            self.extensions.push(extension);
            self
        }

        pub fn meta(mut self, meta: Metadata) -> ManifestBuilder {
            self.meta = Some(meta);
            self
        }

        pub fn build(self) -> Manifest {
            Manifest {
                targets: self.targets,
                monitors: self.monitors,
                extensions: self.extensions,
                meta: self.meta.unwrap_or_default(),
            }
        }
    }
}
