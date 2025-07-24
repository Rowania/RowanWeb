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
        dtos::{CreateNoteRequest, NoteResponse, PaginationQuery, UpdateNoteRequest},
        user,
    },
    service::note_service,
};

/// 获取笔记列表
pub async fn list_notes(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<Json<Value>> {
    let (notes, total) = note_service::list_notes(&state.db, &query).await?;

    let note_responses: Vec<NoteResponse> = notes.into_iter().map(NoteResponse::from).collect();

    Ok(Json(json!({
        "notes": note_responses,
        "total": total,
        "page": query.page,
        "per_page": query.per_page
    })))
}

/// 创建笔记
pub async fn create_note(
    State(state): State<AppState>,
    user: user::Model,
    Json(payload): Json<CreateNoteRequest>,
) -> AppResult<Json<NoteResponse>> {
    payload.validate()?;

    let note = note_service::create_note(&state.db, payload, user.id).await?;
    Ok(Json(NoteResponse::from(note)))
}

/// 获取单个笔记
pub async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<NoteResponse>> {
    let note = note_service::get_note(&state.db, id).await?;
    Ok(Json(NoteResponse::from(note)))
}

/// 更新笔记
pub async fn update_note(
    State(state): State<AppState>,
    user: user::Model,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNoteRequest>,
) -> AppResult<Json<NoteResponse>> {
    payload.validate()?;

    let note = note_service::update_note(&state.db, id, payload, user.id).await?;
    Ok(Json(NoteResponse::from(note)))
}

/// 删除笔记
pub async fn delete_note(
    State(state): State<AppState>,
    user: user::Model,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    note_service::delete_note(&state.db, id, user.id).await?;
    Ok(Json(json!({"message": "Note deleted successfully"})))
}

/// 点赞笔记
pub async fn like_note(
    State(state): State<AppState>,
    user: user::Model,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    note_service::like_note(&state.db, id, user.id).await?;
    Ok(Json(json!({"message": "Note liked successfully"})))
}

/// 取消点赞
pub async fn unlike_note(
    State(state): State<AppState>,
    user: user::Model,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    note_service::unlike_note(&state.db, id, user.id).await?;
    Ok(Json(json!({"message": "Note unliked successfully"})))
}
