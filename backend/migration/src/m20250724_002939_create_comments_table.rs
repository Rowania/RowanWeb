use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .if_not_exists()
                    .col(pk_auto(Comments::Id))
                    .col(integer(Comments::NoteMetadataId).not_null())
                    .col(integer(Comments::VisitorProfileId).not_null()) 
                    .col(text(Comments::Content).not_null())
                    .col(integer_null(Comments::ParentId))
                    .col(
                        timestamp_with_time_zone(Comments::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(boolean(Comments::IsApproved).not_null().default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comments-note_metadata_id")
                            .from(Comments::Table, Comments::NoteMetadataId) 
                            .to(NotesMetadata::Table, NotesMetadata::Id)     
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comments-parent_id")
                            .from(Comments::Table, Comments::ParentId) 
                            .to(Comments::Table, Comments::Id)         
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key( 
                        ForeignKey::create()
                            .name("fk-comments-visitor_profile_id")
                            .from(Comments::Table, Comments::VisitorProfileId)
                            .to(VisitorProfiles::Table, VisitorProfiles::Id)
                            .on_delete(ForeignKeyAction::Restrict), // 防止删除有评论关联的访客资料
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    NoteMetadataId,
    VisitorProfileId,
    Content,
    ParentId,
    CreatedAt,
    IsApproved,
}

#[derive(DeriveIden)]
enum NotesMetadata{
    Table,
    Id,
}

#[derive(DeriveIden)]
enum VisitorProfiles{
    Table,
    Id,
}