use leptos::prelude::*;
use leptos::server_fn::codec::{GetUrl, Json};

use backend::domain::user_db::{User, UserProfile};

#[server(GetUsers, "/api", input = GetUrl, output = Json)]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    let users = User::get_all(&app_state.pool).await.map_err(ServerFnError::new)?;
    Ok(users)
}

#[server(GetUserByUsername, "/api", input = GetUrl, output = Json)]
pub async fn get_user_by_username(username: String) -> Result<Option<User>, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    let user = User::get_by_username(&app_state.pool, &username)
        .await
        .map_err(ServerFnError::new)?;
    Ok(user)
}

#[server(GetUsersToFollow, "/api", input = GetUrl, output = Json)]
pub async fn get_users_to_follow() -> Result<Vec<UserProfile>, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    let users = User::get_others_with_follow_state(&app_state.pool, app_state.current_user_id)
        .await
        .map_err(ServerFnError::new)?;
    Ok(users)
}

#[server(GetUserProfileByUsername, "/api", input = GetUrl, output = Json)]
pub async fn get_user_profile_by_username(
    username: String,
) -> Result<UserProfile, ServerFnError> {
    use crate::common::app_state::use_app_state;

    let app_state = use_app_state()?;
    UserProfile::get_by_username(&app_state.pool, &username, app_state.current_user_id)
        .await
        .map_err(ServerFnError::new)
}
