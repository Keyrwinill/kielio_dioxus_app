use dioxus::prelude::*;
use shared::games::dead_mans_draw::card::{Card, Suit};

#[component]
pub fn CardView(card: Card) -> Element {
    let icon = suit_icon(card.suit);
    let color = suit_color(card.suit);

    rsx! {
        div {
            class: "
                h-32 w-20 sm:h-36 sm:w-24 overflow-hidden rounded-xl
                bg-white text-slate-900 shadow-lg
                transition-transform duration-150 hover:scale-105
            ",

            div {
                class: "{color} px-2 py-2 text-center text-xs font-bold text-white",
                "{icon} {card.suit:?}"
            }

            div {
                class: "
                    flex h-20 flex-col items-center justify-center
                ",

                div {
                    class: "text-3xl sm:text-4xl font-extrabold",
                    "{card.value}"
                }

                div {
                    class: "mt-2 text-xl sm:text-2xl",
                    "{icon}"
                }
            }
        }
    }
}

fn suit_icon(suit: Suit) -> &'static str {
    match suit {
        Suit::Anchor => "⚓",
        Suit::Cannon => "💣",
        Suit::Chest => "🧰",
        Suit::Hook => "🪝",
        Suit::Key => "🔑",
        Suit::Kraken => "🐙",
        Suit::Map => "🗺️",
        Suit::Mermaid => "🧜",
        Suit::Oracle => "🔮",
        Suit::Sword => "⚔️",
    }
}

fn suit_color(suit: Suit) -> &'static str {
    match suit {
        Suit::Anchor => "bg-blue-600",
        Suit::Cannon => "bg-red-600",
        Suit::Chest => "bg-yellow-600",
        Suit::Hook => "bg-orange-600",
        Suit::Key => "bg-amber-500",
        Suit::Kraken => "bg-purple-700",
        Suit::Map => "bg-green-600",
        Suit::Mermaid => "bg-pink-600",
        Suit::Oracle => "bg-indigo-600",
        Suit::Sword => "bg-slate-700",
    }
}