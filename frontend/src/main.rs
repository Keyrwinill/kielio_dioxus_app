mod app;
mod components;
mod pages;
mod services;

fn main() {
    dioxus::launch(app::App);
}

/* Initial 
use dioxus::prelude::*;
use shared::NameResponse;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let message = use_signal(|| "Click for the name".to_string());

    rsx! {
        div {
            h1 { "{message}" }

            button {
                onclick: move |_| {
                    spawn({
                        let mut message = message.clone();
                        async move {
                            let res = reqwest::get("http://127.0.1:3000/api/name")
                                .await
                                .unwrap()
                                .json::<NameResponse>()
                                .await
                                .unwrap();

                            message.set(format!("Hello, {}!", res.name));
                        }
                    });
                },
                "Get Name"
            }
        }
    }
}
*/