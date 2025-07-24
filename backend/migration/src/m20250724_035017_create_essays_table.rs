use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Essays::Table)
                    .if_not_exists()
                    .col(pk_auto(Essays::Id))
                    .col(string_len(Essays::Title, 20).null())
                    .col(text(Essays::Content).not_null())
                    .col(
                        timestamp_with_time_zone(Essays::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .col(
                        timestamp_with_time_zone(Essays::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp())
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Essays::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Essays {
    Table,
    Id,
    Title,
    Content,
    CreatedAt,
    UpdatedAt,
}
