use super::helpers::*;

use crate::games::dead_mans_draw::{abilities::{
    ability::Ability,
    chest::ChestAbility,
    context::AbilityContext,
    key::KeyAbility,
}, player::Player};

#[test]
fn banking_chest_without_key_gets_no_bonus() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Chest, 3),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.players[0].bank.len(), 2);
    assert_eq!(state.discard.len(), 1);
}

#[test]
fn banking_key_without_chest_gets_no_bonus() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Key, 3),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.players[0].bank.len(), 2);
    assert_eq!(state.discard.len(), 1);
}

#[test]
fn banking_chest_and_key_claims_bonus_from_discard() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Chest, 3),
            card(Suit::Key, 4),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));
    state.discard.push(card(Suit::Oracle, 6));
    state.discard.push(card(Suit::Sword, 7));

    handle_action(&mut state, GameAction::Bank);

    // 3 collected + 3 bonus
    assert_eq!(state.players[0].bank.len(), 6);
    assert_eq!(state.discard.len(), 0);
}

#[test]
fn chest_and_key_claims_only_available_discard_cards() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Chest, 3),
            card(Suit::Key, 4),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));

    handle_action(&mut state, GameAction::Bank);

    // 3 collected + only 1 available bonus
    assert_eq!(state.players[0].bank.len(), 4);
    assert_eq!(state.discard.len(), 0);
}

#[test]
fn chest_and_key_do_not_claim_bonus_when_saved_by_anchor_on_bust() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Key, 3),
            card(Suit::Chest, 4),
            card(Suit::Anchor, 5),
        ],
        Some(2),
    );

    state.discard.push(card(Suit::Mermaid, 9));

    state.deck.clear();
    state.deck.push(card(Suit::Anchor, 6));

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.players[0].bank.len(), 2);
    assert_eq!(state.discard.len(), 3); // Mermaid + Anchor 5 + Anchor 6
}

#[test]
fn chest_and_key_with_empty_discard_gives_no_bonus() {
    let mut state = GameState::empty();

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Chest, 3),
            card(Suit::Key, 4),
        ],
        None,
    );

    state.discard.clear();

    handle_action(&mut state, GameAction::Bank);

    // Only the collected cards are banked.
    assert_eq!(state.players[0].bank.len(), 2);
    assert!(state.discard.is_empty());
}

#[test]
fn key_reports_pair_when_chest_is_in_play() {
    let mut state = GameState::empty();

    state.play_area.push(card(Suit::Chest, 3));

    let message = KeyAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Key, 5),
    });

    assert_eq!(
        message.unwrap(),
        "Key pairs with Chest. Bank now to claim bonus cards from discard."
    );
}

#[test]
fn chest_reports_pair_when_key_is_in_play() {
    let mut state = GameState::empty();

    state.play_area.push(card(Suit::Key, 3));

    let message = ChestAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Chest, 5),
    });

    assert_eq!(
        message.unwrap(),
        "Chest pairs with Key. Bank now to claim bonus cards from discard."
    );
}

#[test]
fn key_and_chest_bonus_goes_to_current_player_in_multiplayer_game() {
    let mut state = GameState::empty();
    state.players = vec![
        Player::new("P1", false),
        Player::new("P2", false),
        Player::new("P3", true),
    ];

    state.current_player_index = 1;

    setup_play_area(
        &mut state,
        vec![
            card(Suit::Chest, 3),
            card(Suit::Key, 4),
            card(Suit::Map, 5),
        ],
        None,
    );

    state.discard.push(card(Suit::Mermaid, 9));
    state.discard.push(card(Suit::Oracle, 8));

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.players[1].bank.len(), 5);
    assert!(state.players[0].bank.is_empty());
    assert!(state.players[2].bank.is_empty());
}