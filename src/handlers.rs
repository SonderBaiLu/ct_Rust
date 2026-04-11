use crate::error::AppError;
use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Debug, sqlx::FromRow)]
// 博客文章结构体
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub is_published: bool,
}
// 更新博客文章请求体
#[derive(Deserialize, Debug)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}
// 创建博客文章请求体
#[derive(Deserialize, Debug)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}
// 创建博客文章处理函数
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
// 获取博客文章处理函数
pub async fn get_post(State(pool): State<PgPool>) -> Result<Json<Vec<BlogPost>>, AppError> {
    let posts = sqlx::query_as!(BlogPost, r#"select * from blog_posts"#,)
        .fetch_all(&pool)
        .await?;
    Ok(Json(posts))
}
// 更新博客文章处理函数
// 注意参数里的 Path(id): Path<i32>，它会自动把网址里的 /api/post/5 里的 5 抠出来变成数字！
pub async fn update_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(request): Json<UpdatePostRequest>,
) -> Result<Json<BlogPost>, AppError> {
    let post = sqlx::query_as!(
        BlogPost,
        r#"
update blog_posts set title = $1, content =$2, tags = $3 where id = $4 returning *
"#,
        request.title,
        request.content,
        &request.tags,
        id
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(post))
}
// 删除博客文章处理函数
pub async fn delete_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query!(r#"delete from blog_posts where id = $1"#, id)
        .bind(id)
        .execute(&pool)
        .await?;
    // 删除成功后，没有具体的文章可以返回了，我们就手动拼一个友好的 JSON 返回去
    Ok(Json::from(
        serde_json::json!({"success": true, "message": format!("文章 {} 已经被彻底抹杀", id)}),
    ))
}
