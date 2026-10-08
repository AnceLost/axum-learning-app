use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    dto::{CreateRace, ParticipantDto},
    error::AppResult,
    models::{enums::RaceStatus, horses, jockeys, race_results, races},
    state::AppState,
};

pub async fn create_race(
    State(state): State<AppState>,
    Path(competition_id): Path<Uuid>,
    Json(payload): Json<CreateRace>,
) -> AppResult<(StatusCode, Json<races::Model>)> {
    let race = races::ActiveModel {
        id: Set(Uuid::new_v4()),
        competition_id: Set(competition_id),
        name: Set(payload.name),
        status: Set(RaceStatus::Scheduled),
        ..Default::default()
    };
    let race = race.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(race)))
}

pub async fn race_results_list(
    State(state): State<AppState>,
    Path(race_id): Path<Uuid>,
) -> AppResult<Json<Vec<ParticipantDto>>> {
    let results = race_results::Entity::find()
        .filter(race_results::Column::RaceId.eq(race_id))
        .find_also_related(horses::Entity)
        .find_also_related(jockeys::Entity)
        .all(&state.db)
        .await?;

    let dtos = results
        .into_iter()
        .map(|(r, horse, jockey)| ParticipantDto {
            race_id: r.race_id,
            race_name: None,
            place: r.place,
            finish_time: r.finish_time.map(|ms| ms as f64 / 1000.0),
            horse_id: r.horse_id,
            horse_name: horse.map(|h| h.name).unwrap_or_default(),
            jockey_id: r.jockey_id,
            jockey_name: jockey.map(|j| j.name).unwrap_or_default(),
        })
        .collect();

    Ok(Json(dtos))
}