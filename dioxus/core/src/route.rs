#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::components::home::Home;
use crate::components::page_not_found::PageNotFound;
use crate::components::stories::stories::StoriesWithPreview;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
pub enum Route {
    // The home page is at the / route
    // If the name of the component and variant are the same you can omit the component and props name
    // If they are different you can specify them like this:
    // #[route("/", ComponentName, PropsName)]
    #[route("/")]
    Home,

    #[route("/stories")]
    StoriesWithPreview,

    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}