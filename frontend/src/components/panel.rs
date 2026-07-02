use dioxus::prelude::*;

#[component]
pub fn Panel(
    title: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "
                rounded-2xl
                border border-white/10
                bg-white/10
                p-4
                shadow-lg
                backdrop-blur-sm
            ",

            h2 {
                class: "mb-3 text-xl font-bold text-white",
                "{title}"
            }

            {children}
        }
    }
}