use super::helpers::*;

use crate::games::dead_mans_draw::state::PendingAbility;

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

#[test]
fn drawing_last_card_that_busts_ends_game_after_bust_resolution() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![card(Suit::Cannon, 3)],
        None,
    );

    state.deck.clear();
    state.deck.push(card(Suit::Cannon, 8));

    handle_action(&mut state, GameAction::Draw);

    assert!(state.game_over);
    assert_eq!(state.phase, GamePhase::GameOver);

    assert!(state.play_area.is_empty());
    assert_eq!(state.discard.len(), 2);
    assert!(state.discard.iter().any(|c| c.suit == Suit::Cannon && c.value == 3));
    assert!(state.discard.iter().any(|c| c.suit == Suit::Cannon && c.value == 8));
}

#[test]
fn drawing_last_map_card_waits_for_map_selection_before_game_over() {
    let mut state = GameState::new();

    state.discard.push(card(Suit::Oracle, 5));

    state.deck.clear();
    state.deck.push(card(Suit::Map, 7));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::WaitingForMapTarget);
    assert_eq!(state.pending_ability, Some(PendingAbility::Map));
}

#[test]
fn drawing_last_hook_card_waits_for_hook_selection_before_game_over() {
    let mut state = GameState::new();

    state.players[0].bank.push(card(Suit::Oracle, 5));

    state.deck.clear();
    state.deck.push(card(Suit::Hook, 7));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::WaitingForHookTarget);
    assert_eq!(state.pending_ability, Some(PendingAbility::Hook));
}

#[test]
fn drawing_last_cannon_card_waits_for_cannon_selection_before_game_over() {
    let mut state = GameState::new();

    state.players[1].bank.push(card(Suit::Mermaid, 9));

    state.deck.clear();
    state.deck.push(card(Suit::Cannon, 7));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::WaitingForCannonTarget);
    assert_eq!(state.pending_ability, Some(PendingAbility::Cannon));
}

#[test]
fn drawing_last_sword_card_waits_for_sword_selection_before_game_over() {
    let mut state = GameState::new();

    state.players[1].bank.push(card(Suit::Mermaid, 9));

    state.deck.clear();
    state.deck.push(card(Suit::Sword, 7));

    handle_action(&mut state, GameAction::Draw);

    assert!(!state.game_over);
    assert_eq!(state.phase, GamePhase::WaitingForSwordTarget);
    assert_eq!(state.pending_ability, Some(PendingAbility::Sword));
}