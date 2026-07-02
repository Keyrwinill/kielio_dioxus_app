use super::super::card::Card;
use super::super::state::GameState;

pub struct AbilityContext<'a> {
    pub state: &'a mut GameState,
    pub card: Card,
}