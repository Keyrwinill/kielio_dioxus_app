use crate::{
    dto::GameAction,
    games::dead_mans_draw::{
        card::{Card, Suit},
        engine::handle_action,
        state::{GameConfig, GamePhase, GameState, PendingAbility},
        variant::GameVariant,
    },
};

#[test]
fn mermaid_variant_uses_low_value_mermaids() {
    let config = GameConfig {
        players: GameConfig::default().players,
        variant: GameVariant::Mermaid,
    };

    let state = GameState::new_with_config(config);

    let mut values = state
        .deck
        .iter()
        .chain(state.discard.iter())
        .filter(|card| card.suit == Suit::Mermaid)
        .map(|card| card.value)
        .collect::<Vec<_>>();

    values.sort();

    assert_eq!(values, vec![2, 3, 4, 5, 6, 7]);
}

#[test]
fn base_game_uses_high_value_mermaids() {
    let state = GameState::new();

    let mut values = state
        .deck
        .iter()
        .chain(state.discard.iter())
        .filter(|card| card.suit == Suit::Mermaid)
        .map(|card| card.value)
        .collect::<Vec<_>>();

    values.sort();

    assert_eq!(values, vec![4, 5, 6, 7, 8, 9]);
}

#[test]
fn mermaid_enters_waiting_for_target() {
    let mut state = GameState::empty();
    state.variant = GameVariant::Mermaid;

    state.play_area = vec![
        Card {
            suit: Suit::Anchor,
            value: 5,
        },
        Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    ];

    state.phase = GamePhase::PlayerTurn;

    crate::games::dead_mans_draw::abilities::registry::execute_card_ability(
        &mut state,
        &Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    );

    assert_eq!(state.pending_ability, Some(PendingAbility::Mermaid));
    assert_eq!(state.phase, GamePhase::WaitingForMermaidTarget);
}

#[test]
fn mermaid_resolves_selected_play_area_target() {
    let mut state = GameState::empty();
    state.variant = GameVariant::Mermaid;

    state.play_area = vec![
        Card {
            suit: Suit::Anchor,
            value: 5,
        },
        Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    ];

    state.phase = GamePhase::PlayerTurn;

    crate::games::dead_mans_draw::abilities::registry::execute_card_ability(
        &mut state,
        &Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    );

    handle_action(
        &mut state,
        GameAction::SelectMermaidTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.pending_ability, None);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
    assert!(state.pending_selection.is_none());
}

#[test]
fn mermaid_cannot_target_itself() {
    let mut state = GameState::empty();
    state.variant = GameVariant::Mermaid;

    state.play_area = vec![
        Card {
            suit: Suit::Anchor,
            value: 5,
        },
        Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    ];

    state.phase = GamePhase::PlayerTurn;

    crate::games::dead_mans_draw::abilities::registry::execute_card_ability(
        &mut state,
        &Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    );

    handle_action(
        &mut state,
        GameAction::SelectMermaidTarget {
            target_card_index: 1,
        },
    );

    assert_eq!(state.pending_ability, Some(PendingAbility::Mermaid));
    assert_eq!(state.phase, GamePhase::WaitingForMermaidTarget);
}

#[test]
fn mermaid_moves_selected_card_to_right_of_mermaid() {
    let mut state = GameState::empty();
    state.variant = GameVariant::Mermaid;

    state.play_area = vec![
        Card {
            suit: Suit::Anchor,
            value: 5,
        },
        Card {
            suit: Suit::Oracle,
            value: 4,
        },
        Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    ];

    state.phase = GamePhase::PlayerTurn;

    crate::games::dead_mans_draw::abilities::registry::execute_card_ability(
        &mut state,
        &Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    );

    handle_action(
        &mut state,
        GameAction::SelectMermaidTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.play_area[0].suit, Suit::Oracle);
    assert_eq!(state.play_area[1].suit, Suit::Mermaid);
    assert_eq!(state.play_area[2].suit, Suit::Anchor);

    assert_eq!(state.pending_ability, None);
    assert_eq!(state.phase, GamePhase::PlayerTurn);
}

#[test]
fn mermaid_reactivates_selected_card_ability() {
    let mut state = GameState::empty();
    state.variant = GameVariant::Mermaid;

    state.deck = vec![Card {
        suit: Suit::Key,
        value: 5,
    }];

    state.play_area = vec![
        Card {
            suit: Suit::Oracle,
            value: 4,
        },
        Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    ];

    state.phase = GamePhase::PlayerTurn;

    crate::games::dead_mans_draw::abilities::registry::execute_card_ability(
        &mut state,
        &Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    );

    handle_action(
        &mut state,
        GameAction::SelectMermaidTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(
        state.revealed_next_card,
        Some(Card {
            suit: Suit::Key,
            value: 5
        })
    );
}

#[test]
fn mermaid_reactivating_sword_enters_sword_target_phase() {
    let mut state = GameState::empty();
    state.variant = GameVariant::Mermaid;

    state.play_area = vec![
        Card {
            suit: Suit::Sword,
            value: 5,
        },
        Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    ];

    state.players[1].bank = vec![Card {
        suit: Suit::Anchor,
        value: 4,
    }];

    state.phase = GamePhase::PlayerTurn;

    crate::games::dead_mans_draw::abilities::registry::execute_card_ability(
        &mut state,
        &Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    );

    handle_action(
        &mut state,
        GameAction::SelectMermaidTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.pending_ability, Some(PendingAbility::Sword));
    assert_eq!(state.phase, GamePhase::WaitingForSwordTarget);
}

#[test]
fn mermaid_reactivating_map_enters_map_target_phase() {
    let mut state = GameState::empty();
    state.variant = GameVariant::Mermaid;

    state.discard = vec![
        Card {
            suit: Suit::Key,
            value: 5,
        },
        Card {
            suit: Suit::Chest,
            value: 4,
        },
    ];

    state.play_area = vec![
        Card {
            suit: Suit::Map,
            value: 5,
        },
        Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    ];

    state.phase = GamePhase::PlayerTurn;

    crate::games::dead_mans_draw::abilities::registry::execute_card_ability(
        &mut state,
        &Card {
            suit: Suit::Mermaid,
            value: 2,
        },
    );

    handle_action(
        &mut state,
        GameAction::SelectMermaidTarget {
            target_card_index: 0,
        },
    );

    assert_eq!(state.pending_ability, Some(PendingAbility::Map));
    assert_eq!(state.phase, GamePhase::WaitingForMapTarget);
    assert!(!state.map_choices.is_empty());
}
