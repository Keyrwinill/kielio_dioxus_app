pub use crate::dto::GameAction;
pub use crate::games::dead_mans_draw::card::{Card, Suit};
pub use crate::games::dead_mans_draw::engine::{handle_action, resolve_bust};
pub use crate::games::dead_mans_draw::state::{GamePhase, GameState};

pub fn card(suit: Suit, value: u8) -> Card {
    Card { suit, value }
}

pub fn setup_play_area(
    state: &mut GameState,
    cards: Vec<Card>,
    anchor_index: Option<usize>,
) {
    state.play_area = cards;
    state.anchor_index = anchor_index;
}