use crate::games::dead_mans_draw::engine::add_card_to_play_area;

use super::ability::Ability;
use super::context::AbilityContext;
use super::super::ai::best_valid_opponent_bank_card;
use super::super::engine::{resolve_bust, resolve_drawn_card_effect};
use super::super::rules::has_busted;
use super::super::state::{
    GamePhase, GameState, PendingAbility, PendingSelection,
    SelectionOwner, SelectionSource,
};

pub struct SwordAbility;

impl Ability for SwordAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        let opponent_index = get_opponent_index(ctx.state);

        let has_valid_target = ctx.state.players[opponent_index]
            .bank
            .iter()
            .any(|card| {
                !ctx.state.player_bank_has_suit(
                    ctx.state.current_player_index,
                    card.suit,
                )
            });

        if !has_valid_target {
            return Some("Sword found no valid opponent card to steal.".to_string());
        }

        ctx.state.phase = GamePhase::WaitingForSwordTarget;
        ctx.state.pending_ability = Some(PendingAbility::Sword);
        ctx.state.pending_selection = Some(PendingSelection {
            source: SelectionSource::PlayerBank {
                owner: SelectionOwner::Opponent,
            },
            prompt: "Choose one valid opponent banked card for Sword.".to_string(),
        });

        Some("Choose one valid opponent banked card for Sword.".to_string())
    }
}

pub fn resolve_sword(
    state: &mut GameState,
    target_player_index: usize,
    target_card_index: usize,
) {
    if state.phase != GamePhase::WaitingForSwordTarget {
        state.add_log("No Sword target is currently needed.");
        return;
    }

    let opponent_index = get_opponent_index(state);

    if target_player_index != opponent_index {
        state.add_log("Invalid Sword target player.");
        return;
    }

    if target_card_index >= state.players[target_player_index].bank.len() {
        state.add_log("Invalid Sword target card.");
        return;
    }

    let target_card = state.players[target_player_index].bank[target_card_index].clone();

    if state.player_bank_has_suit(state.current_player_index, target_card.suit) {
        state.add_log("Invalid Sword target: you already have that suit in your bank.");
        return;
    }

    let stolen = state.players[target_player_index]
        .bank
        .remove(target_card_index);

    add_card_to_play_area(state, stolen.clone());

    state.phase = GamePhase::PlayerTurn;
    state.pending_ability = None;
    state.pending_selection = None;

    if has_busted(state) {
        let message = format!(
            "Sword stole {:?} {}, but you busted. Protected cards were banked.",
            stolen.suit,
            stolen.value
        );

        resolve_bust(state, message);
        return;
    }

    let mut message = format!(
        "Sword stole {:?} {} into the play area.",
        stolen.suit,
        stolen.value
    );

    if let Some(extra_message) = resolve_drawn_card_effect(state, &stolen) {
        message.push(' ');
        message.push_str(&extra_message);
    }

    state.add_log(message);
}

pub fn auto_resolve_sword_for_ai(state: &mut GameState) -> Option<String> {
    let opponent_index = get_opponent_index(state);

    let Some(card_index) =
        best_valid_opponent_bank_card(state, opponent_index, |index| {
            let card = &state.players[opponent_index].bank[index];

            !state.player_bank_has_suit(
                state.current_player_index,
                card.suit,
            )
        })
    else {
        return Some("AI drew Sword, but found no valid target.".to_string());
    };

    let stolen = state.players[opponent_index].bank.remove(card_index);
    state.play_area.push(stolen.clone());

    if has_busted(state) {
        let message = format!(
            "AI Sword stole {:?} {}, but busted. Protected cards were banked.",
            stolen.suit,
            stolen.value
        );

        resolve_bust(state, message);
        return None;
    }

    let mut message = format!(
        "AI Sword stole {:?} {} into the play area.",
        stolen.suit,
        stolen.value
    );

    if let Some(extra_message) = resolve_drawn_card_effect(state, &stolen) {
        message.push(' ');
        message.push_str(&extra_message);
    }

    Some(message)
}

fn get_opponent_index(state: &GameState) -> usize {
    (state.current_player_index + 1) % state.players.len()
}