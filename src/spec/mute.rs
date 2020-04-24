use super::schema::mutes;
use super::user::User;
use chrono::{DateTime, Duration, Utc};
use redis_async::{
    error::Error as RedisError,
    resp::{FromResp, RespValue},
};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;

use std::convert::TryFrom;

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

impl Default for Mute {
    fn default() -> Self {
        Self {
            user_id: 0,
            duration: 0,
            initiated_at: Utc::now(),
        }
    }
}

impl Mute {
    /// Creates a new mute primitive, assuming the current time as the
    /// initiation timestamp.
    pub fn new(&self, user_id: i32, duration: u64) -> Self {
        Self {
            user_id,
            duration,
            initiated_at: Utc::now(),
        }
    }

    /// Creates a new mute primitive based off the current mute instance, with
    /// the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID that should be associated with this mute
    pub fn with_user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;

        self
    }

    /// Creats a new mute primitive based off the current mute instance, with
    /// the provided duration (in nanoseconds).
    ///
    /// # Arguments
    ///
    /// * `duration` - The number of nanoseconds that the mute should be active
    /// for
    pub fn with_duration(mut self, duration: u64) -> Self {
        self.duration = duration;

        self
    }

    /// Creates a new mute primitive based off the current mute instance, with
    /// the provided initiation time (UTC).
    ///
    /// # Arguments
    ///
    /// * `initiated_at` - The time at which the mute was issued
    pub fn with_initiation_timestamp(mut self, initiated_at: DateTime<Utc>) -> Self {
        self.initiated_at = initiated_at;

        self
    }

    /// Determines whether or not the mute is active.
    pub fn active(&self) -> bool {
        Utc::now()
            < self.initiated_at + Duration::nanoseconds(self.duration.try_into().or_default())
    }
}

impl TryFrom<Mute> for RespValue {
    type Error = SerdeError;

    fn try_from(m: Mute) -> Result<Self, SerdeError> {
        serde_json::to_vec(&m).map(|raw_serialized| Self::BulkString(raw_serialized))
    }
}

impl FromResp for Mute {
    fn from_resp_int(resp: RespValue) -> Result<Self, RedisError> {
        match resp {
            RespValue::Nil => Err(RedisError::Internal("unexpected nil response".to_owned())),
            RespValue::Array(_) => Err(RedisError::Internal(
                "unexpected vector of unrecognized response value".to_owned(),
            )),
            RespValue::BulkString(arr) => serde_json::from_slice(&arr)
                .map_err(|e| RedisError::RESP(e.to_string(), Some(resp))),
            RespValue::Error(e) => Err(RedisError::Remote(e)),
            RespValue::Integer(_) => Err(RedisError::Internal(
                "unexpected integer response".to_owned(),
            )),
            RespValue::SimpleString(_) => Err(RedisError::Internal(
                "unexpected string response".to_owned(),
            )),
        }
    }
}
