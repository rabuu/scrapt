#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
