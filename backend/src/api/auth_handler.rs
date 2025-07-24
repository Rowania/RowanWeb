use axum::{
    Json,
    extract::State,
};

use crate::{
    AppState,
    error::AppResult,
    models::dtos::UserResponse,
};

/// 获取当前用户信息 (通过 SSH 认证中间件验证)
/// 此函数假设用户已通过 SSH 认证中间件验证
pub async fn get_current_user(
    State(_state): State<AppState>,
    user: crate::models::user::Model,
) -> AppResult<Json<UserResponse>> {
    Ok(Json(UserResponse::from(user)))
}
