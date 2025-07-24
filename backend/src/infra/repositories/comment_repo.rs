// 评论仓库层 - 数据访问接口
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::{
    error::AppResult,
    models::{
        comments::{self, Entity as Comment},
        dtos::PaginationQuery,
    },
};

pub struct CommentRepository;

impl CommentRepository {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> AppResult<Option<comments::Model>> {
        let comment = Comment::find_by_id(id).one(db).await?;
        Ok(comment)
    }

    pub async fn find_by_note_metadata_id(
        db: &DatabaseConnection,
        note_metadata_id: i32,
        query: &PaginationQuery,
    ) -> AppResult<(Vec<comments::Model>, u64)> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.page_size.unwrap_or(10).min(100).max(1);

        let paginator = Comment::find()
            .filter(comments::Column::NoteMetadataId.eq(note_metadata_id))
            .filter(comments::Column::ParentId.is_null()) // 只获取顶级评论
            .order_by_asc(comments::Column::CreatedAt)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let comments = paginator.fetch_page(page - 1).await?;

        Ok((comments, total))
    }

    pub async fn find_replies(
        db: &DatabaseConnection,
        parent_id: i32,
    ) -> AppResult<Vec<comments::Model>> {
        let replies = Comment::find()
            .filter(comments::Column::ParentId.eq(parent_id))
            .order_by_asc(comments::Column::CreatedAt)
            .all(db)
            .await?;

        Ok(replies)
    }

    pub async fn find_by_visitor_profile_id(
        db: &DatabaseConnection,
        visitor_profile_id: i32,
        query: &PaginationQuery,
    ) -> AppResult<(Vec<comments::Model>, u64)> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.page_size.unwrap_or(10).min(100).max(1);

        let paginator = Comment::find()
            .filter(comments::Column::VisitorProfileId.eq(visitor_profile_id))
            .order_by_desc(comments::Column::CreatedAt)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let comments = paginator.fetch_page(page - 1).await?;

        Ok((comments, total))
    }

    pub async fn create(
        db: &DatabaseConnection,
        comment_data: comments::ActiveModel,
    ) -> AppResult<comments::Model> {
        let comment = comment_data.insert(db).await?;
        Ok(comment)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        comment_data: comments::ActiveModel,
    ) -> AppResult<Option<comments::Model>> {
        let comment = Comment::find_by_id(id).one(db).await?;
        if let Some(_comment) = comment {
            let updated_comment = comment_data.update(db).await?;
            Ok(Some(updated_comment))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> AppResult<bool> {
        let result = Comment::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }
}
