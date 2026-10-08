use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::{competitions, horses, jockeys, owners, races, results},
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Владельцы
        .route("/owners", get(owners::list_owners).post(owners::create_owner))
        // Лошади
        .route("/horses", get(horses::list_horses).post(horses::create_horse))
        .route("/horses/{id}/competitions", get(horses::horse_competitions))
        // Жокеи
        .route("/jockeys", get(jockeys::list_jockeys).post(jockeys::create_jockey))
        .route("/jockeys/{id}/competitions", get(jockeys::jockey_competitions))
        // Состязания
        .route(
            "/competitions",
            get(competitions::list_competitions).post(competitions::create_competition),
        )
        .route(
            "/competitions/{id}/participants",
            get(competitions::competition_participants),
        )
        // Заезды внутри состязания
        .route("/competitions/{id}/races", post(races::create_race))
        .route(
            "/races/{id}/results",
            get(races::race_results_list).post(results::add_results),
        )
        .with_state(state)
}