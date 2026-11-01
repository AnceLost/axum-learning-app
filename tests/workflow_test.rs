mod common;

use axum::http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn full_horse_racing_workflow() {
    let _guard = common::lock().await;
    let db = common::get_db().await;
    common::clean_db(&db).await;
    let app = common::app(db);

    // Пункты 2–4: создаём владельца, лошадей, жокеев
    let owner_id = common::create_owner(&app, "Иван Петров").await;
    let horse1 = common::create_horse(&app, &owner_id, "Буцефал").await;
    let horse2 = common::create_horse(&app, &owner_id, "Пегас").await;
    let jockey1 = common::create_jockey(&app, "Пётр Сидоров").await;
    let jockey2 = common::create_jockey(&app, "Семён Иванов").await;

    // Пункт 2: состязание
    let competition_id = common::create_competition(&app, "Кубок весны").await;
    let race_id = common::create_race(&app, &competition_id, "Заезд №1").await;

    // Пункт 5: добавляем результаты
    let (status, _) = common::post_json(
        &app,
        &format!("/races/{}/results", race_id),
        json!([
            {
                "horse_id": horse1, "jockey_id": jockey1,
                "place": 1, "finish_time": 123.456
            },
            {
                "horse_id": horse2, "jockey_id": jockey2,
                "place": 2, "finish_time": 124.500
            }
        ]),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    // Пункт 1: список жокеев и лошадей с местами
    let (status, body) = common::get_json(
        &app,
        &format!("/competitions/{}/participants", competition_id),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let participants = body.as_array().expect("expected array");
    assert_eq!(participants.len(), 2, "expected two participants");

    let first = participants
        .iter()
        .find(|p| p["place"] == 1)
        .expect("first place missing");
    assert_eq!(first["horse_name"], "Буцефал");
    assert_eq!(first["jockey_name"], "Пётр Сидоров");
    assert_eq!(first["race_name"], "Заезд №1");
    let ft = first["finish_time"].as_f64().unwrap();
    assert!((ft - 123.456).abs() < 1e-6, "finish_time mismatch: {}", ft);

    let second = participants
        .iter()
        .find(|p| p["place"] == 2)
        .expect("second place missing");
    assert_eq!(second["horse_name"], "Пегас");
    assert_eq!(second["jockey_name"], "Семён Иванов");

    // Пункт 6: состязания каждой лошади
    let (status, body) = common::get_json(&app, &format!("/horses/{}/competitions", horse1)).await;
    assert_eq!(status, StatusCode::OK);
    let comps = body.as_array().unwrap();
    assert_eq!(comps.len(), 1);
    assert_eq!(comps[0]["id"], competition_id);
    assert_eq!(comps[0]["name"], "Кубок весны");

    // Пункт 7: состязания каждого жокея
    let (status, body) =
        common::get_json(&app, &format!("/jockeys/{}/competitions", jockey2)).await;
    assert_eq!(status, StatusCode::OK);
    let comps = body.as_array().unwrap();
    assert_eq!(comps.len(), 1);
    assert_eq!(comps[0]["id"], competition_id);
}

/// Одно состязание, несколько заездов — участники должны собраться со всех.
#[tokio::test]
async fn participants_across_multiple_races() {
    let _guard = common::lock().await;
    let db = common::get_db().await;
    common::clean_db(&db).await;
    let app = common::app(db);

    let owner_id = common::create_owner(&app, "Владелец").await;
    let horse1 = common::create_horse(&app, &owner_id, "Буцефал").await;
    let horse2 = common::create_horse(&app, &owner_id, "Пегас").await;
    let horse3 = common::create_horse(&app, &owner_id, "Орёл").await;
    let jockey1 = common::create_jockey(&app, "Жокей 1").await;
    let jockey2 = common::create_jockey(&app, "Жокей 2").await;
    let jockey3 = common::create_jockey(&app, "Жокей 3").await;

    let comp_id = common::create_competition(&app, "Кубок").await;
    let race1 = common::create_race(&app, &comp_id, "Заезд А").await;
    let race2 = common::create_race(&app, &comp_id, "Заезд Б").await;

    common::post_json(
        &app,
        &format!("/races/{}/results", race1),
        json!([{
            "horse_id": horse1, "jockey_id": jockey1, "place": 1, "finish_time": 100.0
        }]),
    )
    .await;

    common::post_json(
        &app,
        &format!("/races/{}/results", race2),
        json!([
            { "horse_id": horse2, "jockey_id": jockey2, "place": 1, "finish_time": 101.0 },
            { "horse_id": horse3, "jockey_id": jockey3, "place": 2, "finish_time": 102.0 }
        ]),
    )
    .await;

    let (status, body) =
        common::get_json(&app, &format!("/competitions/{}/participants", comp_id)).await;
    assert_eq!(status, StatusCode::OK);
    let participants = body.as_array().unwrap();
    assert_eq!(participants.len(), 3, "results from both races expected");

    // Проверим, что race_name разный у результатов из разных заездов
    let race_names: std::collections::HashSet<String> = participants
        .iter()
        .filter_map(|p| p["race_name"].as_str().map(|s| s.to_string()))
        .collect();
    assert_eq!(race_names.len(), 2, "expected two distinct race names");
    assert!(race_names.contains("Заезд А"));
    assert!(race_names.contains("Заезд Б"));
}

/// Список результатов заезда отражает то, что мы вставили.
#[tokio::test]
async fn race_results_readable_after_insert() {
    let _guard = common::lock().await;
    let db = common::get_db().await;
    common::clean_db(&db).await;
    let app = common::app(db);

    let owner_id = common::create_owner(&app, "O").await;
    let horse = common::create_horse(&app, &owner_id, "H").await;
    let jockey = common::create_jockey(&app, "J").await;
    let comp = common::create_competition(&app, "C").await;
    let race = common::create_race(&app, &comp, "R").await;

    common::post_json(
        &app,
        &format!("/races/{}/results", race),
        json!([{
            "horse_id": horse, "jockey_id": jockey, "place": 1, "finish_time": 95.25
        }]),
    )
    .await;

    let (status, body) = common::get_json(&app, &format!("/races/{}/results", race)).await;
    assert_eq!(status, StatusCode::OK);
    let rows = body.as_array().unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["horse_name"], "H");
    assert_eq!(rows[0]["jockey_name"], "J");
    assert_eq!(rows[0]["place"], 1);
    let ft = rows[0]["finish_time"].as_f64().unwrap();
    assert!((ft - 95.25).abs() < 1e-6);
}

/// Лошадь без результатов: список состязаний пустой, но не 404.
#[tokio::test]
async fn horse_competitions_empty_without_results() {
    let _guard = common::lock().await;
    let db = common::get_db().await;
    common::clean_db(&db).await;
    let app = common::app(db);

    let owner_id = common::create_owner(&app, "O").await;
    let horse = common::create_horse(&app, &owner_id, "H").await;

    let (status, body) = common::get_json(&app, &format!("/horses/{}/competitions", horse)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 0);
}

/// Жокей без результатов: пустой список, не 404.
#[tokio::test]
async fn jockey_competitions_empty_without_results() {
    let _guard = common::lock().await;
    let db = common::get_db().await;
    common::clean_db(&db).await;
    let app = common::app(db);

    let jockey = common::create_jockey(&app, "J").await;

    let (status, body) = common::get_json(&app, &format!("/jockeys/{}/competitions", jockey)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 0);
}

/// Пустой список результатов — 400.
#[tokio::test]
async fn add_results_rejects_empty_list() {
    let _guard = common::lock().await;
    let db = common::get_db().await;
    common::clean_db(&db).await;
    let app = common::app(db);

    let comp = common::create_competition(&app, "C").await;
    let race = common::create_race(&app, &comp, "R").await;

    let (status, _) = common::post_json(&app, &format!("/races/{}/results", race), json!([])).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

/// place <= 0 — 400.
#[tokio::test]
async fn add_results_rejects_non_positive_place() {
    let _guard = common::lock().await;
    let db = common::get_db().await;
    common::clean_db(&db).await;
    let app = common::app(db);

    let owner_id = common::create_owner(&app, "O").await;
    let horse = common::create_horse(&app, &owner_id, "H").await;
    let jockey = common::create_jockey(&app, "J").await;
    let comp = common::create_competition(&app, "C").await;
    let race = common::create_race(&app, &comp, "R").await;

    let (status, _) = common::post_json(
        &app,
        &format!("/races/{}/results", race),
        json!([{
            "horse_id": horse, "jockey_id": jockey, "place": 0, "finish_time": 100.0
        }]),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}