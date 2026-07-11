use dioxus::prelude::*;
use shared::games::dead_mans_draw::{
    card::{Card, Suit},
    state::GamePhase,
};

use crate::components::card_collection::CardCollection;

#[component]
pub fn PlayAreaView(
    cards: Vec<Card>,
    phase: GamePhase,
    on_select_mermaid_target: EventHandler<usize>,
) -> Element {
    let can_select_mermaid_target = phase == GamePhase::WaitingForMermaidTarget;

    let cards_for_select = cards.clone();

    rsx! {
        div {
            class: "space-y-3",

            div {
                class: "
                    rounded-xl border border-amber-300/40
                    bg-amber-900/20 p-3 text-center
                    text-sm font-semibold text-amber-100
                ",
                "Current turn: {cards.len()} card(s)"
            }

            if can_select_mermaid_target {
                div {
                    class: "
                        rounded-xl border border-purple-300/60
                        bg-purple-500/20 px-3 py-2 text-center
                        text-sm font-bold text-purple-100
                    ",
                    "Mermaid: choose a non-Mermaid card in the play area."
                }
            }

            CardCollection {
                title: if can_select_mermaid_target {
                    "Choose a Mermaid target".to_string()
                } else {
                    "Cards in Play".to_string()
                },
                cards,
                selectable: can_select_mermaid_target,
                border_color: if can_select_mermaid_target {
                    "purple".to_string()
                } else {
                    "gray".to_string()
                },
                overlap: true,
                on_select: move |index: usize| {
                    if let Some(card) = cards_for_select.get(index) {
                        if card.suit != Suit::Mermaid {
                            on_select_mermaid_target.call(index);
                        }
                    }
                },
            }
        }
    }
}
