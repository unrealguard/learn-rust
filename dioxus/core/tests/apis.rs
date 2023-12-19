use core::hackernews_adatper::api::{get_stories, get_comment};

#[test]
fn get_stories_works() {
    tokio_test::block_on(async {
        let stories = get_stories(5).await;
        assert!(stories.is_ok());

        let number_of_stories = stories.unwrap().len();
        println!("Number of stories: {}", number_of_stories);
        assert!(number_of_stories > 0);
    });
}

#[test]
fn get_comment_works() {
    tokio_test::block_on(async {
        let comment_id = 38698579;
        let comment = get_comment(comment_id).await;
        if let Err(error) = &comment {
            println!("Encountered an error: {}", error);
        }
        assert!(comment.is_ok());

        let result_comment_id = comment.unwrap().id;
        assert_eq!(comment_id, result_comment_id)
    });
}
