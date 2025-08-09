use sqlx::{PgPool, postgres::PgQueryResult};
use chrono::Utc;
use crate::models::post::Post;

pub struct PostRepository;

impl PostRepository {
    pub async fn create(pool: &PgPool, text: &str, user_id: i32) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (user_id, text, created_at, updated_at, is_updated)
            VALUES ($1, $2, $3, $3, false)
            RETURNING id, user_id as id_user, text, created_at, updated_at as update_at, is_updated as is_update
            "#,
            user_id,
            text,
            Utc::now()
        )
            .fetch_one(pool)
            .await?;

        Ok(post)
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Post>, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id, user_id as id_user, text, created_at, updated_at as update_at, is_updated as is_update
            FROM posts
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(pool)
            .await?;

        Ok(post)
    }

    pub async fn update(pool: &PgPool, id: i32, text: &str) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET text = $1, updated_at = $2, is_updated = true
            WHERE id = $3
            RETURNING id, user_id as id_user, text, created_at, updated_at as update_at, is_updated as is_update
            "#,
            text,
            Utc::now(),
            id
        )
            .fetch_one(pool)
            .await?;

        Ok(post)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "DELETE FROM posts WHERE id = $1",
            id
        )
            .execute(pool)
            .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, user_id as id_user, text, created_at, updated_at as update_at, is_updated as is_update
            FROM posts
            ORDER BY created_at DESC
            "#
        )
            .fetch_all(pool)
            .await?;

        Ok(posts)
    }
}