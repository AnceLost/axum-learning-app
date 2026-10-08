use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::enums::RaceStatus;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "races")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub competition_id: Uuid,
    pub name: Option<String>,
    pub status: RaceStatus,

    #[sea_orm(belongs_to, from = "competition_id", to = "id")]
    pub competition: HasOne<super::competitions::Entity>,

    #[sea_orm(has_many)]
    pub race_results: HasMany<super::race_results::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}