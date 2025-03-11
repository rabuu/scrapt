use std::collections::HashMap;

use scratch_common_types::{Number, Value};
use serde::{Deserialize, Serialize};

use crate::block::Block;
use crate::common::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub is_stage: bool,
    pub name: Name,
    pub variables: HashMap<Id, Variable>,
    pub lists: HashMap<Id, List>,
    pub broadcasts: HashMap<Id, Broadcast>,
    pub blocks: HashMap<Id, Block>,
    pub comments: HashMap<Id, Comment>,
    pub current_costume: u32,
    pub costumes: Vec<Asset>,
    pub sounds: Vec<Asset>,
    pub layer_order: u32,
    pub volume: Percentage,

    #[serde(flatten)]
    pub target_type: TargetType,
}

impl Target {
    pub fn stage_builder() -> builder::StageBuilder {
        builder::StageBuilder::default()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TargetType {
    Stage(StageTarget),
    Sprite(SpriteTarget),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageTarget {
    pub tempo: u32,
    pub video_state: VideoState,
    pub video_transparency: Percentage,
    pub text_to_speech_language: Option<Language>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpriteTarget {
    pub visible: bool,
    pub x: Coord,
    pub y: Coord,
    pub size: Percentage,
    pub direction: Angle,
    pub draggable: bool,
    pub rotation_style: RotationStyle,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Variable {
    Simple(Name, Value),
    MaybeCloud(Name, Value, bool),
}

pub type List = (Name, Vec<Value>);
pub type Broadcast = Name;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub block_id: Id,
    pub x: CodeCoord,
    pub y: CodeCoord,
    pub width: CodeCoord,
    pub height: CodeCoord,
    pub minimized: bool,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_id: AssetId,
    pub name: Name,
    pub md5ext: String,
    pub data_format: String,

    #[serde(flatten)]
    pub asset_type: AssetType,
}

impl Asset {
    pub fn costume(id: AssetId, name: Name, filename: String, data_format: String) -> Asset {
        Asset {
            asset_id: id,
            name,
            md5ext: filename,
            data_format,

            // TODO
            asset_type: AssetType::Costume(CostumeAsset {
                bitmap_resolution: None,
                rotation_center_x: Number::Integer(240),
                rotation_center_y: Number::Integer(180),
            }),
        }
    }

    pub fn sound(id: AssetId, name: Name, filename: String, data_format: String) -> Asset {
        Asset {
            asset_id: id,
            name,
            md5ext: filename,
            data_format,

            // TODO
            // FIXME
            asset_type: AssetType::Sound(SoundAsset {
                rate: 42.0,
                sample_count: 1,
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssetType {
    Costume(CostumeAsset),
    Sound(SoundAsset),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostumeAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitmap_resolution: Option<Number>,
    pub rotation_center_x: Coord,
    pub rotation_center_y: Coord,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundAsset {
    pub rate: f32,
    pub sample_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VideoState {
    On,
    Off,
    OnFlipped,
}

// TODO: incomplete
#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    Danish,
    Dutch,
    English,
    French,
    German,
    Icelandic,
    Italian,
    Japanese,
    Polish,
    PortugueseBr,
    PortugueseEu,
    Russian,
    SpanishEu,
    SpanishLat,
    ChineseMan,
    Korean,
    Norwegian,
    Romanian,
    Swedish,
    Turkish,
    Welsh,
    Hindi,
    Arabic,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RotationStyle {
    #[serde(rename = "all around")]
    AllAround,
    #[serde(rename = "left-right")]
    LeftRight,
    #[serde(rename = "don't rotate")]
    DontRotate,
}

mod builder {
    use super::*;

    #[derive(Debug)]
    pub struct StageBuilder {
        variables: HashMap<Id, Variable>,
        lists: HashMap<Id, List>,
        broadcasts: HashMap<Id, Broadcast>,
        blocks: HashMap<Id, Block>,
        comments: HashMap<Id, Comment>,
        current_costume: Option<u32>,
        costumes: Vec<Asset>,
        sounds: Vec<Asset>,
        layer_order: u32,
        volume: Percentage,
        tempo: u32,
        video_state: VideoState,
        video_transparency: Percentage,
        text_to_speech_language: Option<Language>,
    }

    impl Default for StageBuilder {
        fn default() -> Self {
            Self {
                variables: Default::default(),
                lists: Default::default(),
                broadcasts: Default::default(),
                blocks: Default::default(),
                comments: Default::default(),
                current_costume: None,
                costumes: Default::default(),
                sounds: Default::default(),
                layer_order: 0,
                volume: 100,
                tempo: 60,
                video_state: VideoState::On,
                video_transparency: 50,
                text_to_speech_language: None,
            }
        }
    }

    impl StageBuilder {
        pub fn volume(mut self, volume: Percentage) -> StageBuilder {
            self.volume = volume;
            self
        }

        pub fn add_costume(mut self, costume: Asset) -> StageBuilder {
            self.costumes.push(costume);
            self
        }

        pub fn current_costume(mut self, index: Option<u32>) -> StageBuilder {
            self.current_costume = index;
            self
        }

        pub fn add_sound(mut self, sound: Asset) -> StageBuilder {
            self.sounds.push(sound);
            self
        }

        pub fn build(self) -> Target {
            debug_assert!(!self.costumes.is_empty(), "Target without costume");

            let stage = StageTarget {
                tempo: self.tempo,
                video_state: self.video_state,
                video_transparency: self.video_transparency,
                text_to_speech_language: self.text_to_speech_language,
            };

            Target {
                is_stage: true,
                name: Name::from("Stage"),
                variables: self.variables,
                lists: self.lists,
                broadcasts: self.broadcasts,
                blocks: self.blocks,
                comments: self.comments,
                current_costume: self.current_costume.unwrap_or(0),
                costumes: self.costumes,
                sounds: self.sounds,
                layer_order: self.layer_order,
                volume: self.volume,
                target_type: TargetType::Stage(stage),
            }
        }
    }
}
