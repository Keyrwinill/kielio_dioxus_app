use super::ability::Ability;
use super::context::AbilityContext;

pub struct KrakenAbility;

impl Ability for KrakenAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        ctx.state.kraken_required_cards += 2;

        Some("Kraken demands 2 more cards before you can bank.".to_string())
    }
}