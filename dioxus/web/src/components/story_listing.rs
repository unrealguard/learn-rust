#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

pub fn StoryListing(cx: Scope) -> Element {
    let title = "Story";
    let by = "William Jones";
    let score = -2;
    let time = chrono::Utc::now();

    cx.render(rsx! {
        div {
            padding: "-1.5rem",
            position: "relative",
            "{title} {by} {score} {time}"
        }       
    })
}