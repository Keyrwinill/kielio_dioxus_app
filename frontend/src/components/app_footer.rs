use dioxus::prelude::*;

#[component]
pub fn AppFooter() -> Element {
    rsx! {
        footer {
            class: "mt-10 text-center text-xs text-white/40",
            "Board Game Hub • Built with Rust + Dioxus"
        }
    }
}
