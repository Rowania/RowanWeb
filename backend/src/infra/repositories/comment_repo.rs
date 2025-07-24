// 评论仓库层 - 数据访问接口
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        comment::{self, Entity as Comment},
        dtos::PaginationQuery,
    },
};

pub struct CommentRepository;

impl CommentRepository {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> AppResult<Option<comment::Model>> {
        let comment = Comment::find_by_id(id).one(db).await?;
        Ok(comment)
    }

    pub async fn find_by_note_id(
        db: &DatabaseConnection,
        note_id: Uuid,
        query: &PaginationQuery,
    ) -> AppResult<(Vec<comment::Model>, u64)> {
        let page = query.page.max(1);
        let per_page = query.per_page.min(100).max(1);

        let paginator = Comment::find()
            .filter(comment::Column::NoteId.eq(note_id))
            .filter(comment::Column::ParentId.is_null()) // 只获取顶级评论
            .order_by_asc(comment::Column::CreatedAt)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let comments = paginator.fetch_page(page - 1).await?;

        Ok((comments, total))
    }

    pub async fn find_replies(
        db: &DatabaseConnection,
        parent_id: Uuid,
    ) -> AppResult<Vec<comment::Model>> {
        let replies = Comment::find()
            .filter(comment::Column::ParentId.eq(parent_id))
            .order_by_asc(comment::Column::CreatedAt)
            .all(db)
            .await?;

        Ok(replies)
    }

    pub async fn find_by_author_id(
        db: &DatabaseConnection,
        author_id: Uuid,
        query: &PaginationQuery,
    ) -> AppResult<(Vec<comment::Model>, u64)> {
        let page = query.page.max(1);
        let per_page = query.per_page.min(100).max(1);

        let paginator = Comment::find()
            .filter(comment::Column::AuthorId.eq(author_id))
            .order_by_desc(comment::Column::CreatedAt)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let comments = paginator.fetch_page(page - 1).await?;

        Ok((comments, total))
    }
}
