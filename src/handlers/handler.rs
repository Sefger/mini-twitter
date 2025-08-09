mod repository;




use axum::{extract::{Path, State},
           http::StatusCode,
           response::IntoResponse,
           Json};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    models::post::Post,

};

#[derive(Deserialize)]
pub struct CreatePostRequest {
    text: String,
    user_id: i32,
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub text: String,
}

pub async fn create_post(State(pool): State<PgPool>,
                         Json(payload): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let post = repository::post::Post::create(&pool, &payload.text, payload.user_id).await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error creating post: {}", e)
            )
        })?;
    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn get_post(State(pool): State<PgPool>, Path(id): Path<i32>) ->
Result<impl IntoResponse, (StatusCode, String)> {
    let post = repository::post::PostRepository::find_by_id(&pool, id)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SRVER_ERROR, format!("Error get post: {}", e))
        })?;
    match post {
        Some(post) => Ok((StatusCode::Ok, Json(post))),
        None => Err((StatusCode::NOT_FOUND, String::from("Post is not found")))
    }
}

pub async fn update_post(
    State(pool): State<PgPool>, Path(id): Path<i32>, Json(payload): Json(UpdatePostRequest),
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let post = repository::post::PostRepository::update(&pool, id, &payload.text)
        .await
        .map_err(|e|{
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error updating post: {}", e))
        })?;
    Ok((StatusCode::Ok, Json(post)))
}
pub async fn delete_post(
    State(pool):State<PgPool>,
    Path(id):Path<i32>
)-> Result<impl IntoResponse, (StatusCode, String)>{
    repository::post::PostRepository::delete(&pool, id)
        .await
        .map_err(|e|
        {
            (StatusCode::INTERNAL_SERVER_ERROR,
             format!("Error deleting post: {}", e))

        })?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn list_posts(State(pool):State<PgPool>)->Result<impl IntoResponse, (StatusCode, String)>{
    let posts = repository::post::Post::get_all(&pool)
        .await
        .map_err(|e|{
            (StatusCode::Internal_SERVER_ERROR,
             format!("Error get all: {}", e))
        })?;
    Ok((StatusCode::OK, Json(posts)))
}