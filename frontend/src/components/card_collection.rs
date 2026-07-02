use dioxus::prelude::*;
use shared::games::dead_mans_draw::card::Card;

use crate::components::card_view::CardView;

#[component]
pub fn CardCollection(
    title: String,
    cards: Vec<Card>,
    selectable: bool,
    border_color: String,
    on_select: EventHandler<usize>,
    overlap: bool,
) -> Element {
    let selectable_class = match border_color.as_str() {
        "red" => "border-red-500",
        "blue" => "border-blue-500",
        "green" => "border-green-500",
        "orange" => "border-orange-500",
        "purple" => "border-purple-500",
        _ => "border-slate-400",
    };

    let container_class = if overlap {
        "flex flex-wrap gap-0"
    } else {
        "flex flex-wrap gap-2 sm:gap-3"
    };

    let spacing_class = if overlap {
        "-ml-6 first:ml-0"
    } else {
        ""
    };

    rsx! {
        div {
            class: "space-y-2",

            h3 {
                class: "text-lg font-bold text-white",
                "{title}"
            }

            if cards.is_empty() {
                div {
                    class: "
                        rounded-xl border border-dashed border-white/30
                        px-4 py-6 text-center text-sm text-white/60
                    ",
                    "No cards"
                }
            } else {
                div {
                    class: "flex flex-wrap gap-0",

                    for (index, card) in cards.into_iter().enumerate() {
                        button {
                            disabled: !selectable,
                            class: if selectable {
                                format!(
                                    "
                                    {} rounded-2xl border-4 {} bg-transparent p-0.5
                                    transition-transform duration-150
                                    hover:z-10 hover:scale-105 hover:bg-white/10
                                    focus:z-10 focus:outline-none focus:ring-4 focus:ring-amber-300
                                    sm:p-1
                                    ",
                                    spacing_class,
                                    selectable_class
                                )
                            } else {
                                format!(
                                    "
                                    {} border-none bg-transparent p-0.5 opacity-90
                                    transition-transform duration-150 hover:z-10 hover:scale-105
                                    sm:p-1
                                    ",
                                    spacing_class
                                )
                            },
                            onclick: move |_| {
                                if selectable {
                                    on_select.call(index);
                                }
                            },
                            CardView { card }
                        }
                    }
                }
            }
        }
    }
}