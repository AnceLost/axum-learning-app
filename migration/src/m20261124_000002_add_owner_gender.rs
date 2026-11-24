use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Колонка nullable: у существующих владельцев пол останется NULL,
        // поэтому миграция безопасна для уже заполненной таблицы.
        manager
            .alter_table(
                Table::alter()
                    .table(Owners::Table)
                    .add_column(ColumnDef::new(Owners::Gender).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Owners::Table)
                    .drop_column(Owners::Gender)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Owners {
    Table,
    Gender,
}
