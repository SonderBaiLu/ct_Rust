mod error;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
mod auth;   // 新增：导入auth模块
mod handlers;// 新增：导入handlers模块
use axum::routing::{get, post, put, delete}; // 新增delete方法
use crate::auth::login; // 修正：从当前crate导入
use crate::handlers::{create_post, get_post, update_post, delete_post}; // 修正
#[tokio::main]
async fn main() {
    // 🌟 去掉返回值
    dotenvy::dotenv().ok();

    // 启动阶段的错误，直接原地爆炸并给出清晰的报错
    let db_url = std::env::var("DATABASE_URL").expect("致命错误：找不到 DATABASE_URL 环境变量！");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("致命错误：无法连接到 PostgreSQL 数据库！");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/login", post(login))
        .route("/api/get_post/", get(get_post))
        .route("/api/create_post/", post(create_post))
        .route("/api/update_post/{id}", put(update_post))
        .route("/api/delete_post/{id}", delete(delete_post))
        .with_state(pool)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("致命错误：3000 端口被占用！");

    println!("🚀 服务器已启动，请访问 http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("致命错误：Web 服务器意外崩溃！");
}
