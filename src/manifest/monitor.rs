use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: Id,
    pub mode: MonitorMode,
    pub opcode: Opcode,
    pub params: HashMap<Name, String>,
    pub sprite_name: Option<Name>,
    pub value: MonitorValue,
    pub width: Coord,
    pub height: Coord,
    pub x: Coord,
    pub y: Coord,
    pub visible: bool,

    // not belonging to lists
    pub slider_min: Option<Number>,
    pub slider_max: Option<Number>,
    pub is_discrete: Option<bool>,
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
