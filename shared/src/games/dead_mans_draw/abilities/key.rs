use super::ability::Ability;
use super::context::AbilityContext;

pub struct KeyAbility;

impl Ability for KeyAbility {
    fn execute(_ctx: &mut AbilityContext) -> Option<String> {
        Some("Key has no effect until collected with Chest.".to_string())
    }
}