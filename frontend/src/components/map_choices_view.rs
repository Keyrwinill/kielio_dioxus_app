use dioxus::prelude::*;
use shared::games::dead_mans_draw::card::Card;

use crate::components::card_collection::CardCollection;

#[component]
pub fn MapChoicesView(
    cards: Vec<Card>,
    selectable: bool,
    on_select: EventHandler<usize>,
) -> Element {
    rsx! {
        div {
            class: "space-y-3",

            div {
                class: "
                    rounded-xl border border-green-300/40
                    bg-green-900/30 p-3 text-center
                    text-sm font-semibold text-green-100
                ",
                "Map revealed {cards.len()} card(s)"
            }

            CardCollection {
                title: "Choose one".to_string(),
                cards,
                selectable,
                border_color: "green".to_string(),
                overlap: false,
                on_select,
            }
        }
    }
}