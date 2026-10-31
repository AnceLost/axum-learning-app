use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "owners")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,

    #[sea_orm(has_many)]
    pub horses: HasMany<super::horses::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
