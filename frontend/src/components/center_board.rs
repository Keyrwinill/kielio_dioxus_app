use dioxus::prelude::*;

use shared::games::dead_mans_draw::state::GameState;

use crate::components::deck_view::DeckView;
use crate::components::discard_pile_view::DiscardPileView;
use crate::components::map_choices_view::MapChoicesView;
use crate::components::panel::Panel;
use crate::components::play_area_view::PlayAreaView;

#[component]
pub fn CenterBoard(
    state: GameState,
    on_select_map_target: EventHandler<usize>,
) -> Element {
    rsx! {
        div {
            class: "mb-5 flex flex-col gap-5 lg:flex-row",
            
            // Deck section
            div {
                class: "lg:w-1/5",

                Panel {
                    title: "Deck".to_string(),

                    DeckView {
                        card_count: state.deck.len()
                    }
                }
            }

            // Play area section
            div {
                class: "rounded-2xl bg-white/10 p-4 shadow-md lg:w-3/5",

                div {
                    class: "space-y-4",

                    PlayAreaView {
                        cards: state.play_area.clone(),
                    }

                    // Map Choices
                    if !state.map_choices.is_empty() {
                        Panel {
                            title: "Map Choices".to_string(),

                            MapChoicesView {
                                cards: state.map_choices.clone(),
                                selectable: state.can_select_map_choices(),
                                on_select: move |card_index| {
                                    on_select_map_target.call(card_index);
                                },
                            }
                        }
                    }
                }
            }

            // Discard section
            div {
                class: "lg:w-1/5",

                Panel {
                    title: "Discard".to_string(),

                    DiscardPileView {
                        cards: state.discard.clone(),
                        selectable: false,
                        on_select: move |_| {},
                    }
                }
            }
        }
    }
}