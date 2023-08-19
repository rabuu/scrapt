use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    Full(FullBlock),
    Primitive(PrimitiveBlock),
}

impl Block {
    pub fn builder() -> builder::BlockBuilder {
        builder::BlockBuilder
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullBlock {
    pub opcode: Opcode,
    pub next: Option<Id>,
    pub parent: Option<Id>,
    pub inputs: HashMap<Name, Input>,
    pub fields: HashMap<Name, Field>,
    pub shadow: bool,
    pub top_level: bool,

    // top level blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub pos: Option<CodePos>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation: Option<Mutation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrimitiveBlock {
    // TODO: investigate different numeral modes
    Simple(u8, Value),
    Advanced(u8, Name, Id),
    AdvancedWithPos(u8, Name, Id, CodeCoord, CodeCoord),
}

impl PrimitiveBlock {
    pub fn builder() -> builder::PrimitiveBlockBuilder {
        builder::PrimitiveBlockBuilder
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Simple(u8, IdOrAnonymous),
    Obscured(u8, IdOrAnonymous, IdOrAnonymous),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Field {
    Simple(Value),
    MaybeWithId(Value, Option<Id>),
    WithId(Value, Id),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    pub tag_name: String,
    pub children: [(); 0],

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub special_mutation: Option<SpecialMutation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpecialMutation {
    Procedure(ProcedureMutation),
    ControlStop(ControlStopMutation),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcedureMutation {
    pub proccode: String,
    pub argumentids: ArgArray,
    pub warp: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub prototype: Option<PrototypeMutation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlStopMutation {
    pub hasnext: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrototypeMutation {
    pub argumentnames: ArgArray,
    pub argumentdefaults: ArgArray,
}

pub mod builder {
    use super::*;

    pub struct BlockBuilder;

    impl BlockBuilder {
        pub fn full(self) -> FullBlockBuilder {
            FullBlockBuilder::new()
        }

        pub fn primitive(self) -> PrimitiveBlockBuilder {
            PrimitiveBlockBuilder
        }
    }

    pub struct FullBlockBuilder;

    impl FullBlockBuilder {
        pub fn new() -> FullBlockBuilder {
            FullBlockBuilder
        }
    }

    pub struct PrimitiveBlockBuilder;

    impl PrimitiveBlockBuilder {
        pub fn number(self, num: Number) -> PrimitiveBlock {
            PrimitiveBlock::Simple(4, Value::Num(num))
        }

        // TODO
        // pub fn positive_number(self, num: PositiveNumber)

        pub fn positive_integer(self, num: u32) -> PrimitiveBlock {
            PrimitiveBlock::Simple(6, Value::Num(Number::Int(num as i64)))
        }

        pub fn integer(self, num: i64) -> PrimitiveBlock {
            PrimitiveBlock::Simple(7, Value::Num(Number::Int(num)))
        }

        pub fn angle(self, angle: Angle) -> PrimitiveBlock {
            PrimitiveBlock::Simple(8, Value::Num(Number::Int(angle as i64)))
        }

        pub fn color(self, color: Color) -> PrimitiveBlock {
            PrimitiveBlock::Simple(9, Value::Str(color))
        }

        pub fn string(self, string: String) -> PrimitiveBlock {
            PrimitiveBlock::Simple(10, Value::Str(string))
        }

        pub fn broadcast(self, name: Name, id: Id) -> PrimitiveBlock {
            PrimitiveBlock::Advanced(11, name, id)
        }

        pub fn variable(self, name: Name, id: Id, pos: Option<CodePos>) -> PrimitiveBlock {
            if let Some(pos) = pos {
                PrimitiveBlock::AdvancedWithPos(12, name, id, pos.x, pos.y)
            } else {
                PrimitiveBlock::Advanced(12, name, id)
            }
        }

        pub fn list(self, name: Name, id: Id, pos: Option<CodePos>) -> PrimitiveBlock {
            if let Some(pos) = pos {
                PrimitiveBlock::AdvancedWithPos(13, name, id, pos.x, pos.y)
            } else {
                PrimitiveBlock::Advanced(13, name, id)
            }
        }
    }
}
