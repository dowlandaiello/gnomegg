use super::schema::mutes;
use super::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Mute represents a mute entry in the SQL database.
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "mutes"]
#[primary_key(user_id)]
pub(crate) struct Mute {
    /// The ID of the user corresponding to this mute
    user_id: i32,

    /// The number of nanoseconds that this mute will be in effect for
    duration: u64,

    /// The time at which this mute was issued
    initiated_at: DateTime<Utc>,
}
