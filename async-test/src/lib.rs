#![allow(non_snake_case)]

use chrono::{DateTime, Utc};
use futures_util::future::join_all;
use serde::{Serialize, Deserialize};

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

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user/";
const COMMENT_DEPTH: i64 = 2;

pub async fn get_story_preview(id: i64) -> Result<StoryItem, reqwest::Error> {
    let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
    reqwest::get(&url).await?.json().await
}

pub async fn get_stories(
    count: usize,
) -> Result<Vec<StoryItem>, reqwest::Error> {
    let url = format!("{}topstories.json", BASE_API_URL);
    let stories_ids =
        &reqwest::get(&url).await?.json::<Vec<i64>>().await?[..count];

    let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
        .iter()
        .map(|&story_id| get_story_preview(story_id));

    let stories = join_all(story_futures)
        .await
        .into_iter()
        .inspect(|story| {
            match story {
                Ok(_) => {},
                Err(error) => println!("Get Story Error: {}", error),
            };
        })
        .filter_map(|story| story.ok())
        .collect();

    Ok(stories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let results = get_stories(5).await;
        match &results {
            Ok(_) => {}
            Err(error) => {
                println!("Error {}", error)
            }
        };
        assert!(results.is_ok());

        results.unwrap().into_iter().for_each(|story| {
            println!("Story: {:?}", story);
        });
    }
}
