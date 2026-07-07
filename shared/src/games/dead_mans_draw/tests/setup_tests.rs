use crate::games::dead_mans_draw::{card::Suit, state::GameState};

#[test]
fn new_game_starts_with_initial_discard_pile() {
    let state = GameState::new();

    assert_eq!(state.deck.len(), 50);
    assert_eq!(state.discard.len(), 10);

    for suit in Suit::all() {
        assert_eq!(
            state.discard.iter().filter(|card| card.suit == suit).count(),
            1
        );
    }
}
