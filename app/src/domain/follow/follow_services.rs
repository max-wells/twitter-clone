use leptos::prelude::*;
use leptos::server_fn::codec::{Json, PostUrl};
use uuid::Uuid;

#[server(ToggleFollow, "/api", input = PostUrl, output = Json)]
pub async fn toggle_follow(following_id: Uuid) -> Result<bool, ServerFnError> {
    use crate::common::app_state::use_app_state;
    use backend::domain::follow_db::Follow;

    let app_state = use_app_state()?;
    let is_following = Follow::toggle(&app_state.pool, app_state.current_user_id, following_id)
        .await
        .map_err(ServerFnError::new)?;
    Ok(is_following)
}
