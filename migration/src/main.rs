use migration::{Migrator, MigratorTrait};
use sea_orm_migration::sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("DATABASE_URL")?;
    let db = Database::connect(&url).await?;
    Migrator::up(&db, None).await?;
    Ok(())
}