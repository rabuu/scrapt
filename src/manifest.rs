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
    costumes: Vec<Costume>,
    sounds: Vec<Sound>,
    layer_order: u32,
    volume: u8,

    // stage
    tempo: Option<u32>,
    video_state: Option<VideoState>,
    video_transparency: Option<u8>,
    text_to_speech_language: Option<Language>,
}

type Variable = (String, i64, bool);
type List = (String, Vec<String>);
type Broadcast = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
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

// TODO
type Input = ();
type Field = ();
