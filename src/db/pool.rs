use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Extract path from sqlite URL
    let db_path = database_url.trim_start_matches("sqlite:");

    // Create the database file if it doesn't exist
    if !Path::new(db_path).exists() {
        tracing::info!("Creating database file: {}", db_path);
        std::fs::File::create(db_path)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    tracing::info!("Running database migrations...");

    let migrations = [
        include_str!("../../migrations/001_initial.sql"),
        include_str!("../../migrations/002_railway_deployment.sql"),
    ];

    for migration_sql in migrations {
        // Split by statement and execute each
        for statement in migration_sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                match sqlx::query(statement).execute(pool).await {
                    Ok(_) => {}
                    Err(e) => {
                        // Ignore "table already exists" or "duplicate column" errors
                        let err_str = e.to_string();
                        if !err_str.contains("already exists") && !err_str.contains("duplicate column") {
                            tracing::warn!("Migration statement warning: {}", e);
                        }
                    }
                }
            }
        }
    }

    tracing::info!("Migrations completed");
    Ok(())
}
