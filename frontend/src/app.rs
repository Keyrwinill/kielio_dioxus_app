use dioxus::prelude::*;

use crate::models::game_manifest::GameRoute;
use crate::games::dead_mans_draw::page::DeadMansDrawPage;
use crate::games::beasty_bar::page::BeastyBarPage;
use crate::pages::{
    home::HomePage,
};

impl GameRoute {
    pub fn to_route(self) -> Route {
        match self {
            GameRoute::DeadMansDraw => Route::DeadMansDrawPage {},
            GameRoute::BeastyBar => Route::BeastyBarPage {}, // temporary until Beasty Bar exists
        }
    }
}

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    HomePage {},

    #[route("/dead-mans-draw")]
    DeadMansDrawPage {},

    #[route("/beasty-bar")]
    BeastyBarPage {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }

        Router::<Route> {}
    }
}