use super::helpers::*;

use crate::games::dead_mans_draw::abilities::{
    ability::Ability,
    anchor::AnchorAbility,
    context::AbilityContext,
};

#[test]
fn anchor_with_no_previous_cards_sets_anchor_index_to_zero() {
    let mut state = GameState::new();

    state.play_area.push(Card {
        suit: Suit::Anchor,
        value: 4,
    });

    AnchorAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Anchor, 4),
    });

    assert_eq!(state.anchor_index, Some(0));
    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Anchor);
}

#[test]
fn anchor_protected_cards_are_banked_when_later_card_busts() {
    let mut state = GameState::new();

    state.play_area.push(Card {
        suit: Suit::Key,
        value: 3,
    });

    state.play_area.push(Card {
        suit: Suit::Chest,
        value: 4,
    });

    state.play_area.push(Card {
        suit: Suit::Cannon,
        value: 5,
    });

    state.play_area.push(Card {
        suit: Suit::Anchor,
        value: 6,
    });

    state.anchor_index = Some(3);

    state.deck.clear();

    state.deck.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    state.deck.push(Card {
        suit: Suit::Cannon,
        value: 7,
    });

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.players[0].bank.len(), 3);
    assert!(state.players[0].bank.iter().any(|c| c.suit == Suit::Key));
    assert!(state.players[0].bank.iter().any(|c| c.suit == Suit::Chest));
    assert!(state.players[0].bank.iter().any(|c| c.suit == Suit::Cannon && c.value == 5));

    assert_eq!(state.discard.len(), 2);
    assert!(state.discard.iter().any(|c| c.suit == Suit::Anchor && c.value == 6));
    assert!(state.discard.iter().any(|c| c.suit == Suit::Cannon && c.value == 7));
}

#[test]
fn banking_with_anchor_banks_everything() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Key, 2),
            card(Suit::Chest, 3),
            card(Suit::Anchor, 4),
            card(Suit::Map, 5),
        ],
        Some(2),
    );

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.players[0].bank.len(), 4);
    assert!(state.play_area.is_empty());
    assert_eq!(state.anchor_index, None);
}

#[test]
fn bust_with_anchor_as_first_card_banks_nothing() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Anchor, 1),
            card(Suit::Map, 2),
            card(Suit::Sword, 3),
        ],
        Some(0),
    );

    resolve_bust(&mut state, "Bust".to_string());

    assert!(state.players[0].bank.is_empty());

    assert_eq!(state.discard.len(), 3);
    assert!(state.play_area.is_empty());
}

#[test]
fn bust_with_anchor_as_last_card_banks_previous_cards() {
let mut state = GameState::new();

setup_play_area(
    &mut state,
    vec![
        card(Suit::Key, 2),
        card(Suit::Chest, 3),
        card(Suit::Anchor, 4),
    ],
    Some(2),
);

resolve_bust(&mut state, "Bust".to_string());

assert_eq!(state.players[0].bank.len(), 2);

assert!(state.players[0].bank.iter().any(|c| c.suit == Suit::Key));
assert!(state.players[0].bank.iter().any(|c| c.suit == Suit::Chest));

assert_eq!(state.discard.len(), 1);
assert_eq!(state.discard[0].suit, Suit::Anchor);
}

#[test]
fn bust_without_anchor_discards_everything() {
    let mut state = GameState::new();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Key, 2),
            card(Suit::Chest, 3),
            card(Suit::Cannon, 4),
        ],
        None,
    );

    resolve_bust(&mut state, "Bust".to_string());

    assert!(state.players[0].bank.is_empty());

    assert_eq!(state.discard.len(), 3);
    assert!(state.play_area.is_empty());
}

#[test]
fn anchor_banks_cards_before_anchor_when_later_bust_happens() {
    let mut state = GameState::new();

    state.play_area.push(Card {
        suit: Suit::Oracle,
        value: 5,
    });

    state.play_area.push(Card {
        suit: Suit::Anchor,
        value: 4,
    });

    crate::games::dead_mans_draw::abilities::anchor::AnchorAbility::execute(
        &mut crate::games::dead_mans_draw::abilities::context::AbilityContext {
            state: &mut state,
            card: Card {
                suit: Suit::Anchor,
                value: 4,
            },
        },
    );

    state.deck.clear();

    state.deck.push(Card {
        suit: Suit::Mermaid,
        value: 9,
    });

    state.deck.push(Card {
        suit: Suit::Anchor,
        value: 6,
    });

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.players[0].bank.len(), 1);
    assert_eq!(state.players[0].bank[0].suit, Suit::Oracle);

    assert_eq!(state.discard.len(), 2);
    assert!(state.discard.iter().any(|c| c.suit == Suit::Anchor && c.value == 4));
    assert!(state.discard.iter().any(|c| c.suit == Suit::Anchor && c.value == 6));
}