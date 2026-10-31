use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::enums::HorseGender;

#[derive(Deserialize)]
pub struct CreateOwner {
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateHorse {
    pub name: String,
    pub gender: HorseGender,
    pub age: i32,
    pub owner_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateJockey {
    pub name: String,
    pub address: Option<String>,
    pub age: Option<i32>,
    pub rating: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateCompetition {
    pub name: Option<String>,
    pub date: chrono::NaiveDate,
    pub time: chrono::NaiveTime,
    pub venue: String,
}

#[derive(Deserialize)]
pub struct CreateRace {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct RaceResultInput {
    pub horse_id: Uuid,
    pub jockey_id: Uuid,
    pub place: i32,
    /// Время в формате секунд (например, 123.456)
    pub finish_time: Option<f64>,
}

// Ответ для "участники заезда/состязания"
#[derive(Serialize)]
pub struct ParticipantDto {
    pub race_id: Uuid,
    pub race_name: Option<String>,
    pub place: i32,
    pub finish_time: Option<f64>,
    pub horse_id: Uuid,
    pub horse_name: String,
    pub jockey_id: Uuid,
    pub jockey_name: String,
}
