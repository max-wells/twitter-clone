use leptos::prelude::*;
use leptos::server_fn::codec::{GetUrl, Json, PostUrl};
use uuid::Uuid;

use backend::domain::tweet_db::Tweet;

#[server(GetTweets, "/api", input = GetUrl, output = Json)]
pub async fn get_tweets() -> Result<Vec<Tweet>, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    let tweets = Tweet::get_all(&app_state.pool, app_state.current_user_id).await.map_err(ServerFnError::new)?;
    Ok(tweets)
}

#[server(GetTweetsByUsername, "/api", input = GetUrl, output = Json)]
pub async fn get_tweets_by_username(username: String) -> Result<Vec<Tweet>, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    let tweets =
        Tweet::get_by_username(&app_state.pool, &username, app_state.current_user_id).await.map_err(ServerFnError::new)?;
    Ok(tweets)
}

#[server(GetFollowingTweets, "/api", input = GetUrl, output = Json)]
pub async fn get_following_tweets() -> Result<Vec<Tweet>, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    let tweets =
        Tweet::get_following(&app_state.pool, app_state.current_user_id).await.map_err(ServerFnError::new)?;
    Ok(tweets)
}

#[server(PostTweet, "/api", input = PostUrl, output = Json)]
pub async fn post_tweet(content: String) -> Result<Tweet, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    let tweet = Tweet::add(&app_state.pool, app_state.current_user_id, content)
        .await
        .map_err(ServerFnError::new)?;
    Ok(tweet)
}

#[server(DeleteTweet, "/api", input = GetUrl, output = Json)]
pub async fn delete_tweet(unid: Uuid) -> Result<Uuid, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    let id = Tweet::delete(&app_state.pool, unid).await.map_err(ServerFnError::new)?;
    Ok(id)
}
