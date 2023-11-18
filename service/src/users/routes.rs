use actix_web::{get, post, delete, web, HttpResponse, ResponseError};

use crate::auth::{JwtAuth, QueryUser, InsertUser};

#[get("users/favorites")]
async fn get_favorites(auth: JwtAuth) -> HttpResponse {
  println!("{:?}", auth);
  match QueryUser::get_by_email(&auth.user.email) {
    Ok(user) => {
      return HttpResponse::Ok().json(user.favorites)
    },
    Err(err) => return ResponseError::error_response(&err)
  }
}

#[post("users/favorites/{icao}")]
async fn add_favorite(icao: web::Path<String>, auth: JwtAuth) -> HttpResponse {
  match QueryUser::get_by_email(&auth.user.email) {
    Ok(user) => {
      if user.favorites.contains(&icao) {
        // Check if the airport ICAO is already in the user's favorites
        return HttpResponse::Conflict().finish()
      } else {
        // Add the airport ICAO to the user's favorites
        let mut favorites = user.favorites;
        favorites.push(icao.into_inner());
        match InsertUser::update_favorites(&user.email, favorites) {
          Ok(_) => return HttpResponse::Ok().finish(),
          Err(err) => return ResponseError::error_response(&err)
        }
      }
    },
    Err(err) => return ResponseError::error_response(&err)
  }
}

#[delete("users/favorites/{icao}")]
async fn delete_favorite(icao: web::Path<String>, auth: JwtAuth) -> HttpResponse {
  let icao: String = icao.into_inner();
  match QueryUser::get_by_email(&auth.user.email) {
    Ok(user) => {
      if user.favorites.contains(&icao) {
        // Check if the airport ICAO is already in the user's favorites
        let mut favorites = user.favorites;
        favorites.retain(|x| x != &icao);
        match InsertUser::update_favorites(&user.email, favorites) {
          Ok(_) => return HttpResponse::Ok().finish(),
          Err(err) => return ResponseError::error_response(&err)
        }
      } else {
        // Remove the airport ICAO from the user's favorites
        return HttpResponse::Conflict().finish()
      }
    },
    Err(err) => return ResponseError::error_response(&err)
  }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(get_favorites);
  config.service(add_favorite);
  config.service(delete_favorite);
}