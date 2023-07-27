use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    targets: Vec<Target>,
    monitors: Vec<Monitor>,
    extensions: Vec<Extension>,
    meta: Metadata,
}

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
pub type Coord = u32;
pub type CodeCoord = u32;
pub type Percentage = u8;
pub type Angle = u16;
pub type Opcode = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdOrAnonymous {
    Id(String),
    Anonymous(ShortBlock),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodePos {
    x: CodeCoord,
    y: CodeCoord,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    is_stage: bool,
    name: Name,
    variables: HashMap<Id, Variable>,
    lists: HashMap<Id, List>,
    broadcasts: HashMap<Id, Broadcast>,
    blocks: HashMap<Id, Block>,
    comments: HashMap<Id, Comment>,
    current_costume: u32,
    costumes: Vec<Asset>,
    sounds: Vec<Asset>,
    layer_order: u32,
    volume: Percentage,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    stage: Option<StageTarget>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    sprite: Option<SpriteTarget>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StageTarget {
    tempo: u32,
    video_state: VideoState,
    video_transparency: Percentage,
    text_to_speech_language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteTarget {
    visible: bool,
    x: Coord,
    y: Coord,
    size: Percentage,
    direction: Angle,
    draggable: bool,
    rotation_style: RotationStyle,
}

pub type Variable = (Name, Value, bool);
pub type List = (Name, Vec<Value>);
pub type Broadcast = Name;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    Full(FullBlock),
    Short(ShortBlock),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBlock {
    opcode: Opcode,
    next: Option<Id>,
    parent: Option<Id>,
    inputs: HashMap<Name, Input>,
    fields: HashMap<Name, Field>,
    shadow: bool,
    top_level: bool,

    // top level blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pos: Option<CodePos>,

    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation: Option<Mutation>,
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
    WithId(Value, Id),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mutation {
    tag_name: String,
    children: [(); 0],

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    proc: Option<ProcMutation>,

    // "control_stop"
    #[serde(skip_serializing_if = "Option::is_none")]
    hasnext: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcMutation {
    proccode: String,
    argumentids: Vec<Id>,
    warp: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    prototype: Option<PrototypeMutation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrototypeMutation {
    argumentnames: Vec<Name>,
    argumentdefaults: Vec<Argument>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    block_id: Id,
    x: CodeCoord,
    y: CodeCoord,
    width: CodeCoord,
    height: CodeCoord,
    minimized: bool,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    asset_id: Id,
    name: Name,
    md5ext: String,
    data_format: String,

    #[serde(flatten)]
    asset_type: AssetType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetType {
    Costume {
        bitmap_resolution: Option<f32>,
        rotation_center_x: Coord,
        rotation_center_y: Coord,
    },
    Sound {
        rate: f32,
        sample_count: u32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VideoState {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on-flipped")]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    id: Id,
    mode: MonitorMode,
    opcode: Opcode,
    params: HashMap<Name, String>,
    sprite_name: Option<Name>,
    value: MonitorValue,
    width: Coord,
    height: Coord,
    x: Coord,
    y: Coord,
    visible: bool,

    // not belonging to lists
    slider_min: Option<Number>,
    slider_max: Option<Number>,
    is_discrete: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MonitorMode {
    Default,
    Large,
    Slider,
    List,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitorValue {
    Value(Value),
    Array(Vec<Value>),
}
