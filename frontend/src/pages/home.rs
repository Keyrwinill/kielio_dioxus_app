use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div {
            h2 { "Home" }
            p { "Choose a board game to play." }
        }
    }
}