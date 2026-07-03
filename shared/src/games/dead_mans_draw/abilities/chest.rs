use crate::games::dead_mans_draw::card::Suit;

use super::ability::Ability;
use super::context::AbilityContext;

pub struct ChestAbility;

impl Ability for ChestAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        let has_key = ctx
            .state
            .play_area
            .iter()
            .any(|card| card.suit == Suit::Key);

        if has_key {
            Some("Chest pairs with Key. Bank now to claim bonus cards from discard.".to_string())
        } else {
            Some("Chest waits for Key.".to_string())
        }
    }
}