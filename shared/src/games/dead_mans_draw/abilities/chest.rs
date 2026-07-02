use super::ability::Ability;
use super::context::AbilityContext;

pub struct ChestAbility;

impl Ability for ChestAbility {
    fn execute(_ctx: &mut AbilityContext) -> Option<String> {
        Some("Chest has no effect until collected with Key.".to_string())
    }
}