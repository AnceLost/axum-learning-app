use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "race_results")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub race_id: Uuid,
    pub horse_id: Uuid,
    pub jockey_id: Uuid,
    pub place: i32,
    pub finish_time: Option<i64>,

    #[sea_orm(belongs_to, from = "race_id", to = "id")]
    pub race: HasOne<super::races::Entity>,

    #[sea_orm(belongs_to, from = "horse_id", to = "id")]
    pub horse: HasOne<super::horses::Entity>,

    #[sea_orm(belongs_to, from = "jockey_id", to = "id")]
    pub jockey: HasOne<super::jockeys::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
