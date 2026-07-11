use super::ability::Ability;
use super::context::AbilityContext;

pub struct OracleAbility;

impl Ability for OracleAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        ctx.state.revealed_next_card = ctx.state.deck.last().cloned();

        ctx.state
            .revealed_next_card
            .as_ref()
            .map(|card| format!("Oracle reveals next card: {:?} {}.", card.suit, card.value))
    }
}
