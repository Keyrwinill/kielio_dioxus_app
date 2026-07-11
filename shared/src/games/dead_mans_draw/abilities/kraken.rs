use super::ability::Ability;
use super::context::AbilityContext;

pub struct KrakenAbility;

impl Ability for KrakenAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        let required = ctx.state.deck.len().min(2);
        ctx.state.kraken_required_cards += required;

        if required == 0 {
            Some("Kraken appeared, but the deck is empty.".to_string())
        } else {
            Some(format!(
                "Kraken demands {} more card(s) before you can bank.",
                required
            ))
        }
    }
}
