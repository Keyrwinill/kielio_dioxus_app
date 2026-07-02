use super::helpers::*;

#[test]
fn ai_banks_when_bust_risk_is_high() {
    let mut state = GameState::new();

    state.current_player_index = 1;

    state.play_area.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.deck.clear();

    state.deck.push(Card {
        suit: Suit::Anchor,
        value: 5,
    });

    state.deck.push(Card {
        suit: Suit::Oracle,
        value: 5,
    });

    crate::games::dead_mans_draw::ai::play_ai_turn(&mut state);

    assert_eq!(state.players[1].bank.len(), 1);
    assert_eq!(state.play_area.len(), 0);
    assert_eq!(state.current_player_index, 0);
}