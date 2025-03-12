use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
