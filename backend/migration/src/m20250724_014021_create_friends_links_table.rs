use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FriendsLinks::Table)
                    .if_not_exists()
                    .col(pk_auto(FriendsLinks::Id))
                    .col(string_len(FriendsLinks::Name, 15).not_null())
                    .col(string_len(FriendsLinks::Url, 255).not_null().unique_key())
                    .col(string_len_null(FriendsLinks::Description, 100))
                    .col(string_len_null(FriendsLinks::LogoUrl, 255))
                    .col(integer(FriendsLinks::SortOrder).not_null())
                    .col(
                        timestamp_with_time_zone(FriendsLinks::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(FriendsLinks::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FriendsLinks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FriendsLinks {
    Table,
    Id,
    Name,
    Url,
    Description,
    LogoUrl,
    SortOrder,
    CreatedAt,
    UpdatedAt,
}
