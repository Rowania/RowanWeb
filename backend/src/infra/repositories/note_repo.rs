// 笔记仓库层 - 数据访问接口
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::{
        dtos::PaginationQuery,
        note::{self, Entity as Note},
    },
};

pub struct NoteRepository;

impl NoteRepository {
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> AppResult<Option<note::Model>> {
        let note = Note::find_by_id(id).one(db).await?;
        Ok(note)
    }

    pub async fn find_by_author_id(
        db: &DatabaseConnection,
        author_id: Uuid,
        query: &PaginationQuery,
    ) -> AppResult<(Vec<note::Model>, u64)> {
        let page = query.page.max(1);
        let per_page = query.per_page.min(100).max(1);

        let paginator = Note::find()
            .filter(note::Column::AuthorId.eq(author_id))
            .order_by_desc(note::Column::CreatedAt)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let notes = paginator.fetch_page(page - 1).await?;

        Ok((notes, total))
    }

    pub async fn find_published(
        db: &DatabaseConnection,
        query: &PaginationQuery,
    ) -> AppResult<(Vec<note::Model>, u64)> {
        let page = query.page.max(1);
        let per_page = query.per_page.min(100).max(1);

        let paginator = Note::find()
            .filter(note::Column::Status.eq("published"))
            .order_by_desc(note::Column::CreatedAt)
            .paginate(db, per_page);

        let total = paginator.num_items().await?;
        let notes = paginator.fetch_page(page - 1).await?;

        Ok((notes, total))
    }
}
