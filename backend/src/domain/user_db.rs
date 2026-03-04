use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub unid: Uuid,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub created_at: OffsetDateTime,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserProfile {
    pub unid: Uuid,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub created_at: OffsetDateTime,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub is_following: bool,
    pub is_current_user: bool,
}

#[cfg(feature = "ssr")]
mod db {
    use sqlx::{PgPool, query_as};

    use super::*;

    impl User {
        pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
            query_as!(
                User,
                r#"
                    SELECT
                        unid,
                        username,
                        display_name,
                        bio,
                        created_at,
                        avatar_url,
                        banner_url
                    FROM users
                    ORDER BY username
                "#
            )
            .fetch_all(pool)
            .await
        }

        pub async fn get_by_username(
            pool: &PgPool,
            username: &str,
        ) -> Result<Option<User>, sqlx::Error> {
            query_as!(
                User,
                r#"
                    SELECT
                        unid,
                        username,
                        display_name,
                        bio,
                        created_at,
                        avatar_url,
                        banner_url
                    FROM users
                    WHERE username = $1
                "#,
                username
            )
            .fetch_optional(pool)
            .await
        }

        pub async fn get_others_with_follow_state(
            pool: &PgPool,
            current_user_id: Uuid,
        ) -> Result<Vec<UserProfile>, sqlx::Error> {
            query_as!(
                UserProfile,
                r#"
                    SELECT
                        u.unid,
                        u.username,
                        u.display_name,
                        u.bio,
                        u.created_at,
                        u.avatar_url,
                        u.banner_url,
                        EXISTS(
                            SELECT 1 FROM follows
                            WHERE follower_id = $1 AND following_id = u.unid
                        ) AS "is_following!: bool",
                        (u.unid = $1) AS "is_current_user!: bool"
                    FROM users u
                    WHERE u.unid != $1
                    ORDER BY u.username
                "#,
                current_user_id,
            )
            .fetch_all(pool)
            .await
        }
    }

    impl UserProfile {
        pub async fn get_by_username(
            pool: &PgPool,
            username: &str,
            current_user_id: Uuid,
        ) -> Result<UserProfile, sqlx::Error> {
            query_as!(
                UserProfile,
                r#"
                    SELECT
                        u.unid,
                        u.username,
                        u.display_name,
                        u.bio,
                        u.created_at,
                        u.avatar_url,
                        u.banner_url,
                        EXISTS(
                            SELECT 1 FROM follows
                            WHERE follower_id = $2 AND following_id = u.unid
                        ) AS "is_following!: bool",
                        (u.unid = $2) AS "is_current_user!: bool"
                    FROM users u
                    WHERE u.username = $1
                "#,
                username,
                current_user_id,
            )
            .fetch_one(pool)
            .await
        }
    }
}
