use axum::Router;
use axum_postgres_app::{routes, state::AppState};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, ConnectOptions};
use std::sync::Once;
use tokio::sync::OnceCell;

static INIT: Once = Once::new();
static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

/// Загружает .env.test ровно один раз за процесс.
fn init_env() {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.test").ok();
    });
}

/// Возвращает общий пул. Инициализируется лениво, асинхронно.
pub async fn get_db() -> DatabaseConnection {
    init_env();
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");

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

/// Полная очистка таблиц. Вызывается в начале каждого теста.
pub async fn clean_db(db: &DatabaseConnection) {
    db.execute_unprepared(
        "TRUNCATE race_results, races, competitions, horses, jockeys, owners RESTART IDENTITY CASCADE",
    )
    .await
    .expect("truncate failed");
}