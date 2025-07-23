use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::{
        comment::{self, Entity as Comment},
        dtos::{CreateCommentRequest, PaginationQuery, UpdateCommentRequest},
    },
};

/// 创建评论
pub async fn create_comment(
    db: &DatabaseConnection,
    req: CreateCommentRequest,
    author_id: Uuid,
) -> AppResult<comment::Model> {
    let new_comment = comment::ActiveModel {
        id: Set(Uuid::new_v4()),
        content: Set(req.content),
        note_id: Set(req.note_id),
        author_id: Set(author_id),
        parent_id: Set(req.parent_id),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    let comment = new_comment.insert(db).await?;
    Ok(comment)
}

/// 获取单个评论
pub async fn get_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
) -> AppResult<comment::Model> {
    let comment = Comment::find_by_id(comment_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(comment)
}

/// 更新评论
pub async fn update_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
    req: UpdateCommentRequest,
    author_id: Uuid,
) -> AppResult<comment::Model> {
    let comment = Comment::find_by_id(comment_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    // 检查权限
    if comment.author_id != author_id {
        return Err(AppError::Forbidden);
    }

    let mut active_comment: comment::ActiveModel = comment.into();
    active_comment.content = Set(req.content);
    active_comment.updated_at = Set(Utc::now());

    let updated_comment = active_comment.update(db).await?;
    Ok(updated_comment)
}

/// 删除评论
pub async fn delete_comment(
    db: &DatabaseConnection,
    comment_id: Uuid,
    author_id: Uuid,
) -> AppResult<()> {
    let comment = Comment::find_by_id(comment_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    // 检查权限
    if comment.author_id != author_id {
        return Err(AppError::Forbidden);
    }

    comment.delete(db).await?;
    Ok(())
}

/// 根据笔记获取评论列表
pub async fn list_comments_by_note(
    db: &DatabaseConnection,
    note_id: Uuid,
    query: &PaginationQuery,
) -> AppResult<(Vec<comment::Model>, u64)> {
    let page = query.page.max(1);
    let per_page = query.per_page.min(100).max(1);

    let paginator = Comment::find()
        .filter(comment::Column::NoteId.eq(note_id))
        .order_by_asc(comment::Column::CreatedAt)
        .paginate(db, per_page);

    let total = paginator.num_items().await?;
    let comments = paginator.fetch_page(page - 1).await?;

    Ok((comments, total))
}
