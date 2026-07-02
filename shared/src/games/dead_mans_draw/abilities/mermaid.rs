use super::super::card::Suit;
use super::super::state::GameState;

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