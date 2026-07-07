use super::ability::Ability;
use super::context::AbilityContext;
use super::super::ai::best_valid_opponent_bank_card;
use super::super::state::{
    GamePhase, GameState, PendingAbility, PendingSelection,
    SelectionOwner, SelectionSource,
};

pub struct CannonAbility;

impl Ability for CannonAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        if !ctx.state.has_any_opponent_bank_cards() {
            return Some("Cannon found no opponent card to discard.".to_string());
        }

        ctx.state.phase = GamePhase::WaitingForCannonTarget;
        ctx.state.pending_ability = Some(PendingAbility::Cannon);
        ctx.state.pending_selection = Some(PendingSelection {
            source: SelectionSource::PlayerBank {
                owner: SelectionOwner::Opponent,
            },
            prompt: "Choose one opponent banked card to destroy.".to_string(),
        });

        Some("Choose one opponent banked card to destroy.".to_string())
    }
}

pub fn resolve_cannon(
    state: &mut GameState,
    target_player_index: usize,
    target_card_index: usize,
) {
    if state.phase != GamePhase::WaitingForCannonTarget {
        state.add_log("No Cannon target is currently needed.".to_string());
        return;
    }

    if target_player_index == state.current_player_index {
        state.add_log("Invalid target player.".to_string());
        return;
    }

    if target_card_index >= state.players[target_player_index].bank.len() {
        state.add_log("Invalid target card.".to_string());
        return;
    }

    if !state.is_top_card_of_suit_stack(target_player_index, target_card_index) {
        state.add_log("Invalid Cannon target: choose the top card of a suit stack.");
        return;
    }

    let removed = state.players[target_player_index]
        .bank
        .remove(target_card_index);

    state.discard.push(removed.clone());

    state.phase = GamePhase::PlayerTurn;
    state.pending_ability = None;

    state.add_log(format!(
        "Cannon destroyed {:?} {}.",
        removed.suit,
        removed.value
    ));
    
    state.pending_selection = None;
}

pub fn auto_resolve_cannon_for_ai(state: &mut GameState) -> Option<String> {
    for opponent_index in state.opponent_indices() {
        if let Some(card_index) = best_valid_opponent_bank_card(
            state,
            opponent_index,
            |index| state.is_top_card_of_suit_stack(opponent_index, index),
        ) {
            let removed = state.players[opponent_index].bank.remove(card_index);
            state.discard.push(removed.clone());

            return Some(format!(
                "AI Cannon destroyed {:?} {}.",
                removed.suit,
                removed.value
            ));
        }
    }

    Some("AI Cannon found no valid target.".to_string())
}
