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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub not_list: Option<NotListMonitor>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotListMonitor {
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
