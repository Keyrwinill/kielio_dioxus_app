use dioxus::prelude::*;

use shared::games::dead_mans_draw::state::GameState;

use crate::components::panel::Panel;

#[component]
pub fn GameStatusPanel(state: GameState) -> Element {
    let current_player = state.current_player().name.clone();

    rsx! {
        Panel {
            title: "Game Status".to_string(),

            div {
                class: "space-y-2",

                p {
                    class: "text-lg font-semibold",
                    "Current player: {current_player}"
                }

                if state.current_player().is_ai && !state.game_over {
                    div {
                        class: "
                            inline-flex items-center gap-2 rounded-full
                            bg-purple-200 px-3 py-1
                            text-sm font-bold text-purple-900
                        ",
                        span { "🤖" }
                        span { "AI is thinking..." }
                    }
                }

                p {
                    class: "text-white/90",
                    "{state.message}"
                }

                p {
                    class: "text-sm text-white/70",
                    "Phase: {state.phase:?}"
                }

                p {
                    class: "text-sm text-white/70",
                    "Cards left: {state.deck.len()}"
                }

                if let Some(selection) = &state.pending_selection {
                    div {
                        class: "
                            mt-2 rounded-xl border-2 border-amber-400
                            bg-amber-50 p-3 font-semibold text-slate-900
                        ",
                        "{selection.prompt}"
                    }
                }
            }
        }
    }
}