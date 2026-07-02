use super::helpers::*;

#[test]
fn cannon_removes_selected_opponent_card() {
    let mut state = GameState::new();

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
    let mut state = GameState::new();

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
    let mut state = GameState::new();

    state.current_player_index = 1;

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.players[0].bank.push(Card {
        suit: Suit::Anchor,
        value: 7,
    });

    crate::games::dead_mans_draw::abilities::cannon::auto_resolve_cannon_for_ai(
        &mut state,
    );

    assert_eq!(state.players[0].bank.len(), 1);
    assert_eq!(state.players[0].bank[0].value, 2);
    assert_eq!(state.discard.len(), 1);
    assert_eq!(state.discard[0].value, 7);
}