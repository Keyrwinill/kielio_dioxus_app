use super::ability::Ability;
use super::context::AbilityContext;

pub struct AnchorAbility;

impl Ability for AnchorAbility {
    fn execute(ctx: &mut AbilityContext) -> Option<String> {
        if ctx.state.anchor_index.is_some() {
            return Some("Anchor is already active.".to_string());
        }

        if ctx.state.play_area.is_empty() {
            return Some("Anchor had nothing to protect.".to_string());
        }

        // Anchor is already in play_area when the ability executes.
        // Its index separates protected cards from unprotected cards.
        ctx.state.anchor_index = Some(ctx.state.play_area.len() - 1);

        Some("Anchor protects cards before it.".to_string())
    }
}
