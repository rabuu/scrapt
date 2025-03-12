use serde::{Deserialize, Serialize};

use crate::Number;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodePosition {
    pub x: Number,
    pub y: Number,
}

impl CodePosition {
    pub fn new(x: Number, y: Number) -> CodePosition {
        CodePosition { x, y }
    }
}
