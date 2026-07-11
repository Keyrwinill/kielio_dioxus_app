use crate::games::dead_mans_draw::state::{GamePhase, PendingAbility};

use super::super::card::Suit;
use super::super::state::GameState;

pub fn resolve_mermaid(state: &mut GameState, play_area_index: usize) -> Result<(), String> {
    if state.pending_ability != Some(PendingAbility::Mermaid) {
        return Err("No Mermaid ability is pending.".to_string());
    }

    if state.phase != GamePhase::WaitingForMermaidTarget {
        return Err("Not waiting for Mermaid target.".to_string());
    }

    if play_area_index >= state.play_area.len() {
        return Err("Invalid Mermaid target.".to_string());
    }

    let target = state.play_area.remove(play_area_index);

    let mermaid_index = state
        .play_area
        .iter()
        .position(|card| card.suit == Suit::Mermaid)
        .ok_or_else(|| "Mermaid card not found in play area.".to_string())?;

    let insert_index = mermaid_index + 1;
    state.play_area.insert(insert_index, target.clone());

    state.pending_ability = None;
    state.pending_selection = None;
    state.phase = GamePhase::PlayerTurn;

    state.add_log(format!(
        "Mermaid moved {:?} {} next to Mermaid.",
        target.suit, target.value
    ));

    if let Some(extra_message) =
        crate::games::dead_mans_draw::abilities::registry::execute_card_ability(state, &target)
    {
        state.add_log(extra_message);
    } else {
        state.add_log(format!(
            "{:?} {} had no visible effect.",
            target.suit, target.value
        ));
    }

    Ok(())
}

pub struct MermaidAbility;

impl MermaidAbility {
    pub fn score(state: &GameState, player_index: usize) -> u32 {
        state.players[player_index]
            .bank
            .iter()
            .filter(|card| card.suit == Suit::Mermaid)
            .map(|card| card.value as u32)
            .max()
            .unwrap_or(0)
    }
}
