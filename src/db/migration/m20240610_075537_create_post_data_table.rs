use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(PostData::Table)
                    .col(
                        ColumnDef::new(PostData::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostData::Project).char_len(64).not_null())
                    .col(ColumnDef::new(PostData::EventType).char_len(32).not_null())
                    .col(ColumnDef::new(PostData::Address).char_len(42).not_null())
                    .col(ColumnDef::new(PostData::Timestamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(PostData::SignMethod).char_len(32).not_null())
                    .col(ColumnDef::new(PostData::Sign).char_len(64).not_null())
                    .col(ColumnDef::new(PostData::Data).json())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostData::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostData {
    Table,
    Id,
    Project,
    EventType,
    Address,
    Timestamp,
    SignMethod,
    Sign,
    Data
}
