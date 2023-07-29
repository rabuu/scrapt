use serde::{Deserialize, Serialize};

pub type Version = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Extension {
    Pen,
    Wedo2,
    Music,
    Microbit,
    Text2speech,
    Translate,
    VideoSensing,
    Ev3,
    Makeymakey,
    Boost,
    Gdxfor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub semver: Version,
    pub vm: Version,
    pub agent: String,
}
