use super::ability::Ability;
use super::context::AbilityContext;
use super::super::ai::best_safe_own_bank_card;
use super::super::engine::{add_card_to_play_area, resolve_bust, resolve_drawn_card_effect};
use super::super::rules::has_busted;
use super::super::state::{
    GamePhase, GameState, PendingAbility, PendingSelection,
    SelectionOwner, SelectionSource,
};

pub struct HookAbility;

impl Ability for HookAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        let current_player_index = ctx.state.current_player_index;

        if ctx.state.players[current_player_index].bank.is_empty() {
            return Some("Hook found no banked cards to replay.".to_string());
        }

        ctx.state.phase = GamePhase::WaitingForHookTarget;
        ctx.state.pending_ability = Some(PendingAbility::Hook);
        ctx.state.pending_selection = Some(PendingSelection {
            source: SelectionSource::PlayerBank {
                owner: SelectionOwner::CurrentPlayer,
            },
            prompt: "Choose one of your banked cards to replay.".to_string(),
        });

        Some("Choose one of your banked cards to replay.".to_string())
    }
}

pub fn resolve_hook(state: &mut GameState, target_card_index: usize) {
    if state.phase != GamePhase::WaitingForHookTarget {
        state.add_log("No Hook target is currently needed.");
        return;
    }

    let current_player_index = state.current_player_index;

    if target_card_index >= state.players[current_player_index].bank.len() {
        state.add_log("Invalid Hook target card.");
        return;
    }

    let card = state.players[current_player_index]
        .bank
        .remove(target_card_index);

    add_card_to_play_area(state, card.clone());

    state.phase = GamePhase::PlayerTurn;
    state.pending_ability = None;
    state.pending_selection = None;

    if has_busted(state) {
        let message = format!(
            "Hook replayed {:?} {}, but you busted. Protected cards were banked!",
            card.suit,
            card.value
        );

        resolve_bust(state, message);
        return;
    }

    let mut message = format!(
        "Hook replayed {:?} {} from your bank.",
        card.suit,
        card.value
    );

    if let Some(extra_message) = resolve_drawn_card_effect(state, &card) {
        message.push(' ');
        message.push_str(&extra_message);
    }

    state.add_log(message);
}

pub fn auto_resolve_hook_for_ai(state: &mut GameState) -> Option<String> {
    let current_player_index = state.current_player_index;

    let Some(card_index) = best_safe_own_bank_card(state) else {
        return Some("AI drew Hook, but found no safe banked card to replay.".to_string());
    };

    let card = state.players[current_player_index].bank.remove(card_index);
    state.play_area.push(card.clone());

    let mut message = format!(
        "AI Hook replayed {:?} {} from its bank.",
        card.suit,
        card.value
    );

    if let Some(extra_message) = resolve_drawn_card_effect(state, &card) {
        message.push(' ');
        message.push_str(&extra_message);
    }

    Some(message)
}