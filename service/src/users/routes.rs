use actix_web::{get, post, delete, web, HttpResponse};

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
  config.service(get_favorites);
  config.service(add_favorite);
  config.service(delete_favorite);
}