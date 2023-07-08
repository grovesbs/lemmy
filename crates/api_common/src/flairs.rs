use actix_web::{get, http::Error, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::context::LemmyContext;

#[derive(Serialize, Deserialize)]
pub struct UserFlairQuery {
  user: i32,
  community_id: i32,
}

/// Get a user's flair from commmunity and person id
pub async fn get_user_flair(info: web::Query<UserFlairQuery>) -> Result<impl Responder> {

  Ok(web::Json({
    
  }))
}
