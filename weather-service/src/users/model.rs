use std::{future::Future, pin::Pin, sync::RwLock};

use actix_web::{dev::Payload, error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest};
use actix_identity::Identity;
use diesel::{query_builder::AsChangeset, prelude::Insertable};
use log::warn;
use crate::schema::users;
use serde::{Serialize, Deserialize};

use super::user_type::UserType;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = users)]
pub struct InsertUser {
  first_name: String,
  last_name: String,
  user_type: UserType,
  favorites: Vec<String>
}

// impl FromRequest for InsertUser {
//   type Config = ();
//   type Error = Error;
//   type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

//   fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
//       let fut = Identity::from_request(req, pl);
//       let sessions: Option<&web::Data<RwLock<Sessions>>> = req.app_data();
//       if sessions.is_none() {
//           warn!("sessions is empty(none)!");
//           return Box::pin(async { Err(ErrorUnauthorized("unauthorized")) });
//       }
//       let sessions = sessions.unwrap().clone();
//       Box::pin(async move {
//           if let Some(identity) = fut.await?.identity() {
//               if let Some(user) = sessions
//                   .read()
//                   .unwrap()
//                   .map
//                   .get(&identity)
//                   .map(|x| x.clone())
//               {
//                   return Ok(user);
//               }
//           };

//           Err(ErrorUnauthorized("unauthorized"))
//       })
//   }
// }