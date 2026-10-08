use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "horse_gender")]
#[serde(rename_all = "UPPERCASE")]
pub enum HorseGender {
    #[sea_orm(string_value = "MARE")]
    Mare,
    #[sea_orm(string_value = "STALLION")]
    Stallion,
    #[sea_orm(string_value = "GELDING")]
    Gelding,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "race_status")]
#[serde(rename_all = "UPPERCASE")]
pub enum RaceStatus {
    #[sea_orm(string_value = "SCHEDULED")]
    Scheduled,
    #[sea_orm(string_value = "FINISHED")]
    Finished,
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,
}