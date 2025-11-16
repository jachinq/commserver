// 用户相关接口
#![allow(dead_code, unused_variables)]

use actix_web::{HttpRequest, HttpResponse, Responder, web};
use lib_date::Calendar;
use serde::{Deserialize, Serialize};

use crate::{JsonResult, utils::auth::create_jwt, model::User, utils::auth::*};
use lib_sql::{sources::Dao, traits::CommInterface};

// 结构体
#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest();

// 登录请求结构体
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub remember: bool,
}

// 注册请求结构体
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

// 修改密码请求结构体
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// 重置密码请求结构体
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

// 登录响应结构体
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub token: String,
}

// 用户响应结构体（不包含密码）
#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
    pub created_at: String,
}

// Token 响应结构体
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar: None,
            created_at: Calendar::from_timestamp(user.created_at as i64).format_datetime(),
        }
    }
}

// 1.1 用户登录
pub async fn handle_login(req: web::Json<LoginRequest>) -> impl Responder {
    let dao = match Dao::new() {
        Ok(dao) => dao,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    // 查找用户
    let users = match dao.list_all::<User>() {
        Ok(users) => users,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    let user = users.iter().find(|u| u.username == req.username);
    if user.is_none() {
        return HttpResponse::InternalServerError()
            .json(JsonResult::<()>::error("Invalid username"));
    }

    let user = user.unwrap();

    // 验证密码
    if !verify_password(&req.password, &user.password) {
        return HttpResponse::InternalServerError()
            .json(JsonResult::<()>::error("Invalid password"));
    }

    // 生成 token 并返回用户信息
    let token = create_jwt(&user.id, req.remember);
    let response = LoginResponse {
        user: UserResponse::from(user.clone()),
        token,
    };

    HttpResponse::Ok().json(JsonResult::success(response))
}

// 1.2 用户注册
pub async fn handle_register(req: web::Json<RegisterRequest>) -> impl Responder {
    // 验证密码一致性
    if req.password != req.confirm_password {
        return HttpResponse::BadRequest().json(JsonResult::<()>::error("Passwords do not match"));
    }

    let dao = match Dao::new() {
        Ok(dao) => dao,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    // 检查用户名是否已存在
    let users = match dao.list_all::<User>() {
        Ok(users) => users,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    if users.iter().any(|u| u.username == req.username) {
        return HttpResponse::Conflict().json(JsonResult::<()>::error("Username already exists"));
    }

    if users.iter().any(|u| u.email == req.email) {
        return HttpResponse::Conflict().json(JsonResult::<()>::error("Email already exists"));
    }

    // 创建新用户
    let mut new_user = User {
        id: 0, // 数据库会自动生成
        username: req.username.clone(),
        password: hash_password(&req.password),
        email: req.email.clone(),
        created_at: Calendar::now().timestamp() as i32,
    };

    match dao.add(new_user.clone()) {
        Ok(_) => {
            // 获取新创建的用户 ID
            let users = dao.list_all::<User>().unwrap_or_default();
            if let Some(user) = users.iter().find(|u| u.username == req.username) {
                new_user.id = user.id;
            }

            let token = generate_token(new_user.id);
            let response = LoginResponse {
                user: UserResponse::from(new_user),
                token,
            };

            HttpResponse::Ok().json(JsonResult::success(response))
        }
        Err(e) => HttpResponse::InternalServerError().json(JsonResult::<()>::error(&format!(
            "Failed to create user: {:?}",
            e
        ))),
    }
}

// 1.3 获取当前用户信息
pub async fn handle_me(user: User) -> impl Responder {
    // 从中间件中获取当前用户 ID
    let user_id = user.id;

    let dao = match Dao::new() {
        Ok(dao) => dao,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    let result = dao.list_all::<User>();
    match result {
        Ok(list) => {
            // 使用从 token 中解析出来的用户 ID
            let user = list.iter().find(|u| u.id == user_id);
            if user.is_none() {
                return HttpResponse::NotFound().json(JsonResult::<()>::error("User not found"));
            }
            let user = user.unwrap().clone();
            let user_response = UserResponse::from(user);
            HttpResponse::Ok().json(JsonResult::success(user_response))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(JsonResult::<()>::error(&format!("Database error: {:?}", e))),
    }
}

// 1.4 用户登出
pub async fn handle_logout() -> impl Responder {
    // 简单实现，只返回成功消息，实际应将 token 加入黑名单
    HttpResponse::Ok().json(JsonResult::<()>::default())
}

// 1.5 刷新 Token
pub async fn handle_refresh(user: User) -> impl Responder {
    // 生成新的 token
    let token = generate_token(user.id);
    HttpResponse::Ok().json(JsonResult::success(TokenResponse { token }))
}

// 1.6 验证 Token
pub async fn handle_verify(req: HttpRequest) -> impl Responder {
    // 从 header 中获取 token，Authorization: Bearer <token>
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => {
            return HttpResponse::Unauthorized()
                .json(JsonResult::<()>::error("Missing Authorization header"));
        }
    };

    let auth_str = match auth_header.to_str() {
        Ok(str) => str,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .json(JsonResult::<()>::error("Invalid Authorization header"));
        }
    };

    // 检查是否以 "Bearer " 开头
    if !auth_str.starts_with("Bearer ") {
        return HttpResponse::Unauthorized()
            .json(JsonResult::<()>::error("Invalid Authorization format"));
    }

    // 提取 token
    let token = auth_str.strip_prefix("Bearer ").unwrap();

    // 验证 token 格式（应该是 user_id-timestamp 格式）
    let parts: Vec<&str> = token.split('-').collect();
    if parts.len() != 2 {
        return HttpResponse::Unauthorized().json(JsonResult::<()>::error("Invalid token format"));
    }

    // 解析用户 ID
    let user_id = match parts[0].parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .json(JsonResult::<()>::error("Invalid user ID in token"));
        }
    };

    // 解析时间戳
    let timestamp = match parts[1].parse::<i64>() {
        Ok(ts) => ts,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .json(JsonResult::<()>::error("Invalid timestamp in token"));
        }
    };

    // 检查 token 是否过期（这里设置24小时过期）
    let current_timestamp = Calendar::now().timestamp();
    let token_age = current_timestamp - timestamp;
    if token_age > 24 * 60 * 60 {
        // 24小时
        return HttpResponse::Unauthorized().json(JsonResult::<()>::error("Token expired"));
    }

    // 验证用户是否存在
    let dao = match Dao::new() {
        Ok(dao) => dao,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    let users = match dao.list_all::<User>() {
        Ok(users) => users,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    let user = users.iter().find(|u| u.id == user_id);
    if user.is_none() {
        return HttpResponse::Unauthorized().json(JsonResult::<()>::error("Invalid user"));
    }

    // Token 验证成功，返回原 token（或生成新的 token）
    let response = TokenResponse {
        token: token.to_string(),
    };
    HttpResponse::Ok().json(JsonResult::success(response))
}

// 1.7 修改密码
pub async fn handle_change_password(
    req: web::Json<ChangePasswordRequest>,
    user: User,
) -> impl Responder {
    let dao = match Dao::new() {
        Ok(dao) => dao,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    // 获取当前用户
    let users = match dao.list_all::<User>() {
        Ok(users) => users,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(JsonResult::<()>::error(&format!("Database error: {:?}", e)));
        }
    };

    let user = users.iter().find(|u| u.id == user.id);
    if user.is_none() {
        return HttpResponse::NotFound().json(JsonResult::<()>::error("User not found"));
    }

    let user = user.unwrap();

    // 验证旧密码
    if !verify_password(&req.old_password, &user.password) {
        return HttpResponse::InternalServerError().json(JsonResult::<()>::error("旧密码不正确"));
    }

    // 更新密码
    let mut updated_user = user.clone();
    updated_user.password = hash_password(&req.new_password);

    match dao.set(updated_user.clone()) {
        Ok(_) => HttpResponse::Ok().json(JsonResult::<()>::default()),
        Err(e) => HttpResponse::InternalServerError().json(JsonResult::<()>::error(&format!(
            "Failed to update password: {:?}",
            e
        ))),
    }
}

// 1.8 重置密码
pub async fn handle_reset_password(req: web::Json<ResetPasswordRequest>) -> impl Responder {
    // 简单实现，只返回成功消息，实际应发送邮件等
    HttpResponse::Ok().json(JsonResult::<()>::default())
}
