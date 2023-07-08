use diesel::prelude::*;

#[skip_serializing_none]
#[derive(
  Clone,
  PartialEq,
  Eq,
  Debug,
  Serialize,
  Deserialize,
  Clone,
  PartialEq,
  Eq,
  Debug,
  Serialize,
  Deserialize,
)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable, TS))]
#[cfg_attr(feature = "full", diesel(table_name = user_flairs))]
#[cfg_attr(feature = "full", ts(export))]
pub struct UserFlairs {
  pub id: i32,
  pub flair_id: Option<i32>,
  pub community_id: Option<i32>,
  pub created_on: Option<chrono::NaiveDateTime>,
}

#[cfg_attr(feature = "full", derive(Queryable, Identifiable, TS))]
#[cfg_attr(feature = "full", diesel(table_name = user_flairs))]
#[cfg_attr(feature = "full", ts(export))]
pub struct flairs {
  pub id: i32,
  pub name: Option<String>,
  pub community_id: Option<i32>,
  pub created_on: Option<chrono::NaiveDateTime>,
}
