use super::helpers::*;

use crate::games::dead_mans_draw::engine::bank_cards;
use crate::games::dead_mans_draw::state::PendingAbility;

use crate::games::dead_mans_draw::{
    abilities::cannon::resolve_cannon, player::Player, state::GameState,
};

#[test]
fn cannon_removes_selected_opponent_card() {
    let mut state = GameState::empty();

    state.players[1].bank.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    state.phase = GamePhase::WaitingForCannonTarget;

    handle_action(
        &mut state,
        GameAction::SelectCannonTarget {
            target_player_index: 1,
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[1].bank.len(), 0);
    assert_eq!(state.discard.len(), 1);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
}

#[test]
fn cannon_cannot_remove_non_top_card_of_suit_stack() {
    let mut state = GameState::empty();

    state.players[1].bank.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Anchor,
        value: 7,
    });

    state.phase = GamePhase::WaitingForCannonTarget;

    handle_action(
        &mut state,
        GameAction::SelectCannonTarget {
            target_player_index: 1,
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[1].bank.len(), 2);
    assert_eq!(state.discard.len(), 0);
    assert_eq!(state.phase, GamePhase::WaitingForCannonTarget);
}

#[test]
fn ai_cannon_targets_only_top_suit_stack_card() {
    let mut state = GameState::empty();

    state.current_player_index = 1;

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 7,
    });

    crate::games::dead_mans_draw::abilities::cannon::auto_resolve_cannon_for_ai(&mut state);

    assert_eq!(state.players[0].bank.len(), 1);
    assert_eq!(state.players[0].bank[0].value, 2);
    assert_eq!(state.discard.len(), 1);
    assert_eq!(state.discard[0].value, 7);
}

#[test]
fn cannon_cannot_target_non_top_card_even_if_called_directly() {
    let mut state = GameState::empty();

    state.players[1].bank.push(card(Suit::Sword, 3));
    state.players[1].bank.push(card(Suit::Sword, 8));

    state.phase = GamePhase::WaitingForCannonTarget;
    state.pending_ability = Some(PendingAbility::Cannon);

    handle_action(
        &mut state,
        GameAction::SelectCannonTarget {
            target_player_index: 1,
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[1].bank.len(), 2);
}

#[test]
fn cannon_can_destroy_top_card_from_any_opponent_in_multiplayer_game() {
    let mut state = GameState::empty();
    state.players = vec![
        Player::new("P1", false),
        Player::new("P2", false),
        Player::new("P3", true),
    ];

    state.current_player_index = 0;
    state.phase = GamePhase::WaitingForCannonTarget;
    state.pending_ability = Some(PendingAbility::Cannon);

    state.players[2].bank.push(card(Suit::Sword, 4));
    state.players[2].bank.push(card(Suit::Sword, 9));

    resolve_cannon(&mut state, 2, 1);

    assert_eq!(state.players[2].bank.len(), 1);
    assert_eq!(state.players[2].bank[0].value, 4);

    assert_eq!(state.discard.len(), 1);
    assert_eq!(state.discard[0].suit, Suit::Sword);
    assert_eq!(state.discard[0].value, 9);

    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert!(state.pending_ability.is_none());
}

#[test]
fn can_bank_after_resolving_cannon_drawn_as_last_card() {
    let mut state = GameState::empty();

    state.deck.clear();

    state.play_area = vec![Card {
        suit: Suit::Cannon,
        value: 5,
    }];

    state.players[1].bank = vec![Card {
        suit: Suit::Anchor,
        value: 4,
    }];

    state.phase = GamePhase::WaitingForCannonTarget;
    state.pending_ability = Some(PendingAbility::Cannon);

    resolve_cannon(&mut state, 1, 0);

    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert!(state.pending_ability.is_none());
    assert!(state.pending_selection.is_none());

    bank_cards(&mut state);

    assert!(state.game_over);
}
