use std::collections::HashSet;

use super::card::Card;
use super::card::Suit;
use super::state::GameState;

pub fn has_busted(state: &GameState) -> bool {
    let mut seen = HashSet::<Suit>::new();

    for card in &state.play_area {
        if seen.contains(&card.suit) {
            return true;
        }

        seen.insert(card.suit);
    }

    false
}

pub fn would_bust(state: &GameState, card: &Card) -> bool {
    state.play_area.iter().any(|c| c.suit == card.suit)
}

pub fn score_player_bank(state: &GameState, player_index: usize) -> u32 {
    let mut best_by_suit = std::collections::HashMap::<Suit, u8>::new();

    for card in &state.players[player_index].bank {
        best_by_suit
            .entry(card.suit)
            .and_modify(|v| *v = (*v).max(card.value))
            .or_insert(card.value);
    }

    best_by_suit.values().map(|v| *v as u32).sum()
}

pub fn winner_index(state: &GameState) -> Option<usize> {
    if !state.game_over {
        return None;
    }

    state
        .players
        .iter()
        .enumerate()
        .max_by_key(|(index, _)| score_player_bank(state, *index))
        .map(|(index, _)| index)
}
