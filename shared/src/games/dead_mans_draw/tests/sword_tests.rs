use super::helpers::*;

#[test]
fn sword_steals_selected_opponent_card_to_play_area() {
    let mut state = GameState::new();

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
    let mut state = GameState::new();

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
    let mut state = GameState::new();

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