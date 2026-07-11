use dioxus::prelude::*;

#[component]
pub fn GameLogPanel(game_log: Vec<String>) -> Element {
    rsx! {
        div {
            class: "rounded-2xl bg-white/10 p-4 shadow-md",

            h2 {
                class: "mb-3 text-xl font-extrabold",
                "Game Log"
            }

            div {
                class: "max-h-48 space-y-2 overflow-y-auto pr-1",

                if game_log.is_empty() {
                    p {
                        class: "text-sm text-white/50",
                        "No events yet."
                    }
                } else {
                    for entry in game_log.iter().rev().take(8) {
                        div {
                            class: "rounded-xl bg-black/20 px-3 py-2 text-sm text-white/80",
                            "{entry}"
                        }
                    }
                }
            }
        }
    }
}
