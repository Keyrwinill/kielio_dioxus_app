use super::helpers::*;

use crate::games::dead_mans_draw::abilities::ability::Ability;

#[test]
fn kraken_requires_two_more_cards_before_banking() {
    let mut state = GameState::new();

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
    let mut state = GameState::new();

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
    let mut state = GameState::new();

    state.kraken_required_cards = 0;

    state.play_area.push(Card {
        suit: Suit::Kraken,
        value: 5,
    });

    handle_action(&mut state, GameAction::Bank);

    assert_eq!(state.players[0].bank.len(), 1);
    assert_eq!(state.players[0].bank[0].suit, Suit::Kraken);
    assert_eq!(state.current_player_index, 1);
}