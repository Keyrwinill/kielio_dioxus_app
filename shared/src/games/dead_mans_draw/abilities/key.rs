use crate::games::dead_mans_draw::card::Suit;

use super::ability::Ability;
use super::context::AbilityContext;

pub struct KeyAbility;

impl Ability for KeyAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        let has_chest = ctx
            .state
            .play_area
            .iter()
            .any(|card| card.suit == Suit::Chest);

        if has_chest {
            Some("Key pairs with Chest. Bank now to claim bonus cards from discard.".to_string())
        } else {
            Some("Key waits for Chest.".to_string())
        }
    }
}
