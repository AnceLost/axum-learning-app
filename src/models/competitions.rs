use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "competitions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: Option<String>,
    pub date: chrono::NaiveDate,
    pub time: chrono::NaiveTime,
    pub venue: String,

    #[sea_orm(has_many)]
    pub races: HasMany<super::races::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
