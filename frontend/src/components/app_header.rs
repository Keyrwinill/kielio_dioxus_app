use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn AppHeader(page_title: Option<String>) -> Element {
    rsx! {
        div {
            class: "
                mb-6 flex items-center justify-between
                rounded-2xl bg-white/10 px-5 py-4 shadow-md
            ",

            div {
                Link {
                    to: Route::HomePage {},
                    class: "text-xl font-extrabold text-white hover:text-amber-200",
                    "🎲 Board Game Hub"
                }

                if let Some(title) = page_title {
                    span {
                        class: "ml-2 text-sm font-bold text-white/60",
                        "/ {title}"
                    }
                }
            }

            div {
                class: "text-sm text-white/60",
                "v0.1.0"
            }
        }
    }
}