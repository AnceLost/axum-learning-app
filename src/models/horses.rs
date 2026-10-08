use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use super::enums::HorseGender;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "horses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub gender: HorseGender,
    pub age: i32,
    pub owner_id: Uuid,

    // Внешний ключ → владелец
    #[sea_orm(belongs_to, from = "owner_id", to = "id")]
    pub owner: HasOne<super::owners::Entity>,

    #[sea_orm(has_many)]
    pub race_results: HasMany<super::race_results::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}