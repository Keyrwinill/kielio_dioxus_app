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
    on_select_mermaid_target: EventHandler<usize>,
) -> Element {
    rsx! {
        div {
            class: "mb-5 grid grid-cols-1 gap-5 lg:grid-cols-[220px_1fr]",

            div {
                Panel {
                    title: "Deck".to_string(),

                    DeckView {
                        card_count: state.deck.len(),
                        revealed_next_card: state.revealed_next_card.clone(),
                    }
                }
            }

            div {
                class: "min-w-0 space-y-5",

                div {
                    class: "rounded-2xl bg-white/10 p-4 shadow-md",

                    PlayAreaView {
                        cards: state.play_area.clone(),
                        phase: state.phase.clone(),
                        on_select_mermaid_target,
                    }

                    if !state.map_choices.is_empty() {
                        div {
                            class: "mt-4",

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

                Panel {
                    title: format!("Discard ({} card(s))", state.discard.len()),

                    div {
                        class: "max-w-full overflow-x-auto pb-2",

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
}
