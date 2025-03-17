use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::string_array::StringArray;
use crate::{Id, IdOrPrimitiveBlock, Name, Number, Opcode, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    Primitive(PrimitiveBlock),
    Full(Box<FullBlock>),
}

impl Block {
    pub fn builder() -> builder::BlockBuilder {
        builder::BlockBuilder
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrimitiveBlock {
    // TODO: investigate different numeral modes
    Simple(u8, Value),
    Advanced(u8, Name, Id),
    AdvancedWithPos(u8, Name, Id, Number, Number),
}

impl PrimitiveBlock {
    pub fn builder() -> builder::PrimitiveBlockBuilder {
        builder::PrimitiveBlockBuilder
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
    pub position: Option<CodePosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation: Option<Mutation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Simple(u8, IdOrPrimitiveBlock),
    Obscuring(u8, IdOrPrimitiveBlock, IdOrPrimitiveBlock),
}

impl Input {
    pub fn builder() -> builder::InputBuilder {
        builder::InputBuilder
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodePosition {
    pub x: Number,
    pub y: Number,
}

impl CodePosition {
    pub fn new(x: Number, y: Number) -> CodePosition {
        CodePosition { x, y }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    pub tag_name: String,
    pub children: [(); 0],

    #[serde(flatten)]
    pub mutation_type: MutationType,
}

impl Mutation {
    pub fn builder() -> builder::MutationBuilder {
        builder::MutationBuilder
    }
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
    pub argumentids: StringArray<Id>,

    // TODO: maybe de/serialize it to/from bool
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
    pub argumentnames: StringArray<String>,
    pub argumentdefaults: StringArray<String>,
}

pub mod builder {
    use super::*;
    use crate::Angle;

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
        position: Option<CodePosition>,
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
                position: None,
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

        pub fn top_level_pos(mut self, pos: CodePosition) -> FullBlockBuilder {
            self.top_level = true;
            self.position = Some(pos);
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
                position: self.position,
                comment: self.comment,
                mutation: self.mutation,
            }
        }
    }

    pub struct PrimitiveBlockBuilder;

    impl PrimitiveBlockBuilder {
        pub fn number(self, num: Number) -> PrimitiveBlock {
            PrimitiveBlock::Simple(4, Value::Number(num))
        }

        // TODO
        // pub fn positive_number(self, num: PositiveNumber)

        pub fn positive_integer(self, num: u32) -> PrimitiveBlock {
            PrimitiveBlock::Simple(6, Value::Number(Number::Integer(num as i32)))
        }

        pub fn integer(self, num: i32) -> PrimitiveBlock {
            PrimitiveBlock::Simple(7, Value::Number(Number::Integer(num)))
        }

        pub fn angle(self, angle: Angle) -> PrimitiveBlock {
            PrimitiveBlock::Simple(8, Value::Number(Number::Integer(angle as i32)))
        }

        pub fn color(self, color: String) -> PrimitiveBlock {
            PrimitiveBlock::Simple(9, Value::String(color))
        }

        pub fn string(self, string: String) -> PrimitiveBlock {
            PrimitiveBlock::Simple(10, Value::String(string))
        }

        pub fn broadcast(self, name: Name, id: Id) -> PrimitiveBlock {
            PrimitiveBlock::Advanced(11, name, id)
        }

        pub fn variable(self, name: Name, id: Id, pos: Option<CodePosition>) -> PrimitiveBlock {
            if let Some(pos) = pos {
                PrimitiveBlock::AdvancedWithPos(12, name, id, pos.x, pos.y)
            } else {
                PrimitiveBlock::Advanced(12, name, id)
            }
        }

        pub fn list(self, name: Name, id: Id, pos: Option<CodePosition>) -> PrimitiveBlock {
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
            Input::Simple(2, IdOrPrimitiveBlock::Id(id))
        }

        pub fn primitive(self, block: PrimitiveBlock) -> Input {
            Input::Simple(2, IdOrPrimitiveBlock::Primitive(block))
        }
    }

    pub struct ShadowInputBuilder;

    impl ShadowInputBuilder {
        pub fn id(self, id: Id) -> Input {
            Input::Simple(1, IdOrPrimitiveBlock::Id(id))
        }

        pub fn primitive(self, block: PrimitiveBlock) -> Input {
            Input::Simple(1, IdOrPrimitiveBlock::Primitive(block))
        }
    }

    pub struct BeginningObscuringInputBuilder;

    impl BeginningObscuringInputBuilder {
        pub fn id(self, id: Id) -> FinishingObscuringInputBuilder {
            FinishingObscuringInputBuilder::new(IdOrPrimitiveBlock::Id(id))
        }

        pub fn primitive(self, block: PrimitiveBlock) -> FinishingObscuringInputBuilder {
            FinishingObscuringInputBuilder::new(IdOrPrimitiveBlock::Primitive(block))
        }
    }

    pub struct FinishingObscuringInputBuilder {
        input: IdOrPrimitiveBlock,
    }

    impl FinishingObscuringInputBuilder {
        pub fn new(input: IdOrPrimitiveBlock) -> FinishingObscuringInputBuilder {
            FinishingObscuringInputBuilder { input }
        }

        pub fn shadow_id(self, id: Id) -> Input {
            Input::Obscuring(3, self.input, IdOrPrimitiveBlock::Id(id))
        }

        pub fn shadow_primitve(self, block: PrimitiveBlock) -> Input {
            Input::Obscuring(3, self.input, IdOrPrimitiveBlock::Primitive(block))
        }
    }

    pub struct MutationBuilder;

    impl MutationBuilder {
        pub fn procedure_call(self, name: &str) -> ProcedureCallMutationBuilder {
            ProcedureCallMutationBuilder::new(name)
        }

        pub fn procedure_prototype(self, name: &str) -> ProcedurePrototypeMutationBuilder {
            ProcedurePrototypeMutationBuilder::new(name)
        }

        /// Construct a `control_stop` mutation
        ///
        /// hasnext: Whether the block has a block following it or not
        ///          (`false` for "stop all" and "stop all in sprite", `true` for "stop other scripts in sprite")
        pub fn control_stop(self, hasnext: bool) -> Mutation {
            Mutation {
                tag_name: String::from("mutation"),
                children: [],
                mutation_type: MutationType::ControlStop(ControlStopMutation { hasnext }),
            }
        }
    }

    pub struct ProcedureCallMutationBuilder {
        proccode: String,
        argumentids: StringArray<Id>,
        warp: String,
    }

    impl ProcedureCallMutationBuilder {
        pub fn new(name: &str) -> ProcedureCallMutationBuilder {
            ProcedureCallMutationBuilder {
                proccode: String::from(name),
                argumentids: StringArray::new(),
                warp: String::from("false"),
            }
        }

        pub fn add_label(mut self, label: &str) -> ProcedureCallMutationBuilder {
            self.proccode.push(' ');
            self.proccode.push_str(label);
            self
        }

        pub fn add_strnum_argument(mut self, id: Id) -> ProcedureCallMutationBuilder {
            self.proccode.push_str(" %s");
            self.argumentids.push(id);
            self
        }

        pub fn add_bool_argument(mut self, id: Id) -> ProcedureCallMutationBuilder {
            self.proccode.push_str(" %b");
            self.argumentids.push(id);
            self
        }

        pub fn warp(mut self, warp: String) -> ProcedureCallMutationBuilder {
            self.warp = warp;
            self
        }

        pub fn build(self) -> Mutation {
            Mutation {
                tag_name: String::from("mutation"),
                children: [],
                mutation_type: MutationType::Procedure(ProcedureMutation {
                    proccode: self.proccode,
                    argumentids: self.argumentids,
                    warp: self.warp,
                    prototype: None,
                }),
            }
        }
    }

    pub struct ProcedurePrototypeMutationBuilder {
        proccode: String,
        argumentids: StringArray<Id>,
        warp: String,
        argumentnames: StringArray<String>,
        argumentdefaults: StringArray<String>,
    }

    impl ProcedurePrototypeMutationBuilder {
        pub fn new(name: &str) -> ProcedurePrototypeMutationBuilder {
            ProcedurePrototypeMutationBuilder {
                proccode: String::from(name),
                argumentids: StringArray::new(),
                warp: String::from("false"),
                argumentnames: StringArray::new(),
                argumentdefaults: StringArray::new(),
            }
        }

        pub fn add_label(mut self, label: &str) -> ProcedurePrototypeMutationBuilder {
            self.proccode.push(' ');
            self.proccode.push_str(label);
            self
        }

        pub fn add_strnum_argument(
            mut self,
            id: Id,
            name: String,
            def: String,
        ) -> ProcedurePrototypeMutationBuilder {
            self.proccode.push_str(" %s");
            self.argumentids.push(id);
            self.argumentnames.push(name);
            self.argumentdefaults.push(def);
            self
        }

        pub fn add_bool_argument(
            mut self,
            id: Id,
            name: String,
            def: bool,
        ) -> ProcedurePrototypeMutationBuilder {
            self.proccode.push_str(" %b");
            self.argumentids.push(id);
            self.argumentnames.push(name);
            self.argumentdefaults.push(def.to_string());
            self
        }

        pub fn warp(mut self, warp: String) -> ProcedurePrototypeMutationBuilder {
            self.warp = warp;
            self
        }

        pub fn build(self) -> Mutation {
            Mutation {
                tag_name: String::from("mutation"),
                children: [],
                mutation_type: MutationType::Procedure(ProcedureMutation {
                    proccode: self.proccode,
                    argumentids: self.argumentids,
                    warp: self.warp,
                    prototype: Some(PrototypeMutation {
                        argumentnames: self.argumentnames,
                        argumentdefaults: self.argumentdefaults,
                    }),
                }),
            }
        }
    }
}
