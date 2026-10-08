mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use axum_postgres_app::models::horses;

async fn create_owner(app: &Router) -> String {
    let req = Request::builder()
        .method("POST")
        .uri("/owners")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({"name": "Test Owner"}).to_string(),
        ))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();
    v["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn create_horse_ok() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    let owner_id = create_owner(&app).await;

    let req = Request::builder()
        .method("POST")
        .uri("/horses")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "name": "Буцефал",
                "gender": "STALLION",
                "age": 5,
                "owner_id": owner_id,
            })
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["name"], "Буцефал");
    assert_eq!(v["gender"], "STALLION"); // важно: сериализуется в верхнем регистре
    assert_eq!(v["age"], 5);
}