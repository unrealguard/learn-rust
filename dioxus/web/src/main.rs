#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use crate::components::story_listing::{StoryListing, StoryItem};

mod components;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "William Jones".to_string(),
                score: 0,
                descendants: 0,
                time: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
        }
    })
}
