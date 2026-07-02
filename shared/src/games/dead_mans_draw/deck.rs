use rand::rng;
use rand::seq::SliceRandom;

use super::card::{Card, Suit};

pub fn create_deck() -> Vec<Card> {
    let suits = [
        Suit::Anchor,
        Suit::Cannon,
        Suit::Chest,
        Suit::Hook,
        Suit::Key,
        Suit::Kraken,
        Suit::Map,
        Suit::Mermaid,
        Suit::Oracle,
        Suit::Sword,
    ];

    let mut deck = Vec::new();

    for suit in suits {
        let values = if suit == Suit::Mermaid {
            4..=9
        } else {
            2..=7
        };

        for value in values {
            deck.push(Card { suit, value });
        }
    }

    let mut rng = rng();
    deck.shuffle(&mut rng);

    deck
}