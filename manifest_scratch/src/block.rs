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
    pub fields: HashMap<Name, (Value, Option<Id>)>,
    pub shadow: bool,
    pub top_level: bool,

    // top level blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub pos: Option<CodePos>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Id>,
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
    Simple(u8, IdOrPrimitive),
    Obscuring(u8, IdOrPrimitive, IdOrPrimitive),
}

impl Input {
    pub fn builder() -> builder::InputBuilder {
        builder::InputBuilder
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    pub tag_name: String,
    pub children: [(); 0],

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub mutation_type: Option<MutationType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MutationType {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ArgArray(String);

impl ArgArray {
    pub fn new() -> ArgArray {
        ArgArray(String::new())
    }

    pub fn with_capacity(cap: usize) -> ArgArray {
        ArgArray(String::with_capacity(cap))
    }

    pub fn from_slice(ids: &[Id]) -> ArgArray {
        // TODO: capacity
        // let mut argarr = ArgArray::with_capacity(??);

        let mut argarr = ArgArray::new();
        argarr.push_slice(ids);
        argarr
    }

    pub fn builder() -> builder::ArgArrayBuilder {
        builder::ArgArrayBuilder::new()
    }

    pub fn push(&mut self, id: &Id) {
        self.0.push_str(r#"[\""#);
        self.0.push_str(&(id.to_string()));
        self.0.push_str(r#"\"]"#);
    }

    pub fn push_slice(&mut self, ids: &[Id]) {
        for id in ids {
            self.push(id)
        }
    }
}

pub mod builder {
    use super::*;

    pub struct BlockBuilder;

    impl BlockBuilder {
        pub fn full(self, opcode: Opcode) -> FullBlockBuilder {
            FullBlockBuilder::new(opcode)
        }

        pub fn primitive(self) -> PrimitiveBlockBuilder {
            PrimitiveBlockBuilder
        }
    }

    pub struct FullBlockBuilder {
        opcode: Opcode,
        next: Option<Id>,
        parent: Option<Id>,
        inputs: HashMap<Name, Input>,
        fields: HashMap<Name, (Value, Option<Id>)>,
        shadow: bool,
        top_level: bool,
        pos: Option<CodePos>,
        comment: Option<Id>,
        mutation: Option<Mutation>,
    }

    impl FullBlockBuilder {
        pub fn new(opcode: Opcode) -> FullBlockBuilder {
            FullBlockBuilder {
                opcode,
                next: None,
                parent: None,
                inputs: HashMap::new(),
                fields: HashMap::new(),
                shadow: false,
                top_level: false,
                pos: None,
                comment: None,
                mutation: None,
            }
        }

        pub fn next(mut self, id: Id) -> FullBlockBuilder {
            self.next = Some(id);
            self
        }

        pub fn parent(mut self, id: Id) -> FullBlockBuilder {
            self.parent = Some(id);
            self
        }

        pub fn add_input(mut self, name: Name, input: Input) -> FullBlockBuilder {
            self.inputs.insert(name, input);
            self
        }

        pub fn add_field(mut self, name: Name, value: Value, id: Option<Id>) -> FullBlockBuilder {
            self.fields.insert(name, (value, id));
            self
        }

        pub fn shadow(mut self) -> FullBlockBuilder {
            self.shadow = true;
            self
        }

        pub fn top_level_pos(mut self, pos: CodePos) -> FullBlockBuilder {
            self.top_level = true;
            self.pos = Some(pos);
            self
        }

        pub fn comment(mut self, id: Id) -> FullBlockBuilder {
            self.comment = Some(id);
            self
        }

        pub fn mutation(mut self, mutation: Mutation) -> FullBlockBuilder {
            self.mutation = Some(mutation);
            self
        }

        pub fn build(self) -> FullBlock {
            FullBlock {
                opcode: self.opcode,
                next: self.next,
                parent: self.parent,
                inputs: self.inputs,
                fields: self.fields,
                shadow: self.shadow,
                top_level: self.top_level,
                pos: self.pos,
                comment: self.comment,
                mutation: self.mutation,
            }
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

    pub struct InputBuilder;

    impl InputBuilder {
        pub fn shadow(self) -> ShadowInputBuilder {
            ShadowInputBuilder
        }

        pub fn obscuring(self) -> BeginningObscuringInputBuilder {
            BeginningObscuringInputBuilder
        }

        pub fn id(self, id: Id) -> Input {
            Input::Simple(2, IdOrPrimitive::Id(id))
        }

        pub fn primitive(self, block: PrimitiveBlock) -> Input {
            Input::Simple(2, IdOrPrimitive::Primitive(block))
        }
    }

    pub struct ShadowInputBuilder;

    impl ShadowInputBuilder {
        pub fn id(self, id: Id) -> Input {
            Input::Simple(1, IdOrPrimitive::Id(id))
        }

        pub fn primitive(self, block: PrimitiveBlock) -> Input {
            Input::Simple(1, IdOrPrimitive::Primitive(block))
        }
    }

    pub struct BeginningObscuringInputBuilder;

    impl BeginningObscuringInputBuilder {
        pub fn id(self, id: Id) -> FinishingObscuringInputBuilder {
            FinishingObscuringInputBuilder::new(IdOrPrimitive::Id(id))
        }

        pub fn primitive(self, block: PrimitiveBlock) -> FinishingObscuringInputBuilder {
            FinishingObscuringInputBuilder::new(IdOrPrimitive::Primitive(block))
        }
    }

    pub struct FinishingObscuringInputBuilder {
        input: IdOrPrimitive,
    }

    impl FinishingObscuringInputBuilder {
        pub fn new(input: IdOrPrimitive) -> FinishingObscuringInputBuilder {
            FinishingObscuringInputBuilder { input }
        }

        pub fn shadow_id(self, id: Id) -> Input {
            Input::Obscuring(3, self.input, IdOrPrimitive::Id(id))
        }

        pub fn shadow_primitve(self, block: PrimitiveBlock) -> Input {
            Input::Obscuring(3, self.input, IdOrPrimitive::Primitive(block))
        }
    }

    pub struct ArgArrayBuilder(ArgArray);

    impl ArgArrayBuilder {
        pub fn new() -> ArgArrayBuilder {
            ArgArrayBuilder(ArgArray::new())
        }

        pub fn add_id(mut self, id: &Id) -> ArgArrayBuilder {
            self.0.push(id);
            self
        }

        pub fn build(self) -> ArgArray {
            self.0
        }
    }
}
