use dioxus::prelude::*;

use shared::games::dead_mans_draw::state::{GamePhase, GameState};
use shared::games::dead_mans_draw::variant::GameVariant;

#[component]
pub fn GameStatusPanel(state: GameState) -> Element {
    let variant_name = match state.variant {
        GameVariant::Base => "Base",
        GameVariant::Mermaid => "Mermaid Variant",
    };

    let current_player = state.current_player();

    let status_title = if state.game_over {
        "🏁 Game Over"
    } else if current_player.is_ai {
        "🤖 AI Turn"
    } else {
        match state.phase {
            GamePhase::PlayerTurn => "🟢 Your Turn",
            GamePhase::WaitingForCannonTarget => "🎯 Choose Cannon target",
            GamePhase::WaitingForHookTarget => "🪝 Choose Hook target",
            GamePhase::WaitingForMapTarget => "🗺️ Choose Map card",
            GamePhase::WaitingForSwordTarget => "⚔️ Choose Sword target",
            GamePhase::WaitingForMermaidTarget => "🧜 Choose Mermaid target",
            GamePhase::GameOver => "🏁 Game Over",
        }
    };

    let instruction = match state.phase {
        GamePhase::PlayerTurn => {
            if current_player.is_ai {
                "Let the AI play."
            } else {
                "Draw a card or bank your cards."
            }
        }
        GamePhase::WaitingForCannonTarget => "Choose an opponent card to discard.",
        GamePhase::WaitingForHookTarget => "Choose one of your top bank cards to replay.",
        GamePhase::WaitingForMapTarget => "Choose one revealed discard card to replay.",
        GamePhase::WaitingForSwordTarget => "Choose an opponent card to steal.",
        GamePhase::WaitingForMermaidTarget => "Choose a play area card for Mermaid.",
        GamePhase::GameOver => "Start a new game.",
    };

    rsx! {
        div {
            class: "rounded-2xl bg-white/10 p-4 shadow-md",

            h2 {
                class: "text-xl font-extrabold",
                "{status_title}"
            }

            p {
                class: "mt-1 text-sm text-white/70",
                "{instruction}"
            }

            div {
                class: "mt-4 grid grid-cols-2 gap-3 text-sm",

                div {
                    class: "rounded-xl bg-black/20 p-3",
                    div { class: "text-white/50", "Current Player" }
                    div { class: "font-bold", "{current_player.name}" }
                }

                div {
                    class: "rounded-xl bg-black/20 p-3",
                    div { class: "text-white/50", "Cards Left" }
                    div { class: "font-bold", "{state.deck.len()}" }
                }

                div {
                    class: "rounded-xl bg-black/20 p-3",
                    div { class: "text-white/50", "Cards in Play" }
                    div { class: "font-bold", "{state.play_area.len()}" }
                }

                div {
                    class: "rounded-xl bg-black/20 p-3",
                    div { class: "text-white/50", "Discard" }
                    div { class: "font-bold", "{state.discard.len()}" }
                }

                div {
                    class: "rounded-xl bg-black/20 p-3",
                    div { class: "text-white/50", "Variant" }
                    div { class: "font-bold", "{variant_name}" }
                }
            }
        }
    }
}
