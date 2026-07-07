use dioxus::prelude::*;

use crate::models::game_manifest::GameStatus;

#[component]
pub fn GameCard(
    title: String,
    category: String,
    description: String,
    status: GameStatus,
    player_count: String,
    children: Element,
) -> Element {
    let class = match status {
        GameStatus::Ready => "rounded-2xl bg-white/10 p-5 shadow-md",
        GameStatus::ComingSoon => "rounded-2xl bg-white/5 p-5 text-white/50 shadow-md",
    };

    rsx! {
        div {
            class,

            h2 {
                class: "text-2xl font-bold",
                "{title}"
            }

            p {
                class: "mt-2 text-xs font-bold text-white/50",
                "{category} • {player_count}"
            }

            div {
                class: "mt-2",

                span {
                    class: status.badge_class(),
                    "{status.label()}"
                }
            }

            p {
                class: "mt-2 text-sm text-white/70",
                "{description}"
            }

            div {
                class: "mt-4",
                {children}
            }
        }
    }
}