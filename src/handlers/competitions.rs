use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    dto::{CreateCompetition, ParticipantDto},
    error::AppResult,
    models::{competitions, horses, jockeys, race_results, races},
    state::AppState,
};

pub async fn list_competitions(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<competitions::Model>>> {
    let comps = competitions::Entity::find().all(&state.db).await?;
    Ok(Json(comps))
}

pub async fn create_competition(
    State(state): State<AppState>,
    Json(payload): Json<CreateCompetition>,
) -> AppResult<(StatusCode, Json<competitions::Model>)> {
    let comp = competitions::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        date: Set(payload.date),
        time: Set(payload.time),
        venue: Set(payload.venue),
    };
    let comp = comp.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(comp)))
}

/// GET /competitions/:id/participants
/// Пункт 1 ТЗ: список жокеев и лошадей с занятыми местами.
/// Возвращаем все результаты всех заездов в этом состязании.
pub async fn competition_participants(
    State(state): State<AppState>,
    Path(competition_id): Path<Uuid>,
) -> AppResult<Json<Vec<ParticipantDto>>> {
    // 1. Все заезды состязания
    let race_rows = races::Entity::find()
        .filter(races::Column::CompetitionId.eq(competition_id))
        .all(&state.db)
        .await?;

    let race_ids: Vec<Uuid> = race_rows.iter().map(|r| r.id).collect();
    if race_ids.is_empty() {
        return Ok(Json(vec![]));
    }

    // 2. Все результаты по этим заездам + horse + jockey одним запросом
    let results = race_results::Entity::find()
        .filter(race_results::Column::RaceId.is_in(race_ids))
        .find_also_related(horses::Entity)
        .find_also_related(jockeys::Entity)
        .all(&state.db)
        .await?;

    // 3. Мапим в DTO. Имя заезда подтягиваем из подготовленного мапа.
    let race_names: std::collections::HashMap<Uuid, Option<String>> =
        race_rows.into_iter().map(|r| (r.id, r.name)).collect();

    let dtos = results
        .into_iter()
        .map(|(r, horse, jockey)| ParticipantDto {
            race_id: r.race_id,
            race_name: race_names.get(&r.race_id).cloned().flatten(),
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
