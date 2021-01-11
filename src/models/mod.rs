use sqlx::{Pool, Postgres};

/// AppState stores the current state of the app, available on every request.
pub struct AppState {
  /// sql is the postgres pool.
  pub pool_sql: Pool<Postgres>,
}
