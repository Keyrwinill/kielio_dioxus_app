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
        let opponent_index = get_opponent_index(ctx.state);

        if ctx.state.players[opponent_index].bank.is_empty() {
            return Some("Cannon fired, but opponent has no banked cards.".to_string());
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

    let opponent_index = get_opponent_index(state);

    if target_player_index != opponent_index {
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

fn get_opponent_index(state: &GameState) -> usize {
    (state.current_player_index + 1) % state.players.len()
}

pub fn auto_resolve_cannon_for_ai(state: &mut GameState) -> Option<String> {
    let opponent_index = get_opponent_index(state);

    let Some(card_index) =
        best_valid_opponent_bank_card(state, opponent_index, |index| {
            state.is_top_card_of_suit_stack(opponent_index, index)
        })
    else {
        return Some("Cannon fired, but opponent has no valid banked cards.".to_string());
    };

    let removed = state.players[opponent_index].bank.remove(card_index);
    state.discard.push(removed.clone());

    Some(format!(
        "AI Cannon destroyed {:?} {}.",
        removed.suit,
        removed.value
    ))
}