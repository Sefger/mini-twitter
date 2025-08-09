use chrono::NaiveDate;
use crate::models::{user::User, post::Post, comments::Comments};
pub struct UserRepository {
    pool: sqlx::PgPool,
}

impl UserRepository {
    pub async fn create_user(
        &self,
        name: &str,
        surname: &str,
        nickname: &str,
        birthday_date: NaiveDate,
        status: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, surname, nickname, birthday_date, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            name,
            surname,
            nickname,
            birthday_date,
            status
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn add_post(&self, user_id: i32, text: &str) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (user_id, text)
            VALUES ($1, $2)
            RETURNING *
            "#,
            user_id,
            text
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(post)
    }

    pub async fn update_post(
        &self,
        user_id: i32,
        post_id: i32,
        text: &str,
    ) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET text = $1, updated_at = NOW(), is_updated = TRUE
            WHERE id = $2 AND user_id = $3
            RETURNING *
            "#,
            text,
            post_id,
            user_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(post)
    }

    pub async fn add_comment(
        &self,
        user_id: i32,
        post_id: i32,
        text: &str,
    ) -> Result<Comments, sqlx::Error> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (user_id, post_id, text)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            user_id,
            post_id,
            text
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(comment)
    }
}