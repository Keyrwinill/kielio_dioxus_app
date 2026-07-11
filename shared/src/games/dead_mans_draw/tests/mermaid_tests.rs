use super::helpers::*;

#[test]
fn mermaid_has_no_activated_ability() {
    let mut state = GameState::empty();

    state.deck.clear();

    // Extra card so drawing Mermaid does not end the game.
    state.deck.push(card(Suit::Oracle, 5));

    // Drawn first because Vec::pop() draws from the end.
    state.deck.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Mermaid);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert!(state.pending_ability.is_none());
    assert!(state.pending_selection.is_none());
}

#[test]
fn mermaid_scores_highest_mermaid_only() {
    let mut state = GameState::empty();

    state.players[0].bank.push(Card {
        suit: Suit::Mermaid,
        value: 4,
    });

    state.players[0].bank.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    let score = crate::games::dead_mans_draw::scoring::score_player(&state, 0);

    assert_eq!(score, 9);
}

#[test]
fn mermaid_duplicate_still_busts() {
    let mut state = GameState::empty();

    state.play_area.push(card(Suit::Mermaid, 4));
    state.deck.clear();
    state.deck.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Draw);

    assert!(state.play_area.is_empty());
    assert_eq!(state.discard.len(), 2);
}
