use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Tweet {
    pub unid: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub author_username: String,
    pub author_display_name: String,
    pub author_avatar_url: Option<String>,
    pub is_mine: bool,
}

#[cfg(feature = "ssr")]
mod db {
    use sqlx::{PgPool, query_as};

    use super::*;

    impl Tweet {
        pub async fn get_all(pool: &PgPool, current_user_id: Uuid) -> Result<Vec<Tweet>, sqlx::Error> {
            query_as!(
                Tweet,
                r#"
                    SELECT
                        t.unid,
                        t.author_id,
                        t.content,
                        t.created_at,
                        u.username AS author_username,
                        u.display_name AS author_display_name,
                        u.avatar_url AS author_avatar_url,
                        (t.author_id = $1) AS "is_mine!: bool"
                    FROM tweets t
                    JOIN users u ON u.unid = t.author_id
                    ORDER BY t.created_at DESC
                "#,
                current_user_id,
            )
            .fetch_all(pool)
            .await
        }

        pub async fn get_by_username(
            pool: &PgPool,
            username: &str,
            current_user_id: Uuid,
        ) -> Result<Vec<Tweet>, sqlx::Error> {
            query_as!(
                Tweet,
                r#"
                    SELECT
                        t.unid,
                        t.author_id,
                        t.content,
                        t.created_at,
                        u.username AS author_username,
                        u.display_name AS author_display_name,
                        u.avatar_url AS author_avatar_url,
                        (t.author_id = $2) AS "is_mine!: bool"
                    FROM tweets t
                    JOIN users u ON u.unid = t.author_id
                    WHERE u.username = $1
                    ORDER BY t.created_at DESC
                "#,
                username,
                current_user_id,
            )
            .fetch_all(pool)
            .await
        }

        pub async fn add(
            pool: &PgPool,
            author_id: Uuid,
            content: String,
        ) -> Result<Tweet, sqlx::Error> {
            query_as!(
                Tweet,
                r#"
                    WITH inserted AS (
                        INSERT INTO tweets (author_id, content)
                        VALUES ($1, $2)
                        RETURNING unid, author_id, content, created_at
                    )
                    SELECT
                        i.unid,
                        i.author_id,
                        i.content,
                        i.created_at,
                        u.username AS author_username,
                        u.display_name AS author_display_name,
                        u.avatar_url AS author_avatar_url,
                        (i.author_id = $1) AS "is_mine!: bool"
                    FROM inserted i
                    JOIN users u ON u.unid = i.author_id
                "#,
                author_id,
                content
            )
            .fetch_one(pool)
            .await
        }

        pub async fn get_following(pool: &PgPool, current_user_id: Uuid) -> Result<Vec<Tweet>, sqlx::Error> {
            query_as!(
                Tweet,
                r#"
                    SELECT
                        t.unid,
                        t.author_id,
                        t.content,
                        t.created_at,
                        u.username AS author_username,
                        u.display_name AS author_display_name,
                        u.avatar_url AS author_avatar_url,
                        (t.author_id = $1) AS "is_mine!: bool"
                    FROM tweets t
                    JOIN users u ON u.unid = t.author_id
                    JOIN follows f ON f.following_id = t.author_id AND f.follower_id = $1
                    ORDER BY t.created_at DESC
                "#,
                current_user_id,
            )
            .fetch_all(pool)
            .await
        }

        pub async fn delete(pool: &PgPool, unid: Uuid) -> Result<Uuid, sqlx::Error> {
            let row = sqlx::query!(
                r#"
                    DELETE FROM tweets
                    WHERE unid = $1
                    RETURNING unid
                "#,
                unid
            )
            .fetch_one(pool)
            .await?;
            Ok(row.unid)
        }
    }
}
