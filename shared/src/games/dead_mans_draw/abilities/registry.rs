use super::ability::Ability;
use super::anchor::AnchorAbility;
use super::cannon::{self, CannonAbility};
use super::chest::ChestAbility;
use super::context::AbilityContext;
use super::hook::{self, HookAbility};
use super::key::KeyAbility;
use super::kraken::KrakenAbility;
use super::map::{self, MapAbility};
use super::oracle::OracleAbility;
use super::sword::{self, SwordAbility};

use crate::games::dead_mans_draw::card::{Card, Suit};
use crate::games::dead_mans_draw::state::{
    GamePhase, GameState, PendingAbility, PendingSelection, SelectionSource,
};
use crate::games::dead_mans_draw::variant::GameVariant;

pub fn execute_card_ability(state: &mut GameState, card: &Card) -> Option<String> {
    match card.suit {
        Suit::Oracle => execute::<OracleAbility>(state, card),
        Suit::Anchor => execute::<AnchorAbility>(state, card),
        Suit::Chest => execute::<ChestAbility>(state, card),
        Suit::Key => execute::<KeyAbility>(state, card),
        Suit::Kraken => execute::<KrakenAbility>(state, card),

        Suit::Cannon => {
            if state.current_player().is_ai {
                cannon::auto_resolve_cannon_for_ai(state)
            } else {
                execute::<CannonAbility>(state, card)
            }
        }

        Suit::Hook => {
            if state.current_player().is_ai {
                hook::auto_resolve_hook_for_ai(state)
            } else {
                execute::<HookAbility>(state, card)
            }
        }

        Suit::Map => {
            if state.current_player().is_ai {
                map::auto_resolve_map_for_ai(state)
            } else {
                execute::<MapAbility>(state, card)
            }
        }

        Suit::Sword => {
            if state.current_player().is_ai {
                sword::auto_resolve_sword_for_ai(state)
            } else {
                execute::<SwordAbility>(state, card)
            }
        }

        Suit::Mermaid => {
            if state.variant != GameVariant::Mermaid {
                return None;
            }

            if state.play_area.len() <= 1 {
                return Some("Mermaid has no valid target.".to_string());
            }

            state.pending_ability = Some(PendingAbility::Mermaid);
            state.pending_selection = Some(PendingSelection {
                source: SelectionSource::PlayArea,
                prompt: "Choose a play area card for Mermaid.".to_string(),
            });
            state.phase = GamePhase::WaitingForMermaidTarget;

            return Some("Choose a play area card for Mermaid.".to_string());
        }
    }
}

fn execute<A: Ability>(state: &mut GameState, card: &Card) -> Option<String> {
    let mut ctx = AbilityContext {
        state,
        card: card.clone(),
    };

    A::execute(&mut ctx)
}
