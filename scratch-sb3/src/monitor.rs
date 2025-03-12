use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Id, Name, Number, Opcode, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: Id,
    pub mode: MonitorMode,
    pub opcode: Opcode,
    pub params: HashMap<Name, String>,
    pub sprite_name: Option<Name>,
    pub value: MonitorValue,
    pub width: Number,
    pub height: Number,
    pub x: Number,
    pub y: Number,
    pub visible: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub slider: Option<Slider>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slider {
    pub slider_min: Number,
    pub slider_max: Number,
    pub is_discrete: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
