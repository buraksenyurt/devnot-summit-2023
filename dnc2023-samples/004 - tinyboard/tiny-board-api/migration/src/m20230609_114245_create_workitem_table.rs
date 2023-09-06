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
                    .table(WorkItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkItem::Title).string().not_null())
                    .col(ColumnDef::new(WorkItem::Summary).string().not_null())
                    .col(ColumnDef::new(WorkItem::BusinessValue).integer().not_null())
                    .col(ColumnDef::new(WorkItem::Completed).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(WorkItem::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum WorkItem {
    Table,
    Id,
    Title,
    Summary,
    BusinessValue,
    Completed,
}
