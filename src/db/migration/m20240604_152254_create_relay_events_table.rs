use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(RelayEvents::Table)
                    .col(
                        ColumnDef::new(RelayEvents::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RelayEvents::EventType).char_len(32).not_null())
                    .col(ColumnDef::new(RelayEvents::TimeStamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(RelayEvents::Address).char_len(64).not_null())
                    .col(ColumnDef::new(RelayEvents::ProjectName).char_len(64).not_null())
                    .col(ColumnDef::new(RelayEvents::Sign).char_len(64).not_null())
                    .col(ColumnDef::new(RelayEvents::EventDate).date().not_null())
                    .col(ColumnDef::new(RelayEvents::Duration).integer())
                    .to_owned(),
            )
            .await


    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RelayEvents::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum RelayEvents {
    Table,
    Id,
    EventType,
    TimeStamp,
    Address,
    ProjectName,
    Sign,
    EventDate,
    Duration
}
