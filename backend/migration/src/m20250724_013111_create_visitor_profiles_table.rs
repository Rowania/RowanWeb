use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VisitorProfiles::Table)
                    .if_not_exists()
                    .col(pk_auto(VisitorProfiles::Id))
                     .col(string_len(VisitorProfiles::CookieId, 40).not_null().unique_key())
                    .col(string_len(VisitorProfiles::Name, 13).not_null().unique_key())
                    .col(string_len(VisitorProfiles::Ip, 45).null())
                    .col(
                        timestamp_with_time_zone(VisitorProfiles::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(VisitorProfiles::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VisitorProfiles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum VisitorProfiles {
    Table,
    Id,
    CookieId,
    Name,
    Ip,
    CreatedAt,
    UpdatedAt,
}
