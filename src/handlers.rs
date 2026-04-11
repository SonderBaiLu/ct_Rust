use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::error::AppError;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub is_published: bool,
}

#[derive(Deserialize, Debug)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}
pub async fn create_post(
     State(pool): State<PgPool>,
     Json(request): Json<CreatePostRequest>,
) -> Result<Json<BlogPost>, AppError> {
    let post = sqlx::query_as!(
        BlogPost,
        r#"insert into  blog_posts (title, content, tags, is_published) VALUES ($1, $2, $3, $4) RETURNING *"#,
        request.title,
        request.content,
        &request.tags,
        true
    )
        .fetch_one(&pool) // 拿着跑车钥匙去执行
        .await?;

    Ok(Json(post))
}

pub async fn get_post(State(pool): State<PgPool>) -> Result<Json<Vec<BlogPost>>, AppError> {
    let posts = sqlx::query_as!(BlogPost, r#"select * from blog_posts"#,)
        .fetch_all(&pool)
        .await?;
    Ok(Json(posts))
}