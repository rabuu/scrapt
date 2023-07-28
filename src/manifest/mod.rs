use serde::{Deserialize, Serialize};

pub mod extension;
pub mod meta;
pub mod monitor;
pub mod target;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub targets: Vec<target::Target>,
    pub monitors: Vec<monitor::Monitor>,
    pub extensions: Vec<extension::Extension>,
    pub meta: meta::Metadata,
}

mod common {
    use super::*;

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

    pub type Id = String;
    pub type Name = String;
    pub type Coord = f64;
    pub type CodeCoord = u32;
    pub type Percentage = u8;
    pub type Angle = u16;
    pub type Opcode = String;
    pub type Version = String;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum IdOrAnonymous {
        Id(String),
        Anonymous(target::ShortBlock),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CodePos {
        pub x: CodeCoord,
        pub y: CodeCoord,
    }
}
