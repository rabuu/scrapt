use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub is_stage: bool,
    pub name: Name,
    pub variables: HashMap<Id, Variable>,
    pub lists: HashMap<Id, List>,
    pub broadcasts: HashMap<Id, Broadcast>,
    pub blocks: HashMap<Id, Block>,
    pub comments: HashMap<Id, Comment>,
    pub current_costume: u32,
    pub costumes: Vec<Asset>,
    pub sounds: Vec<Asset>,
    pub layer_order: u32,
    pub volume: Percentage,

    #[serde(flatten)]
    pub target_type: TargetType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TargetType {
    Stage(StageTarget),
    Sprite(SpriteTarget),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageTarget {
    pub tempo: u32,
    pub video_state: VideoState,
    pub video_transparency: Percentage,
    pub text_to_speech_language: Option<Language>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpriteTarget {
    pub visible: bool,
    pub x: Coord,
    pub y: Coord,
    pub size: Percentage,
    pub direction: Angle,
    pub draggable: bool,
    pub rotation_style: RotationStyle,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Variable {
    Simple(Name, Value),
    MaybeCloud(Name, Value, bool),
}

pub type List = (Name, Vec<Value>);
pub type Broadcast = Name;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    Full(FullBlock),
    Short(ShortBlock),
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub block_id: Id,
    pub x: CodeCoord,
    pub y: CodeCoord,
    pub width: CodeCoord,
    pub height: CodeCoord,
    pub minimized: bool,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_id: Id,
    pub name: Name,
    pub md5ext: String,
    pub data_format: String,

    #[serde(flatten)]
    pub asset_type: AssetType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetType {
    Costume(CostumeAsset),
    Sound(SoundAsset),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostumeAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitmap_resolution: Option<Number>,
    pub rotation_center_x: Coord,
    pub rotation_center_y: Coord,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundAsset {
    pub rate: f32,
    pub sample_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VideoState {
    On,
    Off,
    OnFlipped,
}

// TODO: incomplete
#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    Danish,
    Dutch,
    English,
    French,
    German,
    Icelandic,
    Italian,
    Japanese,
    Polish,
    PortugueseBr,
    PortugueseEu,
    Russian,
    SpanishEu,
    SpanishLat,
    ChineseMan,
    Korean,
    Norwegian,
    Romanian,
    Swedish,
    Turkish,
    Welsh,
    Hindi,
    Arabic,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RotationStyle {
    #[serde(rename = "all around")]
    AllAround,
    #[serde(rename = "left-right")]
    LeftRight,
    #[serde(rename = "don't rotate")]
    DontRotate,
}
