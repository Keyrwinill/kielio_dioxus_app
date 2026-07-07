use dioxus::prelude::*;

use shared::games::dead_mans_draw::player::Player;
use shared::games::dead_mans_draw::scoring::score_player;
use shared::games::dead_mans_draw::state::{GamePhase, GameState};

use crate::components::card_view::CardView;

#[component]
pub fn PlayerPanel(
    state: GameState,
    player: Player,
    player_index: usize,
    on_select_cannon_target: EventHandler<(usize, usize)>,
    on_select_sword_target: EventHandler<(usize, usize)>,
    on_select_hook_target: EventHandler<usize>,
) -> Element {
    let score = score_player(&state, player_index);
    let is_current_player = player_index == state.current_player_index;
    let can_select_from_this_player = state.can_select_player_bank(player_index);
    let phase = state.phase.clone();

    rsx! {
        div {
            class: "mt-3 rounded-2xl border border-white/20 bg-white/10 p-4",

            div {
                class: "mb-3 flex items-start justify-between gap-3",

                div {
                    h4 {
                        class: "text-lg font-extrabold text-white",
                        "{player.name}"
                    }

                    p {
                        class: "mt-1 text-sm text-white/70",
                        "Banked: {player.bank.len()} card(s)"
                    }
                }

                div {
                    class: "text-right",

                    div {
                        class: "text-sm text-white/60",
                        "Score"
                    }

                    div {
                        class: "text-2xl font-extrabold text-white",
                        "{score}"
                    }

                    if is_current_player {
                        span {
                            class: "
                                mt-1 inline-block rounded-full bg-amber-300 px-2 py-0.5
                                text-xs font-bold text-slate-900
                            ",
                            "TURN"
                        }
                    }
                }
            }

            if can_select_from_this_player {
                div {
                    class: "
                        mb-3 rounded-xl border border-amber-300/50
                        bg-amber-300/10 px-3 py-2 text-sm font-bold text-amber-200
                    ",
                    "Select a valid top card."
                }
            }

            div {
                class: "max-w-full overflow-x-auto pb-2",

                div {
                    class: "flex flex-nowrap gap-0",

                    for (card_index, card) in player.bank.into_iter().enumerate() {
                        {
                            let phase_for_click = phase.clone();
                            let can_select_card =
                                state.can_select_bank_card(player_index, card_index);

                            rsx! {
                                button {
                                    disabled: !can_select_card,
                                    class: if can_select_card {
                                        "
                                        -ml-6 first:ml-0
                                        rounded-2xl border-4 border-amber-400
                                        bg-transparent p-0.5
                                        transition-transform duration-150
                                        hover:z-10 hover:scale-105
                                        sm:p-1
                                        "
                                    } else {
                                        "
                                        -ml-6 first:ml-0
                                        border-none bg-transparent p-0.5 opacity-90
                                        transition-transform duration-150
                                        hover:z-10 hover:scale-105
                                        sm:p-1
                                        "
                                    },

                                    onclick: move |_| {
                                        if !can_select_card {
                                            return;
                                        }

                                        match phase_for_click.clone() {
                                            GamePhase::WaitingForCannonTarget => {
                                                on_select_cannon_target.call((player_index, card_index));
                                            }
                                            GamePhase::WaitingForSwordTarget => {
                                                on_select_sword_target.call((player_index, card_index));
                                            }
                                            GamePhase::WaitingForHookTarget => {
                                                on_select_hook_target.call(card_index);
                                            }
                                            _ => {}
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
    }
}