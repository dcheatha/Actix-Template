use actix_web::{App, HttpServer};
use std::io::Result;

mod initialization;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> Result<()> {
  let listen_url: String = std::env::var("LISTEN_URL").unwrap_or(String::from("localhost:8080"));

  let pool_sql = initialization::sql::init().await.unwrap();

  HttpServer::new(move || {
    App::new().configure(routes::mount).data(models::AppState {
      pool_sql: pool_sql.clone(),
    })
  })
  .bind(listen_url)?
  .run()
  .await
}
