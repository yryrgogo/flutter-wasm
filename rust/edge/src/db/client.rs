use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let username = std::env::var("PGUSER").unwrap();
    let password = std::env::var("PGPASSWORD").unwrap();
    let host = std::env::var("PGHOST").unwrap();
    let db_name = std::env::var("PGDATABASE").unwrap();
    let db_port = std::env::var("PGPORT").unwrap();
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, db_port, db_name
    );
    let max_connections = 10;
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres.");

    Ok(pool)
}
