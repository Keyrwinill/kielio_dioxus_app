use dioxus::prelude::*;
use shared::games::dead_mans_draw::card::Card;

use crate::components::card_collection::CardCollection;

#[component]
pub fn PlayAreaView(cards: Vec<Card>) -> Element {
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

            CardCollection {
                title: "Cards in Play".to_string(),
                cards,
                selectable: false,
                border_color: "gray".to_string(),
                overlap: true,
                on_select: move |_| {},
            }
        }
    }
}