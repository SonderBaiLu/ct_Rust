// src/error.rs
// openclaw gateway start
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

// 定义枚举
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    // 使用 #[from] Rust 会自动把 sqlx::Error 转换成 AppError::Database
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    // 你可以随时在这里增加更多的错误类型，比如找不到文章、权限不足等
    #[error("客户端参数错误: {0}")]
    BadRequest(String),
    #[error("用户名或密码不能为空")]
    UsernameEmpty,
    #[error("用户名或密码错误")]
    InvalidOrPassword,
    #[error("未授权访问，请先登录")]
    Unauthorized,
}
// 告诉 Axum，遇到这个 AppError 时，该怎么发给前端
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 在后台终端打印真实的报错堆栈，方便你排查 BUG
        eprintln!("❌ 发生错误: {:?}", self);
        // 将不同的错误类型，映射成不同的 HTTP 状态码和友好的提示
        let (status, error_message) = match &self {
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "服务器开小差了，数据库操作失败",
            ),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::UsernameEmpty => (StatusCode::BAD_REQUEST, "用户名不能为空"),
            AppError::InvalidOrPassword => (StatusCode::BAD_REQUEST, "用户名或密码错误"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "未授权访问，请先登录"),
        };
        // 组装成 JSON 格式返回给前端
        let body = Json(json!({
            "success": false,
            "error": error_message
        }));
        // 返回最终的响应
        (status, body).into_response()
    }
}
