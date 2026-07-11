use dioxus::prelude::*;

use crate::components::{app_footer::AppFooter, app_header::AppHeader};

#[component]
pub fn AppLayout(page_title: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            class: "mx-auto min-h-screen max-w-6xl bg-slate-950 p-6 text-white",

            AppHeader {
                page_title
            }

            {children}

            AppFooter {}
        }
    }
}
