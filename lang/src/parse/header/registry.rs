use std::collections::HashMap;

use super::{costumes::Costume, sounds::Sound};

#[derive(Debug, Default)]
pub struct HeaderRegistry {
    pub costumes: HashMap<String, Costume>,
    pub costumes_list: Vec<String>,
    pub current_costume: Option<u32>,

    pub sounds: HashMap<String, Sound>,
    pub sounds_list: Vec<String>,
}
