pub mod extension;
pub mod meta;
pub mod scratch;
pub mod unscratch;

pub mod common {
    use super::*;

    pub type Version = String;

    pub use extension::Extension;
    pub use meta::Metadata;
}
