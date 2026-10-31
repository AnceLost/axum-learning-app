mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn horse_gender_lowercase_rejected() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    // сначала нужен owner
    let req = Request::builder()
        .method("POST")
        .uri("/owners")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({"name": "X"}).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let owner_id = v["id"].as_str().unwrap().to_string();

    // lowercase должен быть отвергнут
    let req = Request::builder()
        .method("POST")
        .uri("/horses")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "name": "H",
                "gender": "stallion", // <-- неверно
                "age": 3,
                "owner_id": owner_id,
            })
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    // axum при провале Json extractor возвращает 422 или 400
    assert!(
        resp.status() == StatusCode::UNPROCESSABLE_ENTITY
            || resp.status() == StatusCode::BAD_REQUEST,
        "unexpected status: {}",
        resp.status()
    );
}

#[tokio::test]
async fn horse_age_zero_rejected() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    let req = Request::builder()
        .method("POST")
        .uri("/owners")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({"name": "X"}).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let owner_id = v["id"].as_str().unwrap().to_string();

    let req = Request::builder()
        .method("POST")
        .uri("/horses")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "name": "H",
                "gender": "MARE",
                "age": 0, // <-- невалидно
                "owner_id": owner_id,
            })
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
