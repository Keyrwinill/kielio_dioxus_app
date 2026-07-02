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

    let can_select_from_this_player =
        state.can_select_player_bank(player_index);

    let phase = state.phase.clone();

    rsx! {
        div {
            class: "mt-3 rounded-2xl border border-white/20 bg-white/10 p-4",

            h4 {
                class: "mb-2 flex items-center gap-2 text-lg font-bold text-white",

                "{player.name} - Score: {score}"

                if is_current_player {
                    span {
                        class: "
                            rounded-full bg-amber-300 px-2 py-0.5
                            text-xs font-bold text-slate-900
                        ",
                        "TURN"
                    }
                }
            }

            p {
                class: "mb-2 text-sm text-white/80",
                "Banked cards: {player.bank.len()}"
            }

            if can_select_from_this_player {
                p {
                    class: "mb-2 font-bold text-amber-300",
                    "Select a card."
                }
            }

            div {
                class: "flex flex-wrap gap-0",

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