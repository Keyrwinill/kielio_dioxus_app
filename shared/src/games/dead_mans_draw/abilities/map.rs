use rand::seq::SliceRandom;
use rand::rng;

use crate::games::dead_mans_draw::engine::{ReplayResult, add_card_to_play_area, append_extra_message, finish_pending_selection, replay_card_to_play_area};

use super::ability::Ability;
use super::context::AbilityContext;
use super::super::ai::best_safe_card_index_from_list;
use super::super::engine::{resolve_bust, resolve_drawn_card_effect};
use super::super::rules::has_busted;
use super::super::state::{
    GamePhase, 
    GameState, 
    PendingAbility, 
    PendingSelection,
    SelectionSource,
};

pub struct MapAbility;

impl Ability for MapAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        if ctx.state.discard.is_empty() {
            return Some("Map found no cards in discard.".to_string());
        }

        ctx.state.map_choices.clear();

        let mut rng = rng();
        ctx.state.discard.shuffle(&mut rng);

        for _ in 0..3 {
            if let Some(card) = ctx.state.discard.pop() {
                ctx.state.map_choices.push(card);
            }
        }

        ctx.state.phase = GamePhase::WaitingForMapTarget;
        ctx.state.pending_ability = Some(PendingAbility::Map);
        ctx.state.pending_selection = Some(PendingSelection {
            source: SelectionSource::MapChoices,
            prompt: "Choose one revealed Map card to replay.".to_string(),
        });

        Some("Map shuffled discard and revealed cards.".to_string())
    }
}

pub fn resolve_map(state: &mut GameState, target_card_index: usize) {
    if state.phase != GamePhase::WaitingForMapTarget {
        state.add_log("No Map target is currently needed.");
        return;
    }

    if target_card_index >= state.map_choices.len() {
        state.add_log("Invalid Map target card.");
        return;
    }

    let chosen = state.map_choices.remove(target_card_index);

    while let Some(card) = state.map_choices.pop() {
        state.discard.push(card);
    }

    finish_pending_selection(state);

    match replay_card_to_play_area(state, chosen.clone()) {
        ReplayResult::Busted => {
            let message = format!(
                "Map replayed {:?} {}, but you busted. Protected cards were banked.",
                chosen.suit,
                chosen.value
            );

            resolve_bust(state, message);
            return;
        }

        ReplayResult::Continued { extra_message } => {
            let mut message = format!(
                "Map replayed {:?} {}.",
                chosen.suit,
                chosen.value
            );

            append_extra_message(&mut message, extra_message);

            state.add_log(message);
        }
    }
}

pub fn auto_resolve_map_for_ai(state: &mut GameState) -> Option<String> {
    if state.discard.is_empty() {
        return Some("AI drew Map, but discard is empty.".to_string());
    }

    state.map_choices.clear();

    for _ in 0..3 {
        if let Some(card) = state.discard.pop() {
            state.map_choices.push(card);
        }
    }

    let Some(choice_index) =
        best_safe_card_index_from_list(&state.map_choices, state)
    else {
        while let Some(card) = state.map_choices.pop() {
            state.discard.push(card);
        }

        return Some("AI drew Map, but found no safe revealed card.".to_string());
    };

    let chosen = state.map_choices.remove(choice_index);

    while let Some(card) = state.map_choices.pop() {
        state.discard.push(card);
    }

    add_card_to_play_area(state, chosen.clone());

    let mut message = format!(
        "AI Map replayed {:?} {} from discard.",
        chosen.suit,
        chosen.value
    );

    if let Some(extra_message) = resolve_drawn_card_effect(state, &chosen) {
        message.push(' ');
        message.push_str(&extra_message);
    }

    Some(message)
}