use super::helpers::*;
 
use crate::{
    games::dead_mans_draw::{
        abilities::{
            ability::Ability,
            context::AbilityContext,
            oracle::OracleAbility,
        }
    },
};

#[test]
fn oracle_reveals_next_card_without_drawing_it() {
    let mut state = GameState::empty();

    state.deck.clear();

    state.deck.push(card(Suit::Mermaid, 9));

    OracleAbility::execute(
        &mut AbilityContext {
            state: &mut state,
            card: card(Suit::Oracle, 5),
        },
    );

    assert_eq!(
        state.revealed_next_card.as_ref().unwrap().suit,
        Suit::Mermaid
    );
    assert_eq!(state.deck.len(), 1);
    assert_eq!(state.play_area.len(), 0);
}

#[test]
fn oracle_does_nothing_when_deck_is_empty() {
    let mut state = GameState::empty();

    state.deck.clear();

    OracleAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Oracle, 5),
    });

    assert!(state.revealed_next_card.is_none());
    assert!(state.deck.is_empty());
    assert_eq!(state.phase, GamePhase::PlayerTurn);
}

#[test]
fn drawing_after_oracle_draws_the_revealed_card() {
    let mut state = GameState::empty();

    state.deck.clear();

    state.deck.push(card(Suit::Hook, 3));
    state.deck.push(card(Suit::Mermaid, 9));

    OracleAbility::execute(&mut AbilityContext {
        state: &mut state,
        card: card(Suit::Oracle, 5),
    });

    assert_eq!(state.revealed_next_card.as_ref().unwrap().suit, Suit::Mermaid);

    handle_action(&mut state, GameAction::Draw);

    assert_eq!(state.play_area.len(), 1);
    assert_eq!(state.play_area[0].suit, Suit::Mermaid);
    assert!(state.revealed_next_card.is_none());
}