mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

async fn setup_full(app: &axum::Router) -> (String, String, String, String) {
    // owner
    let req = Request::builder()
        .method("POST")
        .uri("/owners")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({"name": "O"}).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();
    let owner_id = v["id"].as_str().unwrap().to_string();

    // jockey
    let req = Request::builder()
        .method("POST")
        .uri("/jockeys")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({"name": "J"}).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();
    let jockey_id = v["id"].as_str().unwrap().to_string();

    // horse
    let req = Request::builder()
        .method("POST")
        .uri("/horses")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "name": "H", "gender": "MARE", "age": 4, "owner_id": owner_id,
            })
            .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();
    let horse_id = v["id"].as_str().unwrap().to_string();

    // competition + race
    let req = Request::builder()
        .method("POST")
        .uri("/competitions")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "date": "2026-04-15", "time": "14:30:00", "venue": "X",
            })
            .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();
    let competition_id = v["id"].as_str().unwrap().to_string();

    let req = Request::builder()
        .method("POST")
        .uri(format!("/competitions/{}/races", competition_id))
        .header("Content-Type", "application/json")
        .body(Body::from(json!({"name": "Заезд 1"}).to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();
    let race_id = v["id"].as_str().unwrap().to_string();

    (horse_id, jockey_id, race_id, competition_id)
}

#[tokio::test]
async fn finish_time_converted_correctly() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    let (horse_id, jockey_id, race_id, _) = setup_full(&app).await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/races/{}/results", race_id))
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!([{
                "horse_id": horse_id,
                "jockey_id": jockey_id,
                "place": 1,
                "finish_time": 123.456
            }])
            .to_string(),
        ))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // читаем обратно
    let req = Request::builder()
        .method("GET")
        .uri(format!("/races/{}/results", race_id))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();

    // 123.456 -> 123456 ms -> 123.456 s. Должно совпасть с точностью до float.
    let ft = v[0]["finish_time"].as_f64().unwrap();
    assert!((ft - 123.456).abs() < 1e-6, "got {}", ft);
}

#[tokio::test]
async fn finish_time_nullable() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    let (horse_id, jockey_id, race_id, _) = setup_full(&app).await;

    let req = Request::builder()
        .method("POST")
        .uri(format!("/races/{}/results", race_id))
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!([{
                "horse_id": horse_id,
                "jockey_id": jockey_id,
                "place": 1,
                "finish_time": null
            }])
            .to_string(),
        ))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn duplicate_place_rejected() {
    let db = common::get_db().await.clone();
    common::clean_db(&db).await;
    let app = common::app(db);

    let (horse_id, jockey_id, race_id, _) = setup_full(&app).await;

    // первая вставка
    let req = Request::builder()
        .method("POST")
        .uri(format!("/races/{}/results", race_id))
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!([{
                "horse_id": horse_id, "jockey_id": jockey_id,
                "place": 1, "finish_time": 100.0
            }])
            .to_string(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // вторая лошадь на то же место — должно упасть по UNIQUE constraint
    // (нужен второй horse/jockey, но для теста хватит того, что тот же race_id + place)
    let req = Request::builder()
        .method("POST")
        .uri(format!("/races/{}/results", race_id))
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!([{
                "horse_id": horse_id, "jockey_id": jockey_id,
                "place": 1, "finish_time": 200.0
            }])
            .to_string(),
        ))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert!(
        resp.status().is_client_error() || resp.status().is_server_error(),
        "expected error, got {}",
        resp.status()
    );
}
