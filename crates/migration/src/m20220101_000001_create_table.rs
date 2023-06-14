use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(File::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(File::Name).string().not_null())
                    .col(ColumnDef::new(File::Path).string().not_null())
                    .col(
                        ColumnDef::new(File::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Modification::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Modification::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Modification::Timestamp)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .col(
                        ColumnDef::new(Modification::Hash)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Modification::FileId).integer().not_null())
                    .col(ColumnDef::new(Modification::PreviousId).integer().null())
                    .col(ColumnDef::new(Modification::Content).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("modification_file_id")
                            .from(Modification::Table, Modification::FileId)
                            .to(File::Table, File::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("modification_previous_id")
                            .from(Modification::Table, Modification::PreviousId)
                            .to(Modification::Table, Modification::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Modification::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum File {
    Table,
    Id,
    Name,
    Path,
    CreatedAt,
}

#[derive(Iden)]
enum Modification {
    Table,
    Id,
    Timestamp,
    Hash,
    FileId,
    PreviousId,
    Content,
}
