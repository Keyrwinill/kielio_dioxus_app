use crate::dto::GameAction;
use super::card::Suit;
use super::rules::{has_busted, would_bust};
use super::state::{GamePhase, GameState};
use super::abilities::{
    registry::execute_card_ability,
    cannon::resolve_cannon,
    hook::resolve_hook,
    map::resolve_map,
    sword::resolve_sword,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawMode {
    Normal,
    ForcedNoAbility,
}

#[derive(Debug, Clone)]
pub enum ReplayResult {
    Busted,
    Continued {
        extra_message: Option<String>,
    },
}

pub fn draw_card(state: &mut GameState) {
    draw_card_internal(state, DrawMode::Normal);
}

pub fn draw_card_internal(state: &mut GameState, mode: DrawMode) {
    if state.game_over {
        return;
    }

    if state.phase != GamePhase::PlayerTurn {
        state.add_log("You must finish the pending ability first.");
        return;
    }

    state.revealed_next_card = None;

    let Some(card) = state.deck.pop() else {
        end_game(state);
        return;
    };

    if would_bust(state, &card) {
        state.discard.push(card.clone());

        let message = format!(
            "{} busted by drawing {:?} {}.",
            state.current_player().name,
            card.suit,
            card.value
        );

        resolve_bust(state, message);
        return;
    }

    add_card_to_play_area(state, card.clone());

    let mut message = format!(
        "{} drew {:?} {}.",
        state.current_player().name,
        card.suit,
        card.value
    );

    if mode == DrawMode::Normal {
        if let Some(extra_message) = execute_card_ability(state, &card) {
            message.push(' ');
            message.push_str(&extra_message);
        }
    }

    state.add_log(message);

    //To be checked
    if state.deck.is_empty() && state.kraken_required_cards > 0 {
        state.add_log(
            "Deck ended before Kraken could be completed.".to_string()
        );

        state.kraken_required_cards = 0;
        end_game(state);
        return;
    }
}

pub fn handle_action(state: &mut GameState, action: GameAction) {
    match action {
        GameAction::Draw => draw_card(state),

        GameAction::Bank => bank_cards(state),

        GameAction::AiTurn => {
            crate::games::dead_mans_draw::ai::play_ai_turn(state);
        }

        GameAction::NewGame => {
            *state = GameState::new();
        }

        GameAction::StartNewGame { config } => {
            *state = GameState::new_with_config(config);
        }

        GameAction::SelectCannonTarget {
            target_player_index,
            target_card_index,
        } => {
            resolve_cannon(state, target_player_index, target_card_index);
        }

        GameAction::SelectHookTarget { target_card_index } => {
            resolve_hook(state, target_card_index);
        }

        GameAction::SelectMapTarget { target_card_index } => {
            resolve_map(state, target_card_index);
        }

        GameAction::SelectSwordTarget {
            target_player_index,
            target_card_index,
        } => {
            resolve_sword(state, target_player_index, target_card_index);
        }
    }
}

pub fn bank_cards(state: &mut GameState) {
    if state.game_over {
        return;
    }

    if state.phase != GamePhase::PlayerTurn {
        state.add_log("You must finish the pending ability first.");
        return;
    }

    if state.play_area.is_empty() {
        state.add_log("There are no cards to bank.");
        return;
    }

    if state.kraken_required_cards > 0 {
        state.add_log(format!(
            "Kraken requires {} more card(s) before you can bank.",
            state.kraken_required_cards
        ));
        return;
    }

    let cards = std::mem::take(&mut state.play_area);
    let collected_count = cards.len();

    let has_chest = cards.iter().any(|c| c.suit == Suit::Chest);
    let has_key = cards.iter().any(|c| c.suit == Suit::Key);

    state.current_player_mut().bank.extend(cards);

    let mut bonus_count = 0;

    if has_chest && has_key {
        for _ in 0..collected_count {
            if let Some(card) = state.discard.pop() {
                state.current_player_mut().bank.push(card);
                bonus_count += 1;
            }
        }
    }

    state.map_choices.clear();
    state.anchor_index = None;

    if bonus_count > 0 {
        state.add_log(format!(
            "{} banked cards and claimed {} bonus card(s) with Key + Chest.",
            state.current_player().name,
            bonus_count,
        ));
    } else {
        state.add_log(format!(
            "{} banked cards.",
            state.current_player().name,
        ));
    }

    if state.deck.is_empty() {
        end_game(state);
    } else {
        state.next_player();
    }
}

pub fn resolve_bust(state: &mut GameState, message: String) {
    if let Some(anchor_index) = state.anchor_index {
        let mut all_cards = std::mem::take(&mut state.play_area);

        let discarded_cards = all_cards.split_off(anchor_index);
        let protected_cards = all_cards;

        state.current_player_mut().bank.extend(protected_cards);
        state.discard.extend(discarded_cards);
    } else {
        state.discard.append(&mut state.play_area);
    }

    state.map_choices.clear();
    state.anchor_index = None;
    state.pending_selection = None;
    state.pending_ability = None;
    state.kraken_required_cards = 0;

    state.add_log(message);

    if state.deck.is_empty() {
        state.game_over = true;
        state.phase = GamePhase::GameOver;
    } else {
        state.next_player();
    }
}

pub fn end_game(state: &mut GameState) {
    state.game_over = true;
    state.phase = GamePhase::GameOver;
    state.add_log("Deck is empty. Game over.".to_string());
}

pub fn resolve_drawn_card_effect(
    state: &mut GameState,
    card: &crate::games::dead_mans_draw::card::Card,
) -> Option<String> {
    execute_card_ability(state, card)
}

pub fn replay_card_to_play_area(
    state: &mut GameState,
    card: crate::games::dead_mans_draw::card::Card,
) -> ReplayResult {
    add_card_to_play_area(state, card.clone());

    if has_busted(state) {
        return ReplayResult::Busted;
    }

    let extra_message = resolve_drawn_card_effect(state, &card);

    ReplayResult::Continued {
        extra_message,
    }
}

pub fn add_card_to_play_area(
    state: &mut GameState,
    card: crate::games::dead_mans_draw::card::Card,
) {
    state.play_area.push(card);

    if state.kraken_required_cards > 0 {
        state.kraken_required_cards -= 1;
    }
}

pub fn append_extra_message(message: &mut String, extra_message: Option<String>) {
    if let Some(extra_message) = extra_message {
        message.push(' ');
        message.push_str(&extra_message);
    }
}

pub fn finish_pending_selection(state: &mut GameState) {
    state.phase = GamePhase::PlayerTurn;
    state.pending_ability = None;
    state.pending_selection = None;
}