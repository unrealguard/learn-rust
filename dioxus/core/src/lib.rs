#![allow(non_snake_case)]

use components::stories::preview::PreviewState;
use dioxus::prelude::*;
use dioxus_router::components::Router;

use crate::route::Route;

mod components;
mod route;
pub mod hackernews_adatper;

pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || PreviewState::Unset);
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            Router::<Route> {}
        }
    })
}



#[cfg(test)]
mod tests {

    #[test]
    fn add_works() {
        assert_eq!(4, 4);
    }
}
