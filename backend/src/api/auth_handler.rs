use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};
use validator::Validate;

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::dtos::{LoginRequest, RegisterRequest, UserResponse},
    service::auth_service,
};

/// 用户注册
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<Value>> {
    payload.validate()?;

    let user = auth_service::register(&state.db, payload).await?;
    let token = auth_service::generate_token(&user, &state.config.jwt)?;

    Ok(Json(json!({
        "user": UserResponse::from(user),
        "token": token,
        "message": "User registered successfully"
    })))
}

/// 用户登录
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<Value>> {
    payload.validate()?;

    let user = auth_service::login(&state.db, payload).await?;
    let token = auth_service::generate_token(&user, &state.config.jwt)?;

    Ok(Json(json!({
        "user": UserResponse::from(user),
        "token": token,
        "message": "Login successful"
    })))
}

/// 获取当前用户信息
pub async fn get_current_user(
    State(state): State<AppState>,
    user: crate::models::user::Model,
) -> AppResult<Json<UserResponse>> {
    Ok(Json(UserResponse::from(user)))
}

/// 刷新令牌
pub async fn refresh_token(
    State(state): State<AppState>,
    user: crate::models::user::Model,
) -> AppResult<Json<Value>> {
    let token = auth_service::generate_token(&user, &state.config.jwt)?;

    Ok(Json(json!({
        "token": token,
        "message": "Token refreshed successfully"
    })))
}
