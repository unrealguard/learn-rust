#![allow(non_snake_case)]
use core::{PreviewState, components::{story_listing::StoryListing, comment::Comment}, hackernews_adatper::api::get_stories};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || PreviewState::Unset);
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            div {
                width: "50%",
                Stories {}
            }
            div {
                width: "50%",
                Preview {}
            }
        }
    })
}

fn Stories(cx: Scope) -> Element {
    let stories = use_future(cx, (), |_| get_stories(10));

    match stories.value() {
        Some(Ok(list)) => {
            render! {
                div {
                    for story in list {
                        StoryListing { story: story.clone() }
                    }
                }
            }
        },
        Some(Err(err)) => {
            render! {"An error occured while fetching stories {err}" }
        },
        None => {
            render! { "Loading items" }
        },
    }
}


fn Preview(cx: Scope) -> Element {
    let preview_state = use_shared_state::<PreviewState>(cx)?;
    match &*preview_state.read() {
        PreviewState::Unset => render! {
            "Hover over a story to preview it here"
        },
        PreviewState::Loading => render! {
            "Loading ..."
        },
        PreviewState::Loaded(story) => {
            let title = &*story.item.title;
            let url = story.item.url.as_deref().unwrap_or_default();
            let text = story.item.text.as_deref().unwrap_or_default();
            render! {
                div {
                    padding: "0.5rem",
                    div {
                        font_size: "1.5rem",
                        a {
                            href: "{url}",
                            {title}
                        }
                    }
                }
                div {
                    dangerous_inner_html: "{text}"
                }
                for comment in &story.comments {
                    Comment { comment: comment.clone() }
                }
            }
        }
    }
}