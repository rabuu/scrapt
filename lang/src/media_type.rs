#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImgType {
    Svg,
    Png,
}

impl ImgType {
    pub fn file_extension(&self) -> &'static str {
        match *self {
            ImgType::Svg => "svg",
            ImgType::Png => "png",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioType {
    Wav,
    Mp4,
}

impl AudioType {
    pub fn file_extension(&self) -> &'static str {
        match *self {
            AudioType::Wav => "wav",
            AudioType::Mp4 => "mp4",
        }
    }
}
