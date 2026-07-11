use rand::rng;
use rand::seq::SliceRandom;

use crate::games::dead_mans_draw::variant::GameVariant;

use super::card::{Card, Suit};

impl Suit {
    pub fn all() -> [Suit; 10] {
        [
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
        ]
    }
}

pub fn create_deck(variant: GameVariant) -> Vec<Card> {
    let mut deck = Vec::new();

    for suit in Suit::all() {
        let values = match (variant, suit) {
            (GameVariant::Base, Suit::Mermaid) => 4..=9,
            (GameVariant::Mermaid, Suit::Mermaid) => 2..=7,
            (_, _) => 2..=7,
        };

        for value in values {
            deck.push(Card { suit, value });
        }
    }

    deck
}

pub fn create_game_deck(variant: GameVariant) -> (Vec<Card>, Vec<Card>) {
    let mut deck = create_deck(variant);
    let mut discard = Vec::new();

    for suit in Suit::all() {
        let Some(lowest_index) = deck
            .iter()
            .enumerate()
            .filter(|(_, card)| card.suit == suit)
            .min_by_key(|(_, card)| card.value)
            .map(|(index, _)| index)
        else {
            continue;
        };

        discard.push(deck.remove(lowest_index));
    }

    let mut rng = rng();
    deck.shuffle(&mut rng);
    discard.shuffle(&mut rng);

    (deck, discard)
}
