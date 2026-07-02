use dioxus::prelude::*;

use crate::components::panel::Panel;

#[component]
pub fn GameLogPanel(game_log: Vec<String>) -> Element {
    rsx! {
        Panel {
            title: "Game Log".to_string(),

            div {
                class: "
                    max-h-44 overflow-y-auto rounded-xl
                    bg-black/20 p-3
                ",

                for item in game_log.iter().rev().take(10) {
                    p {
                        class: "border-b border-white/10 py-1 text-sm text-white/90 last:border-b-0",
                        "{item}"
                    }
                }
            }
        }
    }
}