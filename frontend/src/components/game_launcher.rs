use dioxus::prelude::*;

use crate::models::game_manifest::{GameRoute, GameStatus};

#[component]
pub fn GameLauncher(route: GameRoute, status: GameStatus) -> Element {
    match status {
        GameStatus::Ready => rsx! {
            Link {
                to: route.to_route(),
                class: "
                    inline-block rounded-xl bg-amber-300 px-5 py-2
                    font-bold text-slate-900 hover:bg-amber-200
                ",
                "{status.action_label()}"
            }
        },

        GameStatus::ComingSoon => rsx! {
            span {
                class: "text-sm text-white/50",
                "{status.action_label()}"
            }
        },
    }
}
