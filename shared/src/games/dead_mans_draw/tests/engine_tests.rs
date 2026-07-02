use super::helpers::*;

#[test]
fn drawing_last_card_does_not_end_game_before_banking() {
    let mut state = GameState::new();

    state.deck.clear();
    state.deck.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert_eq!(state.play_area.len(), 1);

    handle_action(&mut state, GameAction::Bank);

    assert!(state.game_over);
    assert_eq!(state.players[0].bank.len(), 1);
}