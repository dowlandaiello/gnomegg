use super::schema::mutes;
use super::user::User;
use chrono::{DateTime, Duration, Utc};
use redis::{FromRedisValue, RedisError, Value};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;

use std::{
    io::{Error as IoError, ErrorKind},
};

/// Mute represents a mute entry in the SQL database.
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "mutes"]
#[primary_key(user_id)]
pub(crate) struct Mute {
    /// The ID of the user corresponding to this mute
    user_id: u64,

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
    pub fn new(user_id: u64, duration: u64) -> Self {
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
    pub fn with_user_id(mut self, user_id: u64) -> Self {
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
        Utc::now() < self.initiated_at + Duration::nanoseconds(self.duration as i64)
    }

    /// Retreieves the ID pertaining to the use who will be muted.
    pub fn concerns(&self) -> u64 {
        self.user_id
    }

    /// Constructs a duration representing the timeframe that the mute will be
    /// active for.
    pub fn active_for(&self) -> Duration {
        Duration::nanoseconds(self.duration as i64)
    }
}

impl FromRedisValue for Mute {
    fn from_redis_value(v: &Value) -> Result<Self, RedisError> {
        match v {
            Value::Data(d) => serde_json::from_slice(&d)
                .map_err(|e| <SerdeError as Into<IoError>>::into(e).into()),
            _ => Err(IoError::new(ErrorKind::Other, "unexpected response type").into()),
        }
    }
}
