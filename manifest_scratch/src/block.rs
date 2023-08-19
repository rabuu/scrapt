use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    Full(FullBlock),
    Short(ShortBlock),
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
pub enum ShortBlock {
    // TODO: investigate different numeral modes
    Simple(u8, Value),
    Advanced(u8, Name, Id),
    AdvancedWithPos(u8, Name, Id, CodeCoord, CodeCoord),
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

        pub fn short(self) -> ShortBlockBuilder {
            ShortBlockBuilder
        }
    }

    pub struct FullBlockBuilder;

    impl FullBlockBuilder {
        pub fn new() -> FullBlockBuilder {
            FullBlockBuilder
        }
    }

    pub struct ShortBlockBuilder;

    impl ShortBlockBuilder {
        pub fn number(self, num: Number) -> ShortBlock {
            ShortBlock::Simple(4, Value::Num(num))
        }
    }
}
