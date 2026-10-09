use axum_postgres_app::{routes, state::AppState};
//use migration::{Migrator};
//use sea_orm_migration::MigratorTrait;
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    let database_url = std::env::var("DATABASE_URL")?;
    let db = Database::connect(&database_url).await?;

    // Применяем миграции при старте,
    //Migrator::up(&db, None).await?;

    let state = AppState { db };
    let app = routes::create_router(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}