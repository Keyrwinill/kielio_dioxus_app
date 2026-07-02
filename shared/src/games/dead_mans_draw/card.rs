use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Anchor,
    Cannon,
    Chest,
    Hook,
    Key,
    Kraken,
    Map,
    Mermaid,
    Oracle,
    Sword,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
}