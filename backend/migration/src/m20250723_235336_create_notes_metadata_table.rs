use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NotesMetadata::Table)
                    .if_not_exists()
                    .col(pk_auto(NotesMetadata::Id))
                    .col(ColumnDef::new(NotesMetadata::FileId).uuid().not_null().unique_key())//使用FileId来存储
                    .col(string_len(NotesMetadata::Slug,255).not_null().unique_key())
                    .col(string_len(NotesMetadata::Title,255).not_null())
                    .col(text_null(NotesMetadata::Summary))
                    .col(
                        timestamp_with_time_zone(NotesMetadata::PublishedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(NotesMetadata::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(NotesMetadata::Views).not_null().default(0))
                    .col(integer(NotesMetadata::LikesCount).not_null().default(0))
                    .col(text_null(NotesMetadata::Tags))
                    .col(text_null(NotesMetadata::Category))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NotesMetadata::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NotesMetadata {
    Table,
    Id,
    FileId,
    Slug,
    Title,
    Summary,
    PublishedAt,
    UpdatedAt,
    Views,
    LikesCount, 
    Tags,
    Category,
}
