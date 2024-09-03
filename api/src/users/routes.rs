// use actix_multipart::Multipart;
// use actix_web::{get, post, delete, web, HttpResponse, ResponseError};
// use futures_util::StreamExt;

// use crate::{
//   auth::Auth,
//   db::{delete_file, get_file, upload_file},
//   error::ServiceError,
//   users::User,
// };

// #[get("/favorites")]
// async fn get_favorites(auth: Auth) -> HttpResponse {
//   match User::get_by_email(&auth.user.email) {
//     Ok(user) => return HttpResponse::Ok().json(user.favorites),
//     Err(err) => return ResponseError::error_response(&err),
//   }
// }

// #[post("/favorites/{icao}")]
// async fn add_favorite(icao: web::Path<String>, auth: Auth) -> HttpResponse {
//   match User::get_by_email(&auth.user.email) {
//     Ok(user) => {
//       if user.favorites.contains(&icao) {
//         // Check if the airport ICAO is already in the user's favorites
//         return HttpResponse::Conflict().finish();
//       } else {
//         // Add the airport ICAO to the user's favorites
//         let mut favorites = user.favorites;
//         favorites.push(icao.into_inner());
//         match User::update_favorites(&user.email, favorites) {
//           Ok(_) => return HttpResponse::Ok().finish(),
//           Err(err) => return ResponseError::error_response(&err),
//         }
//       }
//     }
//     Err(err) => return ResponseError::error_response(&err),
//   }
// }

// #[delete("/favorites/{icao}")]
// async fn delete_favorite(icao: web::Path<String>, auth: Auth) -> HttpResponse {
//   let icao: String = icao.into_inner();
//   match User::get_by_email(&auth.user.email) {
//     Ok(user) => {
//       if user.favorites.contains(&icao) {
//         // Check if the airport ICAO is already in the user's favorites
//         let mut favorites = user.favorites;
//         favorites.retain(|x| x != &icao);
//         match User::update_favorites(&user.email, favorites) {
//           Ok(_) => return HttpResponse::Ok().finish(),
//           Err(err) => return ResponseError::error_response(&err),
//         }
//       } else {
//         // Remove the airport ICAO from the user's favorites
//         return HttpResponse::Conflict().finish();
//       }
//     }
//     Err(err) => return ResponseError::error_response(&err),
//   }
// }

// #[post("/picture")]
// async fn set_picture(mut payload: Multipart, auth: Auth) -> HttpResponse {
//   while let Some(item) = payload.next().await {
//     let mut bytes = web::BytesMut::new();
//     let mut field = match item {
//       Ok(field) => field,
//       Err(err) => return ResponseError::error_response(&err),
//     };
//     let content_type = field.content_disposition();
//     let filename = match content_type.unwrap().get_filename() {
//       Some(name) => match name.split(".").last() {
//         Some(ext) => match ext {
//           "apng" | "avif" | "gif" | "jpg" | "jpeg" | "jfif" | "pjpeg" | "pjp" | "png" | "svg"
//           | "webp" => name,
//           _ => {
//             return ResponseError::error_response(&ServiceError::new(
//               400,
//               "File extension is not supported".to_string(),
//             ))
//           }
//         },
//         None => {
//           return ResponseError::error_response(&ServiceError::new(
//             400,
//             "Unknown file extension".to_string(),
//           ))
//         }
//       },
//       None => {
//         return ResponseError::error_response(&ServiceError::new(
//           400,
//           "File name is not provided".to_string(),
//         ))
//       }
//     };
//     let path = format!("users/{}/{}", auth.user.email, filename);

//     while let Some(chunk) = field.next().await {
//       let data = match chunk {
//         Ok(data) => data,
//         Err(err) => return ResponseError::error_response(&err),
//       };
//       bytes.extend_from_slice(&data);
//     }
//     match upload_file(&path, &bytes).await {
//       Ok(_) => {
//         match User::update_profile_picture(&auth.user.email, Some(&path)) {
//           Ok(_) => {}
//           Err(err) => return ResponseError::error_response(&err),
//         };
//       }
//       Err(err) => return ResponseError::error_response(&err),
//     };
//   }
//   HttpResponse::Ok().finish()
// }

// #[get("/picture")]
// async fn get_picture(auth: Auth) -> HttpResponse {
//   let user = match User::get_by_email(&auth.user.email) {
//     Ok(user) => user,
//     Err(err) => return ResponseError::error_response(&err),
//   };
//   if let Some(path) = user.profile_picture {
//     match get_file(&path).await {
//       Ok(bytes) => HttpResponse::Ok().body(bytes),
//       Err(err) => ResponseError::error_response(&err),
//     }
//   } else {
//     HttpResponse::NotFound().finish()
//   }
// }

// #[delete("/picture")]
// async fn delete_picture(auth: Auth) -> HttpResponse {
//   let user = match User::get_by_email(&auth.user.email) {
//     Ok(user) => user,
//     Err(err) => return ResponseError::error_response(&err),
//   };
//   if let Some(path) = user.profile_picture {
//     match delete_file(&path).await {
//       Ok(_) => match User::update_profile_picture(&auth.user.email, None) {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(err) => ResponseError::error_response(&err),
//       },
//       Err(err) => ResponseError::error_response(&err),
//     }
//   } else {
//     HttpResponse::NotFound().finish()
//   }
// }

pub fn init_routes(config: &mut actix_web::web::ServiceConfig) {
  // config.service(
  //   web::scope("users")
  //     .service(get_favorites)
  //     .service(add_favorite)
  //     .service(delete_favorite)
  //     .service(set_picture)
  //     .service(get_picture)
  //     .service(delete_picture),
  // );
}
