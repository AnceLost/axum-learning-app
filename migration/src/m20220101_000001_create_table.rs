use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // НАШИ ENUM
        manager
            .create_type(
                Type::create()
                    .as_enum(HorseGender::Type)
                    .values([
                        HorseGender::Mare,
                        HorseGender::Stallion,
                        HorseGender::Gelding,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(RaceStatus::Type)
                    .values([
                        RaceStatus::Scheduled,
                        RaceStatus::Finished,
                        RaceStatus::Cancelled,
                    ])
                    .to_owned(),
            )
            .await?;

        // НАШИ ТАБЛИЦЫ
        manager
            .create_table(
                Table::create()
                    .table(Owners::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Owners::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Owners::Name).string().not_null())
                    .col(ColumnDef::new(Owners::Address).string())
                    .col(ColumnDef::new(Owners::Phone).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Horses::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Horses::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Horses::Name).string().not_null())
                    .col(
                        ColumnDef::new(Horses::Gender)
                            .enumeration(
                                HorseGender::Type,
                                [
                                    HorseGender::Mare,
                                    HorseGender::Stallion,
                                    HorseGender::Gelding,
                                ],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Horses::Age).integer().not_null())
                    .col(ColumnDef::new(Horses::OwnerId).uuid().not_null())
                    .check(Expr::col(Horses::Age).gt(0))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Horses::Table, Horses::OwnerId)
                            .to(Owners::Table, Owners::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Jockeys::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Jockeys::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Jockeys::Name).string().not_null())
                    .col(ColumnDef::new(Jockeys::Address).string())
                    .col(ColumnDef::new(Jockeys::Age).integer())
                    .col(ColumnDef::new(Jockeys::Rating).integer().default(0))
                    .check(Expr::col(Jockeys::Age).gt(0))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Competitions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Competitions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Competitions::Name).string())
                    .col(ColumnDef::new(Competitions::Date).date().not_null())
                    .col(ColumnDef::new(Competitions::Time).time().not_null())
                    .col(ColumnDef::new(Competitions::Venue).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Races::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Races::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Races::CompetitionId).uuid().not_null())
                    .col(ColumnDef::new(Races::Name).string())
                    .col(
                        ColumnDef::new(Races::Status)
                            .enumeration(
                                RaceStatus::Type,
                                [
                                    RaceStatus::Scheduled,
                                    RaceStatus::Finished,
                                    RaceStatus::Cancelled,
                                ],
                            )
                            .not_null()
                            .default("SCHEDULED"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Races::Table, Races::CompetitionId)
                            .to(Competitions::Table, Competitions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RaceResults::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RaceResults::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RaceResults::RaceId).uuid().not_null())
                    .col(ColumnDef::new(RaceResults::HorseId).uuid().not_null())
                    .col(ColumnDef::new(RaceResults::JockeyId).uuid().not_null())
                    .col(ColumnDef::new(RaceResults::Place).integer().not_null())
                    .col(ColumnDef::new(RaceResults::FinishTime).big_integer())
                    .check(Expr::col(RaceResults::Place).gt(0))
                    .index(
                        Index::create()
                            .name("idx-raceresults-raceid-horseid-unique")
                            .col(RaceResults::RaceId)
                            .col(RaceResults::HorseId)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("idx-raceresults-raceid-jockeyid-unique")
                            .col(RaceResults::RaceId)
                            .col(RaceResults::JockeyId)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("idx-raceresults-raceid-place-unique")
                            .col(RaceResults::RaceId)
                            .col(RaceResults::Place)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Удаляем таблицы в обратном порядке
        manager
            .drop_table(Table::drop().table(RaceResults::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Races::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Competitions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Horses::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Jockeys::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Owners::Table).to_owned())
            .await?;

        // Удаляем ENUM типы
        manager
            .drop_type(Type::drop().name(HorseGender::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(RaceStatus::Type).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Owners {
    Table,
    Id,
    Name,
    Address,
    Phone,
}
#[derive(DeriveIden)]
enum Horses {
    Table,
    Id,
    Name,
    Gender,
    Age,
    OwnerId,
}
#[derive(DeriveIden)]
enum Jockeys {
    Table,
    Id,
    Name,
    Address,
    Age,
    Rating,
}
#[derive(DeriveIden)]
enum Competitions {
    Table,
    Id,
    Name,
    Date,
    Time,
    Venue,
}
#[derive(DeriveIden)]
enum Races {
    Table,
    Id,
    CompetitionId,
    Name,
    Status,
}
#[derive(DeriveIden)]
enum RaceResults {
    Table,
    Id,
    RaceId,
    HorseId,
    JockeyId,
    Place,
    FinishTime,
}
#[derive(DeriveIden)]
enum HorseGender {
    #[sea_orm(iden = "horse_gender")]
    Type,
    #[sea_orm(iden = "MARE")]
    Mare,
    #[sea_orm(iden = "STALLION")]
    Stallion,
    #[sea_orm(iden = "GELDING")]
    Gelding,
}

#[derive(DeriveIden)]
enum RaceStatus {
    #[sea_orm(iden = "race_status")]
    Type,
    #[sea_orm(iden = "SCHEDULED")]
    Scheduled,
    #[sea_orm(iden = "FINISHED")]
    Finished,
    #[sea_orm(iden = "CANCELLED")]
    Cancelled,
}
