use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait, ConnectionTrait};
use uuid::Uuid;

use crate::{
    dto::RaceResultInput,
    error::{AppError, AppResult},
    models::{enums::RaceStatus, race_results, races},
    state::AppState,
};

pub async fn add_results(
    State(state): State<AppState>,
    Path(race_id): Path<Uuid>,
    Json(inputs): Json<Vec<RaceResultInput>>,
) -> AppResult<StatusCode> {
    if inputs.is_empty() {
        return Err(AppError::BadRequest("results list is empty".into()));
    }

    let txn = state.db.begin().await?;

    insert_results(&txn, race_id, &inputs).await?;

    // Обновляем статус заезда
    let mut race: races::ActiveModel = races::Entity::find_by_id(race_id)
        .one(&txn)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("race {} not found", race_id)))?
        .into();
    race.status = Set(RaceStatus::Finished);
    race.update(&txn).await?;

    txn.commit().await?;

    Ok(StatusCode::CREATED)
}

async fn insert_results<C: ConnectionTrait>(
    conn: &C,
    race_id: Uuid,
    inputs: &[RaceResultInput],
) -> AppResult<()> {
    for input in inputs {
        if input.place <= 0 {
            return Err(AppError::BadRequest("place must be > 0".into()));
        }

        let finish_time_ms = input.finish_time.map(|secs| (secs * 1000.0) as i64);

        race_results::ActiveModel {
            id: Set(Uuid::new_v4()),
            race_id: Set(race_id),
            horse_id: Set(input.horse_id),
            jockey_id: Set(input.jockey_id),
            place: Set(input.place),
            finish_time: Set(finish_time_ms),
        }
        .insert(conn)
        .await?;
    }
    Ok(())
}