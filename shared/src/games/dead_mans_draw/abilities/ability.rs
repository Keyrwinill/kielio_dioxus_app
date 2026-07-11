use super::context::AbilityContext;

pub trait Ability {
    fn execute(ctx: &mut AbilityContext) -> Option<String>;
}
