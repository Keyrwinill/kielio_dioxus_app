use dioxus::prelude::*;

use crate::pages::{
    dead_mans_draw::DeadMansDrawPage,
    home::HomePage,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppPage {
    Home,
    DeadMansDraw,
}

#[component]
pub fn App() -> Element {
    let mut page = use_signal(|| AppPage::Home);

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }

        match page() {
            AppPage::Home => rsx! {
                HomePage {
                    on_play_dead_mans_draw: move |_| {
                        page.set(AppPage::DeadMansDraw);
                    },
                }
            },

            AppPage::DeadMansDraw => rsx! {
                DeadMansDrawPage {}
            },
        }
    }
}