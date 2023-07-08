#[cfg(feature = "full")]
use crate::schema::user_flairs;

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable, TS))]
#[cfg_attr(feature = "full", diesel(table_name = user_flairs))]
#[cfg_attr(feature = "full", diesel(belongs_to(crate::source::site::Site)))]
#[cfg_attr(feature = "full", ts(export))]
pub struct UserFlairs {
    
}