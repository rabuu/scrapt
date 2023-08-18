use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default = "default_semver")]
    pub semver: Version,
    #[serde(default = "default_vm")]
    pub vm: Version,
    #[serde(default = "default_agent")]
    pub agent: String,
}

impl Metadata {
    pub fn builder() -> builder::MetadataBuilder {
        builder::MetadataBuilder::new()
    }
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

pub mod builder {
    use super::*;
    use crate::Version;

    pub struct MetadataBuilder {
        semver: Option<Version>,
        vm: Option<Version>,
        agent: Option<String>,
    }

    impl MetadataBuilder {
        pub fn new() -> MetadataBuilder {
            MetadataBuilder {
                semver: None,
                vm: None,
                agent: None,
            }
        }

        pub fn semver(mut self, semver: Version) -> MetadataBuilder {
            self.semver = Some(semver);
            self
        }

        pub fn vm(mut self, vm: Version) -> MetadataBuilder {
            self.vm = Some(vm);
            self
        }

        pub fn agent(mut self, agent: String) -> MetadataBuilder {
            self.agent = Some(agent);
            self
        }

        pub fn build(self) -> Metadata {
            Metadata {
                semver: self.semver.unwrap_or(default_semver()),
                vm: self.vm.unwrap_or(default_vm()),
                agent: self.agent.unwrap_or(default_agent()),
            }
        }
    }
}
