pub mod block;
pub mod code_position;
pub mod extension;
pub mod id;
pub mod metadata;
pub mod monitor;
pub mod project;
pub mod string_array;
pub mod target;
pub mod value;

pub use code_position::CodePosition;
pub use id::Id;
pub use project::Project;
pub use value::{Number, Value};

use serde::{Deserialize, Serialize};

pub type Name = String;
pub type Opcode = String;
pub type Percentage = u16;
pub type Angle = u16;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdOrPrimitiveBlock {
    Id(Id),
    Primitive(block::PrimitiveBlock),
}
