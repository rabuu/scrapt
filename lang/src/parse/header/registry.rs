use std::collections::HashMap;

use super::costumes::Costume;

#[derive(Debug, Default)]
pub struct HeaderRegistry {
    pub costumes: HashMap<String, Costume>,
    pub costumes_list: Vec<String>,
    pub current_costume: Option<usize>,
}
