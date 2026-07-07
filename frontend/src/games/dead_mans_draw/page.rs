use dioxus::prelude::*;

use shared::dto::GameAction;
use shared::games::dead_mans_draw::state::{GameConfig, GameState};
use shared::games::dead_mans_draw::scoring::{score_player, winner_index};

use crate::games::dead_mans_draw::setup::DeadMansDrawSetup;

use crate::components::{
    app_layout::AppLayout,
    game_toolbar::GameToolbar,
    player_panel::PlayerPanel,
    panel::Panel,
    center_board::CenterBoard,
    game_status_panel::GameStatusPanel,
    game_log_panel::GameLogPanel,
    scoreboard_panel::ScoreboardPanel,
};
use crate::services::api::{fetch_game, send_action};


#[component]
pub fn DeadMansDrawPage(
    
) -> Element {
    let mut game = use_signal(|| None::<GameState>);
    let mut show_setup = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            if let Ok(response) = fetch_game().await {
                game.set(Some(response.state));
            }
        });
    });

    let do_action = move |action: GameAction| {
        spawn(async move {
            match send_action(action).await {
                Ok(response) => {
                    game.set(Some(response.state));
                }
                Err(error) => {
                    println!("Failed to send action: {:?}", error);
                }
            }
        });
    };

    let mut start_new_game = move |config| {
        show_setup.set(false);
        do_action(GameAction::StartNewGame { config });
    };

    rsx! {
        div {
            if let Some(state) = game() {
                DeadMansDrawView {
                    state,
                    show_setup: show_setup(),
                    on_start_new_game: move |config| start_new_game(config),
                    on_draw: move |_| do_action(GameAction::Draw),
                    on_bank: move |_| do_action(GameAction::Bank),
                    on_ai: move |_| do_action(GameAction::AiTurn),
                    on_new_game: move |_| do_action(GameAction::NewGame),
                    on_select_cannon_target: move |(player_index, card_index)| {
                        do_action(GameAction::SelectCannonTarget {
                            target_player_index: player_index,
                            target_card_index: card_index,
                        })
                    },
                    on_select_hook_target: move |card_index| {
                        do_action(GameAction::SelectHookTarget {
                            target_card_index: card_index,
                        })
                    },
                    on_select_map_target: move |card_index| {
                        do_action(GameAction::SelectMapTarget {
                            target_card_index: card_index,
                        })
                    },
                    on_select_sword_target: move |(player_index, card_index)| {
                        do_action(GameAction::SelectSwordTarget {
                            target_player_index: player_index,
                            target_card_index: card_index,
                        })
                    },
                }
            } else {
                p { "Loading..." }
            }
        }
    }
}

#[component]
fn DeadMansDrawView(
    state: GameState,
    show_setup: bool,
    on_start_new_game: EventHandler<GameConfig>,
    on_draw: EventHandler<MouseEvent>,
    on_bank: EventHandler<MouseEvent>,
    on_ai: EventHandler<MouseEvent>,
    on_new_game: EventHandler<MouseEvent>,
    on_select_cannon_target: EventHandler<(usize, usize)>,
    on_select_hook_target: EventHandler<usize>,
    on_select_map_target: EventHandler<usize>,
    on_select_sword_target: EventHandler<(usize, usize)>,
) -> Element {
    let is_ai_turn = state.current_player().is_ai;
    let has_cards_in_play = !state.play_area.is_empty();

    rsx! {

        AppLayout {
            page_title: Some("Dead Man's Draw".to_string()),

            div {
                class: "rounded-2xl bg-emerald-900 p-4 font-sans",
                
                div {
                    class: "mb-4 text-center",

                    h1 {
                        class: "text-3xl font-extrabold tracking-wide",
                        "Dead Man's Draw"
                    }

                    p {
                        class: "mt-2 text-sm text-white/70",
                        "Push your luck, bank your treasure, and avoid busting."
                    }
                }

                if show_setup {
                    DeadMansDrawSetup {
                        on_start: move |config| on_start_new_game.call(config),
                    }
                } else {
                        div {
                        class: "mb-4 grid grid-cols-1 gap-3 md:grid-cols-2",

                        GameStatusPanel {
                            state: state.clone(),
                        }

                        ScoreboardPanel {
                            state: state.clone(),
                        }
                    }

                    // Game over banner
                    if state.game_over {
                        if let Some(winner) = winner_index(&state) {
                            div {
                                class: "
                                    my-5 rounded-2xl bg-amber-300 p-5
                                    text-center text-slate-900 shadow-lg
                                ",

                                div {
                                    class: "text-2xl font-extrabold",
                                    "Game Over!"
                                }

                                div {
                                    class: "mt-1 text-xl font-bold",
                                    "Winner: {state.players[winner].name}"
                                }

                                div {
                                    class: "mt-3 flex flex-wrap justify-center gap-3",

                                    for (index, player) in state.players.iter().enumerate() {
                                        div {
                                            class: "rounded-xl bg-white px-4 py-2 font-bold",
                                            "{player.name}: {score_player(&state, index)}"
                                        }
                                    }
                                }

                                div {
                                    class: "mt-4",

                                    button {
                                        class: "
                                            rounded-xl bg-slate-900 px-5 py-2
                                            font-bold text-white shadow-md
                                            hover:bg-slate-700
                                        ",
                                        onclick: move |event| on_new_game.call(event),
                                        "Play Again"
                                    }
                                }
                            }
                        }
                    }

                    // Opponent area
                    Panel {
                        title: "Opponent".to_string(),

                        for (index, player) in state.players.clone().into_iter().enumerate() {
                            if index != state.current_player_index {
                                PlayerPanel {
                                    state: state.clone(),
                                    player,
                                    player_index: index,
                                    on_select_cannon_target,
                                    on_select_sword_target,
                                    on_select_hook_target,
                                }
                            }
                        }
                    }

                    // Center board
                    CenterBoard {
                        state: state.clone(),
                        on_select_map_target,
                    }

                    // Current player area
                    Panel {
                        title: "Current Player".to_string(),

                        for (index, player) in state.players.clone().into_iter().enumerate() {
                            if index == state.current_player_index {
                                PlayerPanel {
                                    state: state.clone(),
                                    player,
                                    player_index: index,
                                    on_select_cannon_target,
                                    on_select_sword_target,
                                    on_select_hook_target,
                                }
                            }
                        }
                    }

                    GameToolbar {
                        game_over: state.game_over,
                        is_ai_turn: is_ai_turn
                            || state.pending_selection.is_some(),
                        has_cards_in_play,
                        on_draw,
                        on_bank,
                        on_ai,
                        on_new_game,
                    }

                    GameLogPanel {
                        game_log: state.game_log.clone(),
                    }
                }
            }
        }
    }
}