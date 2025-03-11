use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImgType {
    Svg,
    Png,
}

impl ImgType {
    pub fn extension(&self) -> &'static str {
        match self {
            ImgType::Svg => "svg",
            ImgType::Png => "png",
        }
    }
}

impl fmt::Display for ImgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImgType::Svg => write!(f, "SVG"),
            ImgType::Png => write!(f, "PNG"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioType {
    Wav,
    Mp4,
}

impl AudioType {
    pub fn extension(&self) -> &'static str {
        match self {
            AudioType::Wav => "wav",
            AudioType::Mp4 => "mp4",
        }
    }
}

impl fmt::Display for AudioType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioType::Wav => write!(f, "WAV"),
            AudioType::Mp4 => write!(f, "MP4"),
        }
    }
}
