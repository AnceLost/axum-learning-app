use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
    dto::{CreateHorse, ParticipantDto},
    error::{AppError, AppResult},
    models::{competitions, horses, jockeys, race_results, races},
    state::AppState,
};

pub async fn list_horses(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<horses::Model>>> {
    let horses = horses::Entity::find().all(&state.db).await?;
    Ok(Json(horses))
}

pub async fn create_horse(
    State(state): State<AppState>,
    Json(payload): Json<CreateHorse>,
) -> AppResult<(StatusCode, Json<horses::Model>)> {
    if payload.age <= 0 {
        return Err(AppError::BadRequest("age must be > 0".into()));
    }
    let horse = horses::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        gender: Set(payload.gender),
        age: Set(payload.age),
        owner_id: Set(payload.owner_id),
        ..Default::default()
    };
    let horse = horse.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(horse)))
}

/// GET /horses/:id/competitions — список состязаний, в которых участвовала лошадь
pub async fn horse_competitions(
    State(state): State<AppState>,
    Path(horse_id): Path<Uuid>,
) -> AppResult<Json<Vec<competitions::Model>>> {
    // 1. Все результаты этой лошади
    let results = race_results::Entity::find()
        .filter(race_results::Column::HorseId.eq(horse_id))
        .all(&state.db)
        .await?;

    let race_ids: Vec<Uuid> = results.iter().map(|r| r.race_id).collect();
    if race_ids.is_empty() {
        return Ok(Json(vec![]));
    }

    // 2. Заезды, в которых она участвовала
    let race_rows = races::Entity::find()
        .filter(races::Column::Id.is_in(race_ids))
        .all(&state.db)
        .await?;

    let competition_ids: Vec<Uuid> = race_rows.iter().map(|r| r.competition_id).collect();

    // 3. Состязания
    let comps = competitions::Entity::find()
        .filter(competitions::Column::Id.is_in(competition_ids))
        .all(&state.db)
        .await?;

    Ok(Json(comps))
}