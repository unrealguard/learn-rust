#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::stories::comment::Comment;

use super::story_listing::StoryPageData;

#[derive(Clone, Debug)]
pub enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}

pub fn Preview(cx: Scope) -> Element {
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