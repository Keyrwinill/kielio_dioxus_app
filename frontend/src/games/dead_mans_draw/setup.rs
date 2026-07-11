use dioxus::prelude::*;

use shared::games::dead_mans_draw::{player::Player, state::GameConfig, variant::GameVariant};

#[derive(Clone)]
struct PlayerSetup {
    name: String,
    is_ai: bool,
}

fn default_player(index: usize) -> PlayerSetup {
    if index == 0 {
        PlayerSetup {
            name: "You".to_string(),
            is_ai: false,
        }
    } else {
        PlayerSetup {
            name: format!("AI {index}"),
            is_ai: true,
        }
    }
}

#[component]
pub fn DeadMansDrawSetup(on_start: EventHandler<GameConfig>) -> Element {
    let mut players = use_signal(|| {
        vec![
            PlayerSetup {
                name: "You".to_string(),
                is_ai: false,
            },
            PlayerSetup {
                name: "AI 1".to_string(),
                is_ai: true,
            },
        ]
    });

    let mut selected_variant = use_signal(|| GameVariant::Base);

    rsx! {
        div {
            class: "rounded-2xl bg-white/10 p-5 shadow-md",

            h2 {
                class: "text-2xl font-extrabold",
                "New Game"
            }

            p {
                class: "mt-2 text-sm text-white/70",
                "Choose player count. Player 1 is human; the rest are AI for now."
            }

            div {
                class: "mt-4 flex flex-wrap gap-3",

                for count in [2usize, 3, 4] {
                    button {
                        class: if players().len() == count {
                            "rounded-xl bg-amber-300 px-4 py-2 font-bold text-slate-900"
                        } else {
                            "rounded-xl bg-white/10 px-4 py-2 font-bold text-white hover:bg-white/20"
                        },
                        onclick: move |_| {
                            let mut list = players.write();

                            while list.len() < count {
                                let index = list.len();
                                list.push(default_player(index));
                            }

                            list.truncate(count);
                        },
                        "{count} Players"
                    }
                }
            }

            div {
                class: "mt-5 space-y-2",

                for (index, player) in players().iter().enumerate() {
                    div {
                        class: "rounded-xl bg-black/20 px-4 py-3",

                        p {
                            class: "font-bold",
                            "Player {index + 1}: {player.name}"
                        }

                        input {
                            class: "
                                mt-2 w-full rounded-lg bg-black/30 px-3 py-2
                                text-sm text-white outline-none
                                focus:ring-2 focus:ring-amber-300
                            ",
                            value: "{player.name}",
                            oninput: move |event| {
                                players.write()[index].name = event.value();
                            },
                        }

                        div {
                            class: "mt-2 flex gap-2",

                            button {
                                class: if !player.is_ai {
                                    "rounded bg-amber-300 px-3 py-1 text-sm font-bold text-slate-900"
                                } else {
                                    "rounded bg-white/10 px-3 py-1 text-sm font-bold text-white"
                                },

                                onclick: move |_| {
                                    players.write()[index].is_ai = false;
                                },

                                "Human"
                            }

                            button {
                                class: if player.is_ai {
                                    "rounded bg-amber-300 px-3 py-1 text-sm font-bold text-slate-900"
                                } else {
                                    "rounded bg-white/10 px-3 py-1 text-sm font-bold text-white"
                                },

                                onclick: move |_| {
                                    players.write()[index].is_ai = true;
                                },

                                "AI"
                            }
                        }
                    }
                }
            }

            button {
                class: if selected_variant() == GameVariant::Base {
                    "rounded-xl bg-amber-300 px-4 py-2 font-bold text-slate-900"
                } else {
                    "rounded-xl bg-white/10 px-4 py-2 font-bold text-white hover:bg-white/20"
                },
                onclick: move |_| selected_variant.set(GameVariant::Base),
                "Base"
            }

            button {
                class: if selected_variant() == GameVariant::Mermaid {
                    "rounded-xl bg-amber-300 px-4 py-2 font-bold text-slate-900"
                } else {
                    "rounded-xl bg-white/10 px-4 py-2 font-bold text-white hover:bg-white/20"
                },
                onclick: move |_| selected_variant.set(GameVariant::Mermaid),
                "Mermaid Variant"
            }

            button {
                class: "
                    mt-6 w-full rounded-xl bg-amber-300 px-6 py-3
                    text-lg font-extrabold text-slate-900 shadow-lg
                    hover:bg-amber-200
                ",
                onclick: move |_| {
                    let game_players = players()
                        .iter()
                        .map(|player| {
                            let name = if player.name.trim().is_empty() {
                                "Player"
                            } else {
                                player.name.trim()
                            };

                            Player::new(name, player.is_ai)
                        })
                        .collect();

                    on_start.call(GameConfig {
                        players: game_players,
                        variant: selected_variant(),
                    });
                },
                "Start Game"
            }
        }
    }
}
