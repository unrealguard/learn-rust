use core::hackernews_adatper::api::get_stories;

#[test]
fn get_stories_works() {
    tokio_test::block_on(async {
        let stories = get_stories(5).await;
        assert!(stories.is_ok());

        let number_of_stories = stories.unwrap().len();
        assert!(number_of_stories > 0);
    })
}
