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
pub struct Target {
    is_stage: bool,
    name: String,
    variables: HashMap<String, Variable>,
    lists: HashMap<String, List>,
    broadcasts: HashMap<String, Broadcast>,
    blocks: HashMap<String, Block>,
    comments: HashMap<String, Comment>,
    current_costume: u32,
    costumes: Vec<Asset>,
    sounds: Vec<Asset>,
    layer_order: u32,
    volume: u8,

    // stage
    tempo: Option<u32>,
    video_state: Option<VideoState>,
    video_transparency: Option<u8>,
    text_to_speech_language: Option<Language>,
}

type Variable = (String, i32, bool);
type List = (String, Vec<String>);
type Broadcast = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    Full(FullBlock),
    Short(ShortBlock),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBlock {
    opcode: String,
    next: Option<String>,
    parent: Option<String>,
    inputs: HashMap<String, Input>,
    fields: HashMap<String, Field>,
    shadow: bool,
    top_level: bool,
    x: Option<u32>,
    y: Option<u32>,
    comment: Option<String>,
    mutation: Option<Mutation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShortBlock {
    // TODO: investigate different numeral modes
    Simple(u8, String),
    Advanced(u8, String, String),
    AdvancedWithPos(u8, String, String, u32, u32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Simple(u8, IdOrAnonymous),
    Obscured(u8, IdOrAnonymous, IdOrAnonymous),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdOrAnonymous {
    Id(String),
    Anonymous(ShortBlock),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Field {
    Simple(String),
    WithId(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mutation {
    tag_name: String,
    children: [(); 0],

    // "procedures_prototype" and "procedures_call"
    proccode: Option<String>,
    argumentids: Option<Vec<String>>,
    warp: Option<bool>,

    // "procedures_prototype"
    argumentnames: Option<Vec<String>>,
    argumentdefaults: Option<Vec<ArgDefault>>,

    // "control_stop"
    hasnext: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgDefault {
    Str(String),
    Bool(bool),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    block_id: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    minimized: bool,
    test: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    asset_id: String,
    name: String,
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
        rotation_center_x: u32,
        rotation_center_y: u32,
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
