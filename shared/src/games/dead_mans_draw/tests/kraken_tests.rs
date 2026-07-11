use super::helpers::*;

use crate::games::dead_mans_draw::{
    abilities::{ability::Ability, context::AbilityContext, kraken::KrakenAbility},
    engine::add_card_to_play_area,
};

#[test]
fn kraken_requires_two_more_cards_before_banking() {
    let mut state = GameState::empty();

    state.deck.push(card(Suit::Hook, 3));
    state.deck.push(card(Suit::Map, 4));

    crate::games::dead_mans_draw::abilities::kraken::KrakenAbility::execute(
        &mut crate::games::dead_mans_draw::abilities::context::AbilityContext {
            state: &mut state,
            card: Card {
                suit: Suit::Kraken,
                value: 4,
            },
        },
    );

    assert_eq!(state.kraken_required_cards, 2);

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.current_player_index, 0);
    assert_eq!(state.kraken_required_cards, 2);
}

#[test]
fn drawing_cards_satisfies_kraken_requirement() {
    let mut state = GameState::empty();

    state.kraken_required_cards = 2;

    state.deck.clear();

    // Draw order (last pushed is drawn first)
    state.deck.push(Card {
        suit: Suit::Oracle,
        value: 5,
    });

    state.deck.push(Card {
        suit: Suit::Chest,
        value: 4,
    });

    handle_action(&mut state, GameAction::Draw);
    assert_eq!(state.kraken_required_cards, 1);

    handle_action(&mut state, GameAction::Draw);
    assert_eq!(state.kraken_required_cards, 0);
}

#[test]
fn banking_allowed_after_kraken_requirement_is_satisfied() {
    let mut state = GameState::empty();

    // Kraken needs 2 available cards to require 2 draws.
    state.deck.push(card(Suit::Hook, 3));
    state.deck.push(card(Suit::Map, 4));

    state.play_area.push(card(Suit::Kraken, 5));

    KrakenAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Kraken, 5),
    });

    assert_eq!(state.kraken_required_cards, 2);

    add_card_to_play_area(&mut state, card(Suit::Hook, 3));
    add_card_to_play_area(&mut state, card(Suit::Map, 4));

    assert_eq!(state.kraken_required_cards, 0);

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.players[0].bank.len(), 3);
}

#[test]
fn kraken_requirement_is_cleared_after_bust() {
    let mut state = GameState::empty();

    state.kraken_required_cards = 2;

    setup_play_area(&mut state, vec![card(Suit::Cannon, 3)], None);

    state.deck.clear();
    state.deck.push(card(Suit::Cannon, 8));

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.kraken_required_cards, 0);
    assert!(state.play_area.is_empty());
    assert_eq!(state.discard.len(), 2);
}

#[test]
fn kraken_only_requires_one_card_when_one_card_remains_in_deck() {
    let mut state = GameState::empty();

    state.deck.clear();
    state.deck.push(card(Suit::Hook, 5));

    KrakenAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Kraken, 3),
    });

    assert_eq!(state.kraken_required_cards, 1);
}

#[test]
fn kraken_requires_no_cards_when_deck_is_empty() {
    let mut state = GameState::empty();

    state.deck.clear();

    KrakenAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Kraken, 3),
    });

    assert_eq!(state.kraken_required_cards, 0);
}
