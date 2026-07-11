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
            class: "max-w-full overflow-x-auto pb-2",

            CardCollection {
                title: "".to_string(),
                cards,
                selectable,
                border_color: "green".to_string(),
                overlap: false,
                on_select,
            }
        }
    }
}
