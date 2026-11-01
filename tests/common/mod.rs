#![allow(dead_code)]

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum_postgres_app::{routes, state::AppState};
use http_body_util::BodyExt;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use serde_json::{Value, json};
use std::sync::{Once, OnceLock};
use tokio::sync::Mutex;
use tower::ServiceExt;

static INIT: Once = Once::new();
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// Блокировка, сериализующая тесты. Каждый тест должен её взять
/// в начале, чтобы TRUNCATE одного не снёс данные другого.
pub async fn lock() -> tokio::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().await
}

fn init_env() {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.test").ok();
    });
}

pub async fn get_db() -> DatabaseConnection {
    init_env();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL должно быть внутри .env.test");

    let mut opts = ConnectOptions::new(url);
    opts.max_connections(5)
        .min_connections(1)
        .sqlx_logging(false);

    Database::connect(opts)
        .await
        .expect("failed to connect to test database")
}

pub fn app(db: DatabaseConnection) -> Router {
    routes::create_router(AppState { db })
}

pub async fn clean_db(db: &DatabaseConnection) {
    db.execute_unprepared(
        "TRUNCATE race_results, races, competitions, horses, jockeys, owners RESTART IDENTITY CASCADE",
    )
    .await
    .expect("truncate failed");
}

// Низкоуровневые HTTP-функции

pub async fn post_json(app: &Router, uri: &str, body: Value) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };
    (status, json)
}

pub async fn get_json(app: &Router, uri: &str) -> (StatusCode, Value) {
    let req = Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };
    (status, json)
}

// Высокоуровневые функции (создают сущность, возвращают id)

pub async fn create_owner(app: &Router, name: &str) -> String {
    let (status, body) = post_json(app, "/owners", json!({ "name": name })).await;
    assert_eq!(status, StatusCode::CREATED, "create_owner: {}", body);
    body["id"].as_str().unwrap().to_string()
}

pub async fn create_horse(app: &Router, owner_id: &str, name: &str) -> String {
    let (status, body) = post_json(
        app,
        "/horses",
        json!({
            "name": name,
            "gender": "STALLION",
            "age": 5,
            "owner_id": owner_id,
        }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "create_horse: {}", body);
    body["id"].as_str().unwrap().to_string()
}

pub async fn create_jockey(app: &Router, name: &str) -> String {
    let (status, body) = post_json(
        app,
        "/jockeys",
        json!({
            "name": name,
            "age": 30,
            "rating": 1500,
        }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "create_jockey: {}", body);
    body["id"].as_str().unwrap().to_string()
}

pub async fn create_competition(app: &Router, name: &str) -> String {
    let (status, body) = post_json(
        app,
        "/competitions",
        json!({
            "name": name,
            "date": "2026-04-15",
            "time": "14:30:00",
            "venue": "Центральный ипподром",
        }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "create_competition: {}", body);
    body["id"].as_str().unwrap().to_string()
}

pub async fn create_race(app: &Router, competition_id: &str, name: &str) -> String {
    let (status, body) = post_json(
        app,
        &format!("/competitions/{}/races", competition_id),
        json!({ "name": name }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "create_race: {}", body);
    body["id"].as_str().unwrap().to_string()
}
