use super::helpers::*;

#[test]
fn hook_moves_selected_banked_card_to_play_area() {
    let mut state = GameState::new();

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
    let mut state = GameState::new();

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
    let mut state = GameState::new();

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

    crate::games::dead_mans_draw::abilities::hook::auto_resolve_hook_for_ai(
        &mut state,
    );

    assert_eq!(state.players[1].bank.len(), 1);
    assert_eq!(state.play_area.len(), 2);
    assert_eq!(state.play_area[1].suit, Suit::Mermaid);
}