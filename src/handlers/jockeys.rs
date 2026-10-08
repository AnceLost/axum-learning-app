use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    dto::CreateJockey,
    error::AppResult,
    models::{competitions, jockeys, race_results, races},
    state::AppState,
};

pub async fn list_jockeys(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<jockeys::Model>>> {
    let jockeys = jockeys::Entity::find().all(&state.db).await?;
    Ok(Json(jockeys))
}

pub async fn create_jockey(
    State(state): State<AppState>,
    Json(payload): Json<CreateJockey>,
) -> AppResult<(StatusCode, Json<jockeys::Model>)> {
    let jockey = jockeys::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        address: Set(payload.address),
        age: Set(payload.age),
        rating: Set(payload.rating.unwrap_or(0)),
        ..Default::default()
    };
    let jockey = jockey.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(jockey)))
}

/// GET /jockeys/:id/competitions
pub async fn jockey_competitions(
    State(state): State<AppState>,
    Path(jockey_id): Path<Uuid>,
) -> AppResult<Json<Vec<competitions::Model>>> {
    let results = race_results::Entity::find()
        .filter(race_results::Column::JockeyId.eq(jockey_id))
        .all(&state.db)
        .await?;

    let race_ids: Vec<Uuid> = results.iter().map(|r| r.race_id).collect();
    if race_ids.is_empty() {
        return Ok(Json(vec![]));
    }

    let race_rows = races::Entity::find()
        .filter(races::Column::Id.is_in(race_ids))
        .all(&state.db)
        .await?;

    let competition_ids: Vec<Uuid> = race_rows.iter().map(|r| r.competition_id).collect();

    let comps = competitions::Entity::find()
        .filter(competitions::Column::Id.is_in(competition_ids))
        .all(&state.db)
        .await?;

    Ok(Json(comps))
}