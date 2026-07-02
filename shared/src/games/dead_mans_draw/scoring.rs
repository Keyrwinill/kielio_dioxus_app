use std::collections::HashMap;

use super::abilities::mermaid::MermaidAbility;
use super::card::Suit;
use super::state::GameState;

pub fn score_player(state: &GameState, player_index: usize) -> u32 {
    let mut best_by_suit = HashMap::<Suit, u8>::new();

    for card in &state.players[player_index].bank {
        if card.suit == Suit::Mermaid {
            continue;
        }

        best_by_suit
            .entry(card.suit)
            .and_modify(|value| *value = (*value).max(card.value))
            .or_insert(card.value);
    }

    let normal_score: u32 = best_by_suit.values().map(|value| *value as u32).sum();
    let mermaid_score = MermaidAbility::score(state, player_index);

    normal_score + mermaid_score
}

pub fn winner_index(state: &GameState) -> Option<usize> {
    if !state.game_over {
        return None;
    }

    state
        .players
        .iter()
        .enumerate()
        .max_by_key(|(index, _)| score_player(state, *index))
        .map(|(index, _)| index)
}