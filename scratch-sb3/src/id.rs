use std::fmt;
use std::fmt::Write;

use serde::de::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id([char; 20]);

impl TryFrom<String> for Id {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let vec: Vec<char> = value.chars().collect();

        if vec.len() != 20 {
            return Err("ID must consist of 20 chars");
        }

        let arr: [char; 20] = match vec.try_into() {
            Ok(arr) => arr,
            Err(_) => return Err("Error while deserializing ID"),
        };

        Ok(Self(arr))
    }
}

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
        let id = Id::try_from(s).map_err(D::Error::custom)?;
        Ok(id)
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
