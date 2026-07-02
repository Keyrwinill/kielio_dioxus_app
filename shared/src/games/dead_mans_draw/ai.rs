use crate::games::dead_mans_draw::state::GameState;

use super::engine::{bank_cards, draw_card};

pub fn play_ai_turn(state: &mut GameState) {
    if state.game_over || !state.current_player().is_ai {
        return;
    }

    while !state.game_over && state.current_player().is_ai {
        if should_bank(state) {
            bank_cards(state);
            break;
        }

        draw_card(state);

        if !state.current_player().is_ai {
            break;
        }
    }
}

fn should_bank(state: &GameState) -> bool {
    if state.kraken_required_cards > 0 {
        return false;
    }

    if state.play_area.is_empty() {
        return false;
    }

    let bust_risk = estimate_bust_risk(state);

    if state.play_area.len() >= 4 {
        return true;
    }

    if bust_risk >= 0.35 {
        return true;
    }

    false
}

fn estimate_bust_risk(state: &GameState) -> f32 {
    if state.deck.is_empty() {
        return 1.0;
    }

    let current_suits: std::collections::HashSet<_> =
        state.play_area.iter().map(|card| card.suit).collect();

    let risky_cards = state
        .deck
        .iter()
        .filter(|card| current_suits.contains(&card.suit))
        .count();

    risky_cards as f32 / state.deck.len() as f32
}

pub fn best_valid_opponent_bank_card(
    state: &GameState,
    opponent_index: usize,
    validator: impl Fn(usize) -> bool,
) -> Option<usize> {
    state.players[opponent_index]
        .bank
        .iter()
        .enumerate()
        .filter(|(index, _)| validator(*index))
        .max_by_key(|(_, card)| card.value)
        .map(|(index, _)| index)
}

pub fn best_safe_own_bank_card(
    state: &GameState,
) -> Option<usize> {
    let current_player_index = state.current_player_index;

    state.players[current_player_index]
        .bank
        .iter()
        .enumerate()
        .filter(|(_, card)| {
            !state
                .play_area
                .iter()
                .any(|play_card| play_card.suit == card.suit)
        })
        .max_by_key(|(_, card)| card.value)
        .map(|(index, _)| index)
}

pub fn best_safe_card_index_from_list(
    cards: &[crate::games::dead_mans_draw::card::Card],
    state: &GameState,
) -> Option<usize> {
    cards
        .iter()
        .enumerate()
        .filter(|(_, card)| {
            !state
                .play_area
                .iter()
                .any(|play_card| play_card.suit == card.suit)
        })
        .max_by_key(|(_, card)| card.value)
        .map(|(index, _)| index)
}