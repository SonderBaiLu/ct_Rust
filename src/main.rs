use axum::{extract::State, Json, Router}; // 🌟 引入 State
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use sqlx::PgPool; // 🌟 引入 PgPool

#[derive(Serialize, Debug, sqlx::FromRow)]
struct BlogPost {
    id: i32,
    title: String,
    content: String,
    tags: Vec<String>,
    is_published: bool,
}

#[derive(Deserialize, Debug)]
struct CreatePostRequest {
    title: String,
    content: String,
    tags: Vec<String>,
}

async fn create_post(
    State(pool): State<PgPool>,
    Json(request): Json<CreatePostRequest>,
) -> Json<BlogPost> {
    let post = sqlx::query_as!(
        BlogPost,
        r#"insert into  blog_posts (title, content, tags, is_published) VALUES ($1, $2, $3, $4) RETURNING *"#,
        request.title,
        request.content,
        &request.tags,
        true
    )
        .fetch_one(&pool) // 拿着跑车钥匙去执行
        .await
        .expect("数据库插入失败");

    Json(post)
}

async fn get_post(State(pool): State<PgPool>) -> Json<Vec<BlogPost>> {
    let posts = sqlx::query_as!(
        BlogPost,
        r#"select * from blog_posts"#,
    ).fetch_all(&pool).await.expect("数据库查询失败");
    Json(posts)
}
// 🌟 批量插入博客文章
async fn bulk_insert_posts(State(pool): State<PgPool>) -> Json<Vec<BlogPost>> {
    let posts_data = vec![
        ("Rust 入门指南".to_string(), "Rust 是一种系统编程语言，注重安全性和性能。".to_string(), vec!["Rust".to_string(), "编程".to_string(), "入门".to_string()], true),
        ("Rust 所有权系统详解".to_string(), "所有权是 Rust 最独特的特性之一，确保内存安全。".to_string(), vec!["Rust".to_string(), "内存管理".to_string(), "所有权".to_string()], true),
        ("Rust 异步编程".to_string(), "使用 async/await 编写高性能的异步代码。".to_string(), vec!["Rust".to_string(), "异步".to_string(), "性能".to_string()], true),
        ("Rust Web 开发".to_string(), "使用 Axum 框架构建 Web 应用程序。".to_string(), vec!["Rust".to_string(), "Web".to_string(), "Axum".to_string()], true),
        ("Rust 与数据库交互".to_string(), "使用 SQLx 库进行数据库操作。".to_string(), vec!["Rust".to_string(), "数据库".to_string(), "SQLx".to_string()], true),
        ("Rust 测试驱动开发".to_string(), "学习如何为 Rust 代码编写测试。".to_string(), vec!["Rust".to_string(), "测试".to_string(), "TDD".to_string()], true),
        ("Rust 宏编程".to_string(), "深入了解 Rust 的宏系统。".to_string(), vec!["Rust".to_string(), "宏".to_string(), "元编程".to_string()], true),
        ("Rust 错误处理最佳实践".to_string(), "Result 和 Option 类型的正确使用方法。".to_string(), vec!["Rust".to_string(), "错误处理".to_string(), "最佳实践".to_string()], true),
        ("Rust 性能优化技巧".to_string(), "提升 Rust 代码性能的各种方法。".to_string(), vec!["Rust".to_string(), "性能".to_string(), "优化".to_string()], true),
        ("Rust 生态系统概览".to_string(), "介绍 Rust 社区中的主要库和工具。".to_string(), vec!["Rust".to_string(), "生态系统".to_string(), "工具链".to_string()], true),
    ];
    let mut inserted_posts = Vec::new();
    for (title, content, tags, is_published) in posts_data {
        let post = sqlx::query_as!(
            BlogPost,
            r#"insert into blog_posts (title, content, tags, is_published) VALUES ($1, $2, $3, $4) RETURNING *"#,
            title,
            content,
            &tags,
            is_published
        )
            .fetch_one(&pool)
            .await
            .expect("数据库插入失败");
        inserted_posts.push(post);
    }
    Json(inserted_posts)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("木有 DATABASE_URL");

    // 🌟 修正 1：用 let pool = 把连接池存下来
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&db_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/api/get_post/", get(get_post))
        .route("/api/create_post/", post(create_post))
        .route("/api/bulk_insert/", post(bulk_insert_posts))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 服务器已启动，请访问 http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}