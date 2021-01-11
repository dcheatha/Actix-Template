use actix_web::{get, HttpResponse, Responder, Scope};

/// Mounts all routes for this module.
/// May mount other modules by using router.service(module::mount(web::scope("/path"))).
pub fn mount(router: Scope) -> Scope {
  router.service(health).service(middleware)
}

#[get("")]
async fn health() -> impl Responder {
  HttpResponse::Ok().body("Ok")
}

#[get("middleware")]
async fn middleware() -> impl Responder {
  HttpResponse::Ok().body("Ok")
}
