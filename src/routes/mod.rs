use actix_web::{web, Scope};

mod health;

/// Mounts all route modules.
pub fn mount(app: &mut web::ServiceConfig) {
  let routes: Vec<(fn(Scope) -> Scope, &str)> = vec![(health::mount, "/health")];

  for (mount, path) in &routes {
    app.service(mount(web::scope(path)));
  }
}
