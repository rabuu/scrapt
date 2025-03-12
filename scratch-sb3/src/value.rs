use std::{cmp, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(Number),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "\"{s}\""),
        }
    }
}

impl From<Number> for Value {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(x) => write!(f, "{x}"),
            Number::Float(x) => write!(f, "{x}"),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        f32::from(*self).partial_cmp(&f32::from(*other))
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> Self {
        match value {
            Number::Integer(i) => i as f32,
            Number::Float(f) => f,
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}
