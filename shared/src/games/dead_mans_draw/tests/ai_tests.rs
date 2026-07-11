use super::helpers::*;

use crate::games::dead_mans_draw::state::PendingAbility;

#[test]
fn ai_banks_when_bust_risk_is_high() {
    let mut state = GameState::empty();

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

#[test]
fn ai_hook_selects_only_top_card_of_suit_stack() {
    let mut state = GameState::empty();

    state.current_player_index = 1;

    state.players[1].bank.push(card(Suit::Cannon, 3));
    state.players[1].bank.push(card(Suit::Cannon, 8));

    state.phase = GamePhase::WaitingForHookTarget;
    state.pending_ability = Some(PendingAbility::Hook);

    crate::games::dead_mans_draw::ai::resolve_ai_pending_ability(&mut state);

    assert_eq!(state.players[1].bank.len(), 1);
    assert_eq!(state.players[1].bank[0].value, 3);

    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Cannon);
    assert_eq!(state.play_area[0].value, 8);
}

#[test]
fn ai_cannon_targets_only_top_card_of_opponent_suit_stack() {
    let mut state = GameState::empty();

    state.current_player_index = 1;

    state.players[0].bank.push(card(Suit::Sword, 3));
    state.players[0].bank.push(card(Suit::Sword, 8));

    state.phase = GamePhase::WaitingForCannonTarget;
    state.pending_ability = Some(PendingAbility::Cannon);

    crate::games::dead_mans_draw::ai::resolve_ai_pending_ability(&mut state);

    assert_eq!(state.players[0].bank.len(), 1);
    assert_eq!(state.players[0].bank[0].value, 3);

    assert_eq!(state.discard.len(), 1);
    assert_eq!(state.discard[0].suit, Suit::Sword);
    assert_eq!(state.discard[0].value, 8);

    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert!(state.pending_ability.is_none());
}

#[test]
fn ai_sword_does_not_steal_suit_it_already_has() {
    let mut state = GameState::empty();

    // AI is player 1
    state.current_player_index = 1;

    // AI already owns Cannon
    state.players[1].bank.push(card(Suit::Cannon, 2));

    // Opponent only has Cannon
    state.players[0].bank.push(card(Suit::Cannon, 8));

    state.phase = GamePhase::WaitingForSwordTarget;
    state.pending_ability = Some(PendingAbility::Sword);

    crate::games::dead_mans_draw::ai::resolve_ai_pending_ability(&mut state);

    // No legal target exists.
    assert_eq!(state.players[0].bank.len(), 1);
    assert_eq!(state.players[1].bank.len(), 1);

    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert!(state.pending_ability.is_none());
}

#[test]
fn ai_map_chooses_safe_revealed_card() {
    let mut state = GameState::empty();

    state.current_player_index = 1;

    state.play_area.push(card(Suit::Cannon, 3));

    state.map_choices.push(card(Suit::Cannon, 8)); // unsafe
    state.map_choices.push(card(Suit::Mermaid, 9)); // safe

    state.phase = GamePhase::WaitingForMapTarget;
    state.pending_ability = Some(PendingAbility::Map);

    crate::games::dead_mans_draw::ai::resolve_ai_pending_ability(&mut state);

    assert_eq!(state.play_area.len(), 2);
    assert_eq!(state.play_area[1].suit, Suit::Mermaid);
    assert_eq!(state.play_area[1].value, 9);

    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert!(state.pending_ability.is_none());
}
