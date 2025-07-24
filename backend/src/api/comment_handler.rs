use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::{Value, json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    error::AppResult,
    models::{
        dtos::{CommentResponse, CreateCommentRequest, PaginationQuery, UpdateCommentRequest},
        user,
    },
    service::comment_service,
};

/// 创建评论
pub async fn create_comment(
    State(state): State<AppState>,
    user: user::Model,
    Json(payload): Json<CreateCommentRequest>,
) -> AppResult<Json<CommentResponse>> {
    payload.validate()?;

    let comment = comment_service::create_comment(&state.db, payload, user.id).await?;
    Ok(Json(CommentResponse::from(comment)))
}

/// 获取单个评论
pub async fn get_comment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<CommentResponse>> {
    let comment = comment_service::get_comment(&state.db, id).await?;
    Ok(Json(CommentResponse::from(comment)))
}

/// 更新评论
pub async fn update_comment(
    State(state): State<AppState>,
    user: user::Model,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCommentRequest>,
) -> AppResult<Json<CommentResponse>> {
    payload.validate()?;

    let comment = comment_service::update_comment(&state.db, id, payload, user.id).await?;
    Ok(Json(CommentResponse::from(comment)))
}

/// 删除评论
pub async fn delete_comment(
    State(state): State<AppState>,
    user: user::Model,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    comment_service::delete_comment(&state.db, id, user.id).await?;
    Ok(Json(json!({"message": "Comment deleted successfully"})))
}

/// 根据笔记获取评论列表
pub async fn list_comments_by_note(
    State(state): State<AppState>,
    Path(note_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Value>> {
    let (comments, total) =
        comment_service::list_comments_by_note(&state.db, note_id, &query).await?;

    let comment_responses: Vec<CommentResponse> =
        comments.into_iter().map(CommentResponse::from).collect();

    Ok(Json(json!({
        "comments": comment_responses,
        "total": total,
        "page": query.page,
        "per_page": query.per_page
    })))
}
