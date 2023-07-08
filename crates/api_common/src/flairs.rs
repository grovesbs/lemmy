use actix_web::{web, Responder};

use lemmy_db_schema::source::local_site::LocalSite;
use lemmy_utils::error::LemmyError;
use serde::{Deserialize, Serialize};
use crate::context::LemmyContext;

#[derive(Serialize, Deserialize)]
pub struct UserFlairQuery {
  user: i32,
  community_id: i32,
}

/// Get a user's flair from commmunity and person id
pub async fn get_user_flair(data: web::Data<LemmyContext>, info: web::Query<UserFlairQuery>) -> Result<impl Responder, LemmyError> {
  let local_user_view = LocalSite::read(data.pool()).await?;
  Ok(web::Json({}))
}
