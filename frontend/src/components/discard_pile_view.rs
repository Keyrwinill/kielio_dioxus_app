use dioxus::prelude::*;
use shared::games::dead_mans_draw::card::Card;

use crate::components::card_collection::CardCollection;

#[component]
pub fn DiscardPileView(
    cards: Vec<Card>,
    selectable: bool,
    on_select: EventHandler<usize>,
) -> Element {
    rsx! {
        div {
            class: "space-y-3",

            div {
                class: "
                    rounded-xl border border-dashed border-white/30
                    bg-black/20 p-3 text-center text-sm text-white/70
                ",
                "Discard pile: {cards.len()} card(s)"
            }

            CardCollection {
                title: "Cards".to_string(),
                cards,
                selectable,
                border_color: "green".to_string(),
                overlap: true,
                on_select,
            }
        }
    }
}