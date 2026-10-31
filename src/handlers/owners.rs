use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;

use crate::{dto::CreateOwner, error::AppResult, models::owners, state::AppState};

pub async fn list_owners(State(state): State<AppState>) -> AppResult<Json<Vec<owners::Model>>> {
    let owners = owners::Entity::find().all(&state.db).await?;
    Ok(Json(owners))
}

pub async fn create_owner(
    State(state): State<AppState>,
    Json(payload): Json<CreateOwner>,
) -> AppResult<(StatusCode, Json<owners::Model>)> {
    let owner = owners::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        address: Set(payload.address),
        phone: Set(payload.phone),
    };
    let owner = owner.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(owner)))
}
