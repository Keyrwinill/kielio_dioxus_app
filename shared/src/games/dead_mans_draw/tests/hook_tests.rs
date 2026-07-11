use super::helpers::*;

use crate::games::dead_mans_draw::state::PendingAbility;

#[test]
fn hook_moves_selected_banked_card_to_play_area() {
    let mut state = GameState::empty();

    state.players[0].bank.push(Card {
        suit: Suit::Mermaid,
        value: 8,
    });

    state.phase = GamePhase::WaitingForHookTarget;

    handle_action(
        &mut state,
        GameAction::SelectHookTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[0].bank.len(), 0);
    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Mermaid);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
}

#[test]
fn hook_replay_satisfies_kraken_requirement() {
    let mut state = GameState::empty();

    state.kraken_required_cards = 1;

    state.players[0].bank.push(Card {
        suit: Suit::Mermaid,
        value: 8,
    });

    state.phase = GamePhase::WaitingForHookTarget;

    handle_action(
        &mut state,
        GameAction::SelectHookTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.kraken_required_cards, 0);
    assert_eq!(state.play_area.len(), 1);
}

#[test]
fn ai_hook_replays_safe_highest_banked_card() {
    let mut state = GameState::empty();

    state.current_player_index = 1;

    state.play_area.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Anchor,
        value: 7,
    });

    state.players[1].bank.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    crate::games::dead_mans_draw::abilities::hook::auto_resolve_hook_for_ai(&mut state);

    assert_eq!(state.players[1].bank.len(), 1);
    assert_eq!(state.play_area.len(), 2);
    assert_eq!(state.play_area[1].suit, Suit::Mermaid);
}

#[test]
fn hook_cannot_replay_non_top_card_of_suit_stack() {
    let mut state = GameState::empty();

    state.players[0].bank.push(card(Suit::Cannon, 3));
    state.players[0].bank.push(card(Suit::Cannon, 8));

    state.phase = GamePhase::WaitingForHookTarget;
    state.pending_ability = Some(PendingAbility::Hook);

    handle_action(
        &mut state,
        GameAction::SelectHookTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.players[0].bank.len(), 2);
    assert!(state.play_area.is_empty());
}

#[test]
fn hook_can_replay_top_card_of_suit_stack() {
    let mut state = GameState::empty();

    state.players[0].bank.push(card(Suit::Cannon, 3));
    state.players[0].bank.push(card(Suit::Cannon, 8));

    state.phase = GamePhase::WaitingForHookTarget;
    state.pending_ability = Some(PendingAbility::Hook);

    handle_action(
        &mut state,
        GameAction::SelectHookTarget {
            target_card_index: 1,
        },
    );

    assert_eq!(state.players[0].bank.len(), 1);
    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Cannon);
    assert_eq!(state.play_area[0].value, 8);
}
