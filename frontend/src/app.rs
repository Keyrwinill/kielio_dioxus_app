use dioxus::prelude::*;

//use crate::pages::home::HomePage;
use crate::pages::dead_mans_draw::DeadMansDrawPage;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        
        div {
            class: "mx-auto min-h-screen max-w-6xl bg-emerald-900 p-6 font-sans text-white",

            h1 { "Board Game App" }

            DeadMansDrawPage {}
            //HomePage {}
        }
    }
}