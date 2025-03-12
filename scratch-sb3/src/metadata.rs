use serde::{Deserialize, Serialize};

pub const DEFAULT_SEMVER: &str = "3.0.0";
pub const DEFAULT_VM: &str = "1.5.91";
pub const DEFAULT_AGENT: &str = "Mozilla/5 (X11; U; Linux x86_64; en-US) Gecko/2010 Firefox/115";

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default = "default_semver")]
    pub semver: String,
    #[serde(default = "default_vm")]
    pub vm: String,
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

fn default_semver() -> String {
    String::from(DEFAULT_SEMVER)
}

fn default_vm() -> String {
    String::from(DEFAULT_VM)
}

fn default_agent() -> String {
    String::from(DEFAULT_AGENT)
}

pub mod builder {
    use super::*;

    #[derive(Default)]
    pub struct MetadataBuilder {
        semver: Option<String>,
        vm: Option<String>,
        agent: Option<String>,
    }

    impl MetadataBuilder {
        pub fn new() -> MetadataBuilder {
            Default::default()
        }

        pub fn semver(mut self, semver: String) -> MetadataBuilder {
            self.semver = Some(semver);
            self
        }

        pub fn vm(mut self, vm: String) -> MetadataBuilder {
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
