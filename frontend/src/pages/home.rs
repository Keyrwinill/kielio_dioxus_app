use dioxus::prelude::*;

use crate::{
    components::{
        app_layout::AppLayout,
        game_card::GameCard,
        game_launcher::GameLauncher,
    },
    data::games::GAMES,
};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        AppLayout {
            page_title: None,

            div {
                class: "mb-8 text-center",

                h1 {
                    class: "text-4xl font-extrabold",
                    "Board Game Hub"
                }

                p {
                    class: "mt-2 text-white/70",
                    "Choose a game to play."
                }
            }

            div {
                class: "mb-4",

                h2 {
                    class: "text-2xl font-bold",
                    "Available Games"
                }

                p {
                    class: "mt-1 text-sm text-white/60",
                    "Pick a game below. More games can be added to the catalog later."
                }
            }

            div {
                class: "grid gap-4 md:grid-cols-2",

                for game in GAMES {                    
                    
                    GameCard {
                        title: game.title.to_string(),
                        category: game.category.label().to_string(),
                        description: game.description.to_string(),
                        status: game.status,
                        player_count: game.player_count_label(),                        

                        GameLauncher {
                            route: game.route,
                            status: game.status,
                        }
                    }
                }
            }
        }
    }
}