use migration::{Migrator, MigratorTrait};
use sea_orm_migration::MigrationStatus;
use sea_orm_migration::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

/// Сколько раз пробуем подключиться к БД (по умолчанию 30 раз с паузой 2 с).
/// Нужно, чтобы мигратор переживал перезагрузку/старт удалённой БД.
const DEFAULT_CONNECT_ATTEMPTS: u32 = 30;
const CONNECT_DELAY: Duration = Duration::from_secs(2);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("DATABASE_URL")?;
    let command = std::env::args().nth(1).unwrap_or_else(|| "up".to_string());

    if command != "up" && command != "status" {
        eprintln!("Неизвестная команда: '{command}'");
        eprintln!("Использование: migrator [up|status]  (по умолчанию: up)");
        std::process::exit(2);
    }

    let db = connect_with_retry(&url).await?;

    match command.as_str() {
        "up" => {
            Migrator::up(&db, None).await?;
            println!("Миграции применены, схема актуальна.");
        }
        "status" => status(&db).await?,
        _ => unreachable!(),
    }

    Ok(())
}

/// Подключение с повторами: удалённая БД может подниматься дольше, чем идёт коннект.
async fn connect_with_retry(url: &str) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let attempts = std::env::var("DB_CONNECT_ATTEMPTS")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_CONNECT_ATTEMPTS);

    for attempt in 1..=attempts {
        let mut options = ConnectOptions::new(url.to_owned());
        options.connect_timeout(CONNECT_TIMEOUT);

        match Database::connect(options).await {
            Ok(db) => return Ok(db),
            Err(err) if attempt == attempts => return Err(Box::new(err)),
            Err(err) => {
                eprintln!(
                    "БД недоступна (попытка {attempt}/{attempts}): {err}. Повтор через {} с.",
                    CONNECT_DELAY.as_secs()
                );
                tokio::time::sleep(CONNECT_DELAY).await;
            }
        }
    }

    unreachable!("цикл всегда возвращает результат")
}

/// Печатает состояние миграций. Возвращает ошибку, если в БД есть миграции,
/// которых нет в коде (схема "впереди" приложения — откатили деплой / чужой migrator).
async fn status(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let migrations = Migrator::get_migration_with_status(db).await?;

    let mut pending = 0;
    for migration in &migrations {
        if migration.status() == MigrationStatus::Pending {
            pending += 1;
        }
        println!("[{:7}] {}", migration.status(), migration.name());
    }

    println!();
    if pending == 0 {
        println!(
            "Схема актуальна: применены все миграции ({}/{}).",
            migrations.len(),
            migrations.len()
        );
    } else {
        println!(
            "Не применено миграций: {pending} из {}. Они будут применены командой 'up'.",
            migrations.len()
        );
    }

    Ok(())
}
