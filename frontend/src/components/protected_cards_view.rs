use dioxus::prelude::*;
use shared::games::dead_mans_draw::card::Card;

use crate::components::card_collection::CardCollection;

#[component]
pub fn ProtectedCardsView(cards: Vec<Card>) -> Element {
    rsx! {
        div {
            class: "space-y-3",

            div {
                class: "
                    rounded-xl border border-sky-300/40
                    bg-sky-900/20 p-3 text-center
                    text-sm font-semibold text-sky-100
                ",
                "🛡 Protected: {cards.len()} card(s)"
            }

            CardCollection {
                title: "Safe Cards".to_string(),
                cards,
                selectable: false,
                border_color: "blue".to_string(),
                overlap: true,
                on_select: move |_| {},
            }
        }
    }
}