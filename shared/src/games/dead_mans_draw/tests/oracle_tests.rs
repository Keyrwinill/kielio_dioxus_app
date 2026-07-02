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
    let mut state = GameState::new();

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