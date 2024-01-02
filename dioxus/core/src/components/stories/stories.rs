#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{hackernews_adatper::api::get_stories, components::stories::{story_listing::StoryListing, preview::Preview}};


#[component]
pub fn StoriesWithPreview(cx: Scope) -> Element {
    cx.render(rsx! {
            div {
                width: "50%",
                Stories {}
            }
            div {
                width: "50%",
                Preview {}
            }
        })
}

#[component]
fn Stories(cx: Scope) -> Element {
    let stories = use_future(cx, (), |_| get_stories(9));

    match stories.value() {
        Some(Ok(list)) => {
            render! {
                div {
                    for story in list {
                        StoryListing { story: story.clone() }
                    }
                }
            }
        }
        Some(Err(err)) => {
            render! {"An error occured while fetching stories {err}" }
        }
        None => {
            render! { "Loading items" }
        }
    }
}