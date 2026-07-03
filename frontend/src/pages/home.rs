use dioxus::prelude::*;

#[component]
pub fn HomePage(
    on_play_dead_mans_draw: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "mx-auto min-h-screen max-w-6xl bg-slate-950 p-6 text-white",

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
                class: "grid gap-4 md:grid-cols-2",

                div {
                    class: "rounded-2xl bg-white/10 p-5 shadow-md",

                    h2 {
                        class: "text-2xl font-bold",
                        "Dead Man's Draw"
                    }

                    p {
                        class: "mt-2 text-sm text-white/70",
                        "Push your luck, bank your treasure, and avoid busting."
                    }

                    button {
                        class: "
                            mt-4 rounded-xl bg-amber-300 px-5 py-2
                            font-bold text-slate-900 hover:bg-amber-200
                        ",
                        onclick: move |event| on_play_dead_mans_draw.call(event),
                        "Play"
                    }
                }

                div {
                    class: "rounded-2xl bg-white/5 p-5 text-white/50 shadow-md",

                    h2 {
                        class: "text-2xl font-bold",
                        "Coming Soon"
                    }

                    p {
                        class: "mt-2 text-sm",
                        "More board games will be added later."
                    }
                }
            }
        }
    }
}