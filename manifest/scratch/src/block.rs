use std::{collections::HashMap, marker::PhantomData};

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
    pub argumentids: ArgArray<Id>,

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
    pub argumentnames: ArgArray<String>,
    pub argumentdefaults: ArgArray<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct ArgArray<T: ToString>(String, #[serde(skip)] PhantomData<T>);

impl<T: ToString> ArgArray<T> {
    pub fn new() -> ArgArray<T> {
        ArgArray(String::new(), PhantomData)
    }

    pub fn with_capacity(cap: usize) -> ArgArray<T> {
        ArgArray(String::with_capacity(cap), PhantomData)
    }

    pub fn from_slice(elems: &[T]) -> ArgArray<T> {
        // TODO: capacity
        // let mut argarr = ArgArray::with_capacity(??);

        let mut argarr = ArgArray::new();
        argarr.push_slice(elems);
        argarr
    }

    pub fn push(&mut self, elem: &T) {
        if !self.0.is_empty() {
            self.0.push(',');
        }

        self.0.push_str(r#"[""#);
        self.0.push_str(&(elem.to_string()));
        self.0.push_str(r#""]"#);
    }

    pub fn push_slice(&mut self, elems: &[T]) {
        for elem in elems {
            self.push(elem)
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
        argumentids: ArgArray<Id>,
        warp: String,
    }

    impl ProcedureCallMutationBuilder {
        pub fn new(name: &str) -> ProcedureCallMutationBuilder {
            ProcedureCallMutationBuilder {
                proccode: String::from(name),
                argumentids: ArgArray::new(),
                warp: String::from("false"),
            }
        }

        pub fn add_label(mut self, label: &str) -> ProcedureCallMutationBuilder {
            self.proccode.push(' ');
            self.proccode.push_str(label);
            self
        }

        pub fn add_strnum_argument(mut self, id: &Id) -> ProcedureCallMutationBuilder {
            self.proccode.push_str(" %s");
            self.argumentids.push(id);
            self
        }

        pub fn add_bool_argument(mut self, id: &Id) -> ProcedureCallMutationBuilder {
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
        argumentids: ArgArray<Id>,
        warp: String,
        argumentnames: ArgArray<String>,
        argumentdefaults: ArgArray<String>,
    }

    impl ProcedurePrototypeMutationBuilder {
        pub fn new(name: &str) -> ProcedurePrototypeMutationBuilder {
            ProcedurePrototypeMutationBuilder {
                proccode: String::from(name),
                argumentids: ArgArray::new(),
                warp: String::from("false"),
                argumentnames: ArgArray::new(),
                argumentdefaults: ArgArray::new(),
            }
        }

        pub fn add_label(mut self, label: &str) -> ProcedurePrototypeMutationBuilder {
            self.proccode.push(' ');
            self.proccode.push_str(label);
            self
        }

        pub fn add_strnum_argument(
            mut self,
            id: &Id,
            name: String,
            def: String,
        ) -> ProcedurePrototypeMutationBuilder {
            self.proccode.push_str(" %s");
            self.argumentids.push(id);
            self.argumentnames.push(&name);
            self.argumentdefaults.push(&def);
            self
        }

        pub fn add_bool_argument(
            mut self,
            id: &Id,
            name: String,
            def: bool,
        ) -> ProcedurePrototypeMutationBuilder {
            self.proccode.push_str(" %b");
            self.argumentids.push(id);
            self.argumentnames.push(&name);
            self.argumentdefaults.push(&format!("{}", def));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_argarray() {
        let argarray: ArgArray<String> = serde_json::from_str(r#""[\"false\"]""#).unwrap();

        let mut expected = ArgArray::new();
        expected.push(&"false".to_string());

        assert_eq!(argarray, expected);
    }

    #[test]
    fn serialize_argarray() {
        let mut argarray = ArgArray::new();
        argarray.push(&"hallo".to_string());

        let serialized = serde_json::to_value(argarray).unwrap();
        assert_eq!(serialized.to_string(), String::from(r#""[\"hallo\"]""#));
    }
}
