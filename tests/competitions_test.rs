mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn create_competition_ok() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    let req = Request::builder()
        .method("POST")
        .uri("/competitions")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "name": "Кубок весны",
                "date": "2026-04-15",
                "time": "14:30:00",
                "venue": "Центральный ипподром"
            })
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["date"], "2026-04-15");
    assert_eq!(v["time"], "14:30:00"); // chrono сериализует с секундами
}

#[tokio::test]
async fn competition_invalid_date_rejected() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    // 31 февраля не существует
    let req = Request::builder()
        .method("POST")
        .uri("/competitions")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "date": "2026-02-31",
                "time": "10:00:00",
                "venue": "X"
            })
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert!(
        resp.status() == StatusCode::UNPROCESSABLE_ENTITY
            || resp.status() == StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn competition_invalid_time_rejected() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    // 25 часов — не бывает
    let req = Request::builder()
        .method("POST")
        .uri("/competitions")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "date": "2026-04-15",
                "time": "25:00:00",
                "venue": "X"
            })
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert!(
        resp.status() == StatusCode::UNPROCESSABLE_ENTITY
            || resp.status() == StatusCode::BAD_REQUEST
    );
}
