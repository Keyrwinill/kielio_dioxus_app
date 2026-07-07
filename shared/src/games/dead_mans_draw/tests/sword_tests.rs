use super::helpers::*;

use crate::games::dead_mans_draw::state::PendingAbility;
use crate::games::dead_mans_draw::abilities::{
    ability::Ability,
    context::AbilityContext,
    sword::SwordAbility,
};

#[test]
fn sword_steals_selected_opponent_card_to_play_area() {
    let mut state = GameState::empty();

    state.players[1].bank.push(Card {
        suit: Suit::Anchor,
        value: 6,
    });

    state.phase = GamePhase::WaitingForSwordTarget;

    handle_action(
        &mut state,
        GameAction::SelectSwordTarget {
            target_player_index: 1,
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[1].bank.len(), 0);
    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Anchor);
    assert_eq!(state.discard.len(), 0);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
}

#[test]
fn sword_cannot_steal_suit_already_in_own_bank() {
    let mut state = GameState::empty();

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Anchor,
        value: 6,
    });

    state.phase = GamePhase::WaitingForSwordTarget;

    handle_action(
        &mut state,
        GameAction::SelectSwordTarget {
            target_player_index: 1,
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[1].bank.len(), 1);
    assert_eq!(state.play_area.len(), 0);
    assert_eq!(state.phase, GamePhase::WaitingForSwordTarget);
}

#[test]
fn ai_sword_steals_valid_card_to_play_area() {
    let mut state = GameState::empty();

    state.current_player_index = 1;

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 6,
    });

    crate::games::dead_mans_draw::abilities::sword::auto_resolve_sword_for_ai(
        &mut state,
    );

    assert_eq!(state.players[0].bank.len(), 0);
    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Anchor);
}

#[test]
fn sword_cannot_steal_non_top_card_of_suit_stack() {
    let mut state = GameState::empty();

    state.players[1].bank.push(card(Suit::Cannon, 3));
    state.players[1].bank.push(card(Suit::Cannon, 8));

    state.phase = GamePhase::WaitingForSwordTarget;
    state.pending_ability = Some(PendingAbility::Sword);

    handle_action(
        &mut state,
        GameAction::SelectSwordTarget {
            target_player_index: 1,
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[1].bank.len(), 2);
    assert!(state.play_area.is_empty());
}

#[test]
fn sword_can_steal_top_card_of_suit_stack() {
    let mut state = GameState::empty();

    state.players[1].bank.push(card(Suit::Cannon, 3));
    state.players[1].bank.push(card(Suit::Cannon, 8));

    state.phase = GamePhase::WaitingForSwordTarget;
    state.pending_ability = Some(PendingAbility::Sword);

    handle_action(
        &mut state,
        GameAction::SelectSwordTarget {
            target_player_index: 1,
            target_card_index: 1,
        },
    );

    assert_eq!(state.players[1].bank.len(), 1);
    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Cannon);
    assert_eq!(state.play_area[0].value, 8);
}

#[test]
fn sword_does_not_enter_target_state_when_no_valid_target_exists() {
    let mut state = GameState::empty();

    state.players[0].bank.push(card(Suit::Cannon, 3));
    state.players[1].bank.push(card(Suit::Cannon, 8));

    let message = SwordAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Sword, 5),
    });

    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert!(state.pending_ability.is_none());
    assert!(state.pending_selection.is_none());
    assert_eq!(
        message.unwrap(),
        "Sword found no valid opponent card to steal."
    );
}