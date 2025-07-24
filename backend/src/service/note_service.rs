use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::{
        dtos::{CreateNoteRequest, PaginationQuery, UpdateNoteRequest},
        note::{self, Entity as Note},
    },
};

/// 获取笔记列表
pub async fn list_notes(
    db: &DatabaseConnection,
    query: &PaginationQuery,
) -> AppResult<(Vec<note::Model>, u64)> {
    let page = query.page.max(1);
    let per_page = query.per_page.min(100).max(1);

    let paginator = Note::find()
        .order_by_desc(note::Column::CreatedAt)
        .paginate(db, per_page);

    let total = paginator.num_items().await?;
    let notes = paginator.fetch_page(page - 1).await?;

    Ok((notes, total))
}

/// 创建笔记
pub async fn create_note(
    db: &DatabaseConnection,
    req: CreateNoteRequest,
    author_id: Uuid,
) -> AppResult<note::Model> {
    let new_note = note::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(req.title),
        content: Set(req.content),
        summary: Set(req.summary),
        author_id: Set(author_id),
        status: Set(req.status.unwrap_or_else(|| "published".to_string())),
        tags: Set(req.tags),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    let note = new_note.insert(db).await?;
    Ok(note)
}

/// 获取单个笔记
pub async fn get_note(db: &DatabaseConnection, note_id: Uuid) -> AppResult<note::Model> {
    let note = Note::find_by_id(note_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(note)
}

/// 更新笔记
pub async fn update_note(
    db: &DatabaseConnection,
    note_id: Uuid,
    req: UpdateNoteRequest,
    author_id: Uuid,
) -> AppResult<note::Model> {
    let note = Note::find_by_id(note_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    // 检查权限
    if note.author_id != author_id {
        return Err(AppError::Forbidden);
    }

    let mut active_note: note::ActiveModel = note.into();

    if let Some(title) = req.title {
        active_note.title = Set(title);
    }
    if let Some(content) = req.content {
        active_note.content = Set(content);
    }
    if let Some(summary) = req.summary {
        active_note.summary = Set(summary);
    }
    if let Some(status) = req.status {
        active_note.status = Set(status);
    }
    if let Some(tags) = req.tags {
        active_note.tags = Set(tags);
    }
    active_note.updated_at = Set(Utc::now());

    let updated_note = active_note.update(db).await?;
    Ok(updated_note)
}

/// 删除笔记
pub async fn delete_note(db: &DatabaseConnection, note_id: Uuid, author_id: Uuid) -> AppResult<()> {
    let note = Note::find_by_id(note_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    // 检查权限
    if note.author_id != author_id {
        return Err(AppError::Forbidden);
    }

    note.delete(db).await?;
    Ok(())
}

/// 点赞笔记
pub async fn like_note(db: &DatabaseConnection, note_id: Uuid, _user_id: Uuid) -> AppResult<()> {
    let note = Note::find_by_id(note_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut active_note: note::ActiveModel = note.into();
    active_note.likes_count = Set(active_note.likes_count.unwrap() + 1);
    active_note.updated_at = Set(Utc::now());

    active_note.update(db).await?;
    Ok(())
}

/// 取消点赞
pub async fn unlike_note(db: &DatabaseConnection, note_id: Uuid, _user_id: Uuid) -> AppResult<()> {
    let note = Note::find_by_id(note_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut active_note: note::ActiveModel = note.into();
    let current_likes = active_note.likes_count.unwrap().max(1);
    active_note.likes_count = Set(current_likes - 1);
    active_note.updated_at = Set(Utc::now());

    active_note.update(db).await?;
    Ok(())
}
