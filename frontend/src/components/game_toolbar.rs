use dioxus::prelude::*;

#[component]
pub fn GameToolbar(
    game_over: bool,
    is_ai_turn: bool,
    has_cards_in_play: bool,
    on_draw: EventHandler<MouseEvent>,
    on_bank: EventHandler<MouseEvent>,
    on_ai: EventHandler<MouseEvent>,
    on_new_game: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "my-5 flex flex-wrap justify-center gap-3",

            button {
                class: "rounded-xl bg-white px-5 py-2 font-bold text-slate-900 shadow-md disabled:cursor-not-allowed disabled:opacity-40",
                disabled: game_over || is_ai_turn,
                onclick: move |event| on_draw.call(event),
                "Draw"
            }

            button {
                class: "min-w-24 rounded-xl bg-white px-4 py-2 font-bold text-slate-900 shadow-md disabled:cursor-not-allowed disabled:opacity-40 sm:px-5",
                disabled: game_over || is_ai_turn || !has_cards_in_play,
                onclick: move |event| on_bank.call(event),
                "Bank"
            }

            button {
                class: "min-w-24 rounded-xl bg-white px-4 py-2 font-bold text-slate-900 shadow-md disabled:cursor-not-allowed disabled:opacity-40 sm:px-5",
                disabled: game_over || !is_ai_turn,
                onclick: move |event| on_ai.call(event),
                "Let AI Play"
            }

            button {
                class: "min-w-24 rounded-xl bg-amber-300 px-4 py-2 font-bold text-slate-900 shadow-md hover:bg-amber-200 sm:px-5",
                onclick: move |event| on_new_game.call(event),
                "New Game"
            }
        }
    }
}
