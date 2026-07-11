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

impl Suit {
    pub fn sort_order(self) -> u8 {
        match self {
            Suit::Anchor => 0,
            Suit::Cannon => 1,
            Suit::Chest => 2,
            Suit::Hook => 3,
            Suit::Key => 4,
            Suit::Kraken => 5,
            Suit::Map => 6,
            Suit::Mermaid => 7,
            Suit::Oracle => 8,
            Suit::Sword => 9,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub value: u8,
}
