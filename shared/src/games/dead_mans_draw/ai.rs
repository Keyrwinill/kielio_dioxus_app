use crate::games::dead_mans_draw::{card::Suit, state::GameState};

use super::engine::{bank_cards, draw_card, handle_action};
use crate::dto::GameAction;
use super::state::{PendingAbility};

//helpers
pub fn resolve_ai_pending_ability(state: &mut GameState) -> bool {
    match state.pending_ability {
        Some(PendingAbility::Cannon) => {
            let opponent_index = (state.current_player_index + 1) % state.players.len();

            if let Some(card_index) = best_valid_opponent_bank_card(
                state,
                opponent_index,
                |index| state.is_top_card_of_suit_stack(opponent_index, index),
            ) {
                handle_action(
                    state,
                    GameAction::SelectCannonTarget {
                        target_player_index: opponent_index,
                        target_card_index: card_index,
                    },
                );
            } else {
                skip_pending_ability(state, "AI Cannon found no valid target.");
            }

            true
        }

        Some(PendingAbility::Hook) => {
            if let Some(card_index) = best_safe_own_bank_card(state) {
                handle_action(
                    state,
                    GameAction::SelectHookTarget {
                        target_card_index: card_index,
                    },
                );
            } else {
                skip_pending_ability(state, "AI Hook found no valid target.");
            }

            true
        }

        Some(PendingAbility::Map) => {
            if let Some(card_index) =
                best_safe_card_index_from_list(&state.map_choices, state)
            {
                handle_action(
                    state,
                    GameAction::SelectMapTarget {
                        target_card_index: card_index,
                    },
                );
            } else {
                skip_pending_ability(state, "AI Map found no valid target.");
            }

            true
        }

        Some(PendingAbility::Sword) => {
            let opponent_index = (state.current_player_index + 1) % state.players.len();

            if let Some(card_index) = best_valid_opponent_bank_card(
                state,
                opponent_index,
                |index| {
                    let Some(card) = state.players[opponent_index].bank.get(index) else {
                        return false;
                    };

                    state.is_top_card_of_suit_stack(opponent_index, index)
                        && !state.player_bank_has_suit(
                            state.current_player_index,
                            card.suit,
                        )
                },
            ) {
                handle_action(
                    state,
                    GameAction::SelectSwordTarget {
                        target_player_index: opponent_index,
                        target_card_index: card_index,
                    },
                );
            } else {
                skip_pending_ability(state, "AI Sword found no valid target.");
            }

            true
        }

        None => false,
    }
}

fn skip_pending_ability(state: &mut GameState, message: &str) {
    state.add_log(message.to_string());
    state.phase = super::state::GamePhase::PlayerTurn;
    state.pending_ability = None;
    state.pending_selection = None;
}

pub fn play_ai_turn(state: &mut GameState) {
    if state.game_over || !state.current_player().is_ai {
        return;
    }

    while !state.game_over && state.current_player().is_ai {
        if resolve_ai_pending_ability(state) {
            continue;
        }

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
    
    if state.deck.is_empty() {
        return true;
    }

    let has_key = state.play_area.iter().any(|c| c.suit == Suit::Key);
    let has_chest = state.play_area.iter().any(|c| c.suit == Suit::Chest);

    if has_key && has_chest && !state.discard.is_empty() {
        return true;
    }
    
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
        .filter(|(index, card)| {
            state.is_top_card_of_suit_stack(current_player_index, *index)
                && !state
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