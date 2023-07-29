use serde::{Deserialize, Serialize};

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
