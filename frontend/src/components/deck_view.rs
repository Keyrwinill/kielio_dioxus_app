use dioxus::prelude::*;
use shared::games::dead_mans_draw::card::Card;

#[component]
pub fn DeckView(card_count: usize, revealed_next_card: Option<Card>) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center gap-2",

            div {
                class: "
                    relative flex h-32 w-20 items-center justify-center
                    rounded-xl border-2 border-white bg-slate-900
                    shadow-lg
                    before:absolute before:-left-1 before:top-1 before:h-32 before:w-20
                    before:rounded-xl before:border before:border-white/50 before:bg-slate-800
                    after:absolute after:-left-2 after:top-2 after:h-32 after:w-20
                    after:rounded-xl after:border after:border-white/30 after:bg-slate-700
                    sm:h-36 sm:w-24
                    sm:before:h-36 sm:before:w-24
                    sm:after:h-36 sm:after:w-24
                ",

                div {
                    class: "z-10 text-center",

                    div {
                        class: "text-2xl font-extrabold text-white",
                        "{card_count}"
                    }

                    div {
                        class: "text-xs font-bold tracking-widest text-white/70",
                        "DECK"
                    }
                }
            }

            if let Some(card) = revealed_next_card {
                div {
                    class: "
                        rounded-xl bg-amber-300 px-3 py-2
                        text-center text-xs font-bold text-slate-900
                    ",
                    "Next: {card.suit:?} {card.value}"
                }
            }
        }
    }
}
