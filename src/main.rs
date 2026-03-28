use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct BlogPost{
    id: i32,
    title: String,
    content: String,
    tags: Vec<String>, // (标签数组，比如 ["Rust", "Web"])
    is_published: bool,
}
async fn get_post() -> Json<BlogPost>{
    Json(BlogPost{
        id: 1,
        title: "Rust 入门".to_string(),
        content: "这是一个关于 Rust 的入门教程".to_string(),
        tags: vec!["Rust".to_string(), "Web".to_string()],
        is_published: true,
    })
}
