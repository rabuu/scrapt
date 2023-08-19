pub use manifest_common::Id;

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Num(Number),
    Str(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
    Int(i64),
    Float(f32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Str(String),
    Bool(bool),
}

pub type Name = String;
pub type Coord = Number;
pub type CodeCoord = Number;
pub type Percentage = u16;
pub type Angle = u16;
pub type Color = String;
pub type Opcode = String;
pub type ArgArray = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdOrPrimitive {
    Id(Id),
    Primitive(block::PrimitiveBlock),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodePos {
    pub x: CodeCoord,
    pub y: CodeCoord,
}

impl CodePos {
    pub fn new(x: CodeCoord, y: CodeCoord) -> CodePos {
        CodePos { x, y }
    }
}
