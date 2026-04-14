// src/auth.rs
use crate::error::AppError;
use axum::{Json, extract::State};
use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use chrono::{Duration, Utc}; // 🌟 引入时间工具
use jsonwebtoken::{DecodingKey, Validation, decode};
use jsonwebtoken::{EncodingKey, Header, encode}; // 🌟 引入 JWT 工具
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// 📦 1. 接收前端登录请求
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// 📦 2. 登录成功后，发给前端的 VIP 通行证响应纸盒
#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub message: String,
}

// 📦 3. 写在 VIP 通行证（JWT）里面的核心信息（Claims）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject: 存用户名
    pub exp: usize,  // expiration: 存过期时间 (时间戳)
}

// 📦 4. 用来接数据库数据的纸盒
#[derive(sqlx::FromRow)]
struct User {
    // id: i32, // 如果这里不用可以不写，避免警告
    // username: String,
    password_hash: String,
}
// 检查是否包含 admin 字符串
fn validate_username(username: &str) -> bool{
    username.contains("admin")
}
// 🚧 员工 C：负责处理登录
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    tracing::info!(
        "🔍 收到请求: 账号='{}', 密码='{}'",
        payload.username,
        payload.password
    );
    let real_hash = bcrypt::hash("123456", 4).unwrap();
    tracing::info!("🔐 真正的 123456 哈希乱码是: {}", real_hash);
    // 【防线 1：参数判空】
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AppError::BadRequest("用户名或密码不能为空".to_string()));
    }
    tracing::info!("🔍 开始查询数据库，检查账号是否存在...");
    let user = sqlx::query_as!(
        User,
        r#"SELECT password_hash FROM users WHERE username = $1"#,
        &payload.username
    )
    .fetch_optional(&pool)
    .await?;
    if user.is_none() {
        tracing::error!(
            "❌ 破案了：数据库里根本没找到 [{}] 这个账号！",
            payload.username
        );
    } else {
        tracing::info!("✅ 数据库查到账号了，准备进入 bcrypt 验钞机...");
    }

    // 【防线 3：拆开纸盒，核对密码并印发通行证】
    tracing::info!("🔍 开始验证密码...");
    match user {
        // 情况 A：数据库里查到了这个人
        Some(db_user) => {
            // 核对密码
            let is_valid =
                bcrypt::verify(&payload.password, &db_user.password_hash).unwrap_or(false);
            println!("💡 验钞机比对结果: {}", is_valid);

            if is_valid {
                // 密码正确，开始印发通行证
                let expiration = Utc::now()
                    .checked_add_signed(Duration::hours(24))
                    .expect("时间计算错误")
                    .timestamp() as usize;

                let claims = &Claims {
                    sub: payload.username, // 消耗掉 payload 里的 username
                    exp: expiration,
                };

                // 生成 Token 字符串
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(b"my_super_secret"),
                )
                .expect("Token 生成失败");

                // 打包返回给前端
                Ok(Json(AuthResponse {
                    token,
                    message: "登录成功，欢迎回来！".to_string(),
                }))
            } else {
                // 密码错误
                Err(AppError::BadRequest("用户名或密码错误".to_string()))
            }
        }
        // 情况 B：数据库里压根没这个人
        None => Err(AppError::BadRequest("用户名或密码错误".to_string())),
    }
}

// 🛡️ 铁面保安上岗：实现特征，让 Claims 变成提取器
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    // 如果查票失败，就掏出我们刚才升级的电击枪
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 连环动作：去请求头里找 -> 尝试转成字符串 -> 确保是以 "Bearer " 开头的
        let auth_str = if let Some(header_value) = parts.headers.get(AUTHORIZATION) {
            if let Ok(str_value) = header_value.to_str() {
                str_value // 成功
            } else {
                return Err(AppError::Unauthorized); // 包含非法字符，踢飞！
            }
        } else {
            return Err(AppError::Unauthorized); // 压根没带请求头，滚蛋！
        };
        // 查验格式并剪裁
        if !auth_str.starts_with("Bearer ") {
            println!("❌ 通行证格式不对，不是 'Bearer ({}) ' 开头的！", auth_str);
            return Err(AppError::Unauthorized); // 格式不对，踢飞！
        }
        // 完美拿到极其纯净的 token 乱码！
        let token = auth_str.trim_start_matches("Bearer ");
        println!("🔍 发现通行证: {}", token);

        // 3. 验钞机启动：用 jsonwebtoken::decode 来验证这串乱码！
        // （提示：需要用到 DecodingKey::from_secret(b"my_super_secret") 和 Validation::default()）
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(b"my_super_secret"),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;
        // 4. 放行或踢飞：
        // - 如果验证成功，拿到里面的 claims 数据，用 Ok(claims) 放行！
        // - 如果中间任何一步出错了（没带头、格式不对、解析失败），统统 return Err(AppError::Unauthorized);
        Ok(token_data.claims)
    }
}
