use serde::{Deserialize, Serialize};

use super::card::Card;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub bank: Vec<Card>,
    pub is_ai: bool,
}

impl Player {
    pub fn new(name: &str, is_ai: bool) -> Self {
        Self {
            name: name.to_string(),
            bank: Vec::new(),
            is_ai,
        }
    }
}
