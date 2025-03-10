use std::fmt::{self, Write};

use scratch_common_types::Number;
use serde::de::Error;

use crate::*;

pub type Name = String;
pub type Coord = Number;
pub type CodeCoord = Number;
pub type Percentage = u16;
pub type Angle = u16;
pub type Color = String;
pub type Opcode = String;
pub type AssetId = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Str(String),
    Bool(bool),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdOrPrimitive {
    Id(Id),
    Primitive(block::PrimitiveBlock),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodePos {
    pub x: CodeCoord,
    pub y: CodeCoord,
}

impl CodePos {
    pub fn new(x: CodeCoord, y: CodeCoord) -> CodePos {
        CodePos { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Id([char; 20]);

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = String::from_iter(self.0.iter());
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let vec: Vec<char> = s.chars().collect();

        if vec.len() != 20 {
            return Err(D::Error::custom("ID must consist of 20 chars"));
        }

        let arr: [char; 20] = match vec.try_into() {
            Ok(arr) => arr,
            Err(_) => return Err(D::Error::custom("Error while deserializing ID")),
        };

        Ok(Id(arr))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.0 {
            f.write_char(*c)?;
        }
        Ok(())
    }
}
