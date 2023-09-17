use actix_web::{get, post, delete, put, web, HttpResponse};

#[get("users")]
async fn get() -> HttpResponse {
  HttpResponse::NotImplemented().finish()
}

#[get("users/{id}")]
async fn get_all() -> HttpResponse {
  HttpResponse::NotImplemented().finish()
}

#[post("users")]
async fn create() -> HttpResponse {
  HttpResponse::NotImplemented().finish()
}

#[delete("users")]
async fn delete() -> HttpResponse {
  HttpResponse::NotImplemented().finish()
}

#[put("users")]
async fn update() -> HttpResponse {
  HttpResponse::NotImplemented().finish()
}

#[get("users/favorites")]
async fn get_favorites() -> HttpResponse {
  HttpResponse::NotImplemented().finish()
}

#[post("users/favorites")]
async fn add_favorite() -> HttpResponse {
  HttpResponse::NotImplemented().finish()
}

#[delete("users/favorites")]
async fn delete_favorite() -> HttpResponse {
  HttpResponse::NotImplemented().finish()
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(get);
  config.service(create);
  config.service(delete);
  config.service(update);
  config.service(get_favorites);
  config.service(add_favorite);
  config.service(delete_favorite);
}