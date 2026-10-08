use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "jockeys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub age: Option<i32>,
    pub rating: i32,

    #[sea_orm(has_many)]
    pub race_results: HasMany<super::race_results::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}