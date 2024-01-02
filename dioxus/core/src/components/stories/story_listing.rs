#![allow(non_snake_case)]
use chrono::{DateTime, Utc};
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hackernews_adatper::api::get_story;

use super::{comment::Comment, preview::PreviewState};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}

async fn resolve_story(
    full_story: UseRef<Option<StoryPageData>>,
    preview_state: UseSharedState<PreviewState>,
    story_id: i64,
) {
    if let Some(cached) = &*full_story.read() {
        *preview_state.write() = PreviewState::Loaded(cached.clone());
        return;
    }

    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_story(story_id).await {
        *preview_state.write() = PreviewState::Loaded(story.clone());
        *full_story.write() = Some(story)
    }
}

#[component]
pub fn StoryListing(cx: Scope, story: StoryItem) -> Element {
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        id,
        ..
    } = story;

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!(
        "{score} {}",
        if *score == 1 { " points" } else { " points" }
    );

    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );

    let time = time.format("%D %l:%M %p");
    let preview_state = use_shared_state::<PreviewState>(cx).unwrap();
    let full_story = use_ref(cx, || None::<StoryPageData>);
    cx.render(rsx! {
        div {
            padding: "-1.5rem",
            position: "relative",
            onmouseenter: move |_| {
                resolve_story(full_story.clone(), preview_state.clone(), *id)
            },
            div {
                font_size: "1.5rem",
                a {
                    href: url,
                    onfocus: move |_event| {
                        resolve_story(full_story.clone(), preview_state.clone(), *id)
                    },
                    "{title}"
                }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                color: "gray",
                div {
                    "{score}"
                }
                div {
                    padding_left: "0.5rem",
                    "by {by}"
                }
                div {
                    padding_left: "0.5rem",
                    "{time}"
                },
                div {
                    padding_left: "0.5rem",
                    "{comments}"
                }
            }
        }
    })
}