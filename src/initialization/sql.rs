use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::io::{ErrorKind, Result};

/// Prepares the SQL pool and connects to its host
pub async fn init() -> Result<Pool<Postgres>> {
  let sql_url: String = std::env::var("SQL_URL").unwrap_or(String::from("postgres://localhost"));

  let sql_max_connections = (std::env::var("SQL_MAX_CONNECTIONS").unwrap_or(String::from("32")))
    .parse::<u32>()
    .unwrap();

  let sql_pool = PgPoolOptions::new()
    .max_connections(sql_max_connections)
    .connect(&*sql_url)
    .await
    .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;

  Ok(sql_pool)
}
