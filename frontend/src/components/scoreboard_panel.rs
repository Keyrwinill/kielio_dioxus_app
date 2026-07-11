use dioxus::prelude::*;

use shared::games::dead_mans_draw::scoring::score_player;
use shared::games::dead_mans_draw::state::GameState;

use crate::components::panel::Panel;

#[component]
pub fn ScoreboardPanel(state: GameState) -> Element {
    rsx! {
        Panel {
            title: "Scoreboard".to_string(),

            div {
                class: "space-y-2",

                for (index, player) in state.players.iter().enumerate() {
                    div {
                        class: "flex items-center justify-between rounded-xl bg-black/20 px-3 py-2",

                        span {
                            class: "font-bold",
                            "{player.name}"
                        }

                        span {
                            class: "rounded-full bg-white px-3 py-1 font-bold text-slate-900",
                            "{score_player(&state, index)}"
                        }
                    }
                }
            }
        }
    }
}
