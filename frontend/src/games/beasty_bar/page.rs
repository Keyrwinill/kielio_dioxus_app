use dioxus::prelude::*;

use crate::components::app_layout::AppLayout;

#[component]
pub fn BeastyBarPage() -> Element {
    rsx! {
        AppLayout {
            page_title: Some("Beasty Bar".to_string()),

            div {
                class: "rounded-2xl bg-white/10 p-6 text-center shadow-md",

                h1 {
                    class: "text-3xl font-extrabold",
                    "Beasty Bar"
                }

                p {
                    class: "mt-2 text-white/70",
                    "Coming soon."
                }
            }
        }
    }
}