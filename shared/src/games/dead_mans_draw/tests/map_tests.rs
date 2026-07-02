use super::helpers::*;

#[test]
fn map_moves_selected_map_choice_to_play_area() {
    let mut state = GameState::new();

    state.map_choices = vec![
        Card {
            suit: Suit::Anchor,
            value: 2,
        },
        Card {
            suit: Suit::Oracle,
            value: 5,
        },
    ];

    state.phase = GamePhase::WaitingForMapTarget;

    handle_action(
        &mut state,
        GameAction::SelectMapTarget {
            target_card_index: 1,
        },
    );

    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Oracle);
    assert_eq!(state.map_choices.len(), 0);
    assert_eq!(state.discard.len(), 1);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
}

#[test]
fn map_replay_satisfies_kraken_requirement() {
    let mut state = GameState::new();

    state.kraken_required_cards = 1;

    state.map_choices.push(Card {
        suit: Suit::Oracle,
        value: 5,
    });

    state.phase = GamePhase::WaitingForMapTarget;

    handle_action(
        &mut state,
        GameAction::SelectMapTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.kraken_required_cards, 0);
    assert_eq!(state.play_area.len(), 1);
}

#[test]
fn ai_map_replays_safe_highest_revealed_discard_card() {
    let mut state = GameState::new();

    state.current_player_index = 1;

    state.play_area.push(Card {
        suit: Suit::Anchor,
        value: 2,
    });

    state.discard.clear();

    state.discard.push(Card {
        suit: Suit::Oracle,
        value: 5,
    });

    state.discard.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    state.discard.push(Card {
        suit: Suit::Anchor,
        value: 7,
    });

    crate::games::dead_mans_draw::abilities::map::auto_resolve_map_for_ai(
        &mut state,
    );

    assert_eq!(state.play_area.len(), 2);
    assert_eq!(state.play_area[1].suit, Suit::Mermaid);
}