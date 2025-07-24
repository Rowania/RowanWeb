use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Likes::Table)
                    .if_not_exists()
                    .col(pk_auto(Likes::Id))
                    .col(integer(Likes::NoteMetadataId).not_null())
                    .col(string_len(Likes::IpAddress, 45).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-likes-note_metadata_id")
                            .from(Likes::Table, Likes::NoteMetadataId)
                            .to(NotesMetadata::Table, NotesMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_likes_noteid_ip_unique")
                    .table(Likes::Table)
                    .col(Likes::NoteMetadataId)
                    .col(Likes::IpAddress)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_likes_noteid_ip_unique").table(Likes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Likes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Likes {
    Table,
    Id,
    NoteMetadataId,
    IpAddress,
}

#[derive(DeriveIden)]
enum NotesMetadata {
    Table,
    Id,
}