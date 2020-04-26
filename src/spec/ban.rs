use super::{schema::bans, user::User};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::Associations;
use serde::{Deserialize, Serialize};

/// Ban represents a ban entry in the SQL database.
#[derive(
    Identifiable, Insertable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug,
)]
#[belongs_to(User)]
#[table_name = "bans"]
#[primary_key(user_id)]
pub struct Ban {
    /// The ID of the user corresponding to this ban
    user_id: u64,

    /// The (optional) number of nanoseconds that this ban will be in effect for
    duration: Option<u64>,

    /// The time at which the ban was issued
    initiated_at: NaiveDateTime,

    /// The IP address of the user being banned
    ip: Option<String>,
}

impl Default for Ban {
    fn default() -> Self {
        Self {
            user_id: 0,
            duration: None,
            initiated_at: Utc::now().naive_utc(),
            ip: None,
        }
    }
}

impl Ban {
    /// Creates a new ban primitive, assuming a permaban at the current time.
    pub fn new(user_id: u64) -> Self {
        Self {
            user_id,
            duration: None,
            initiated_at: Utc::now().naive_utc(),
            ip: None,
        }
    }

    /// Creates a new ban primitive based off the current ban instance, with
    /// the provided user ID.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID that should be associated with the ban
    pub fn with_user_id(mut self, user_id: u64) -> Self {
        self.user_id = user_id;

        self
    }

    /// Creates a new ban primitive based off the current ban instance, with
    /// the provided duration (in nanoseconds).
    ///
    /// # Arguments
    ///
    /// * `duration` - The number of nanoseconds that the ban should be active
    /// for
    pub fn with_duration(mut self, duration: u64) -> Self {
        self.duration = Some(duration);

        self
    }

    /// Creates a new ban primitive based off the current ban instance, with
    /// the provided initiation time (UTC).
    ///
    /// # Arguments
    ///
    /// * `initiated_at` - The time at which the ban was issued
    pub fn with_initiation_timestamp(mut self, initiated_at: DateTime<Utc>) -> Self {
        self.initiated_at = initiated_at.naive_utc();

        self
    }

    /// Creates a new ban primitive based off the current ban instance, with
    /// the provided IP.
    ///
    /// # Arguments
    ///
    /// * `ip` - The IP of the user being banned
    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);

        self
    }

    /// Determines whether or not the ban is active.
    pub fn active(&self) -> bool {
        self.active_for()
            .map_or(true, |d| Utc::now().naive_utc() < self.initiated_at + d)
    }

    /// Retreieves the ID pertaining to the use who will be band.
    pub fn concerns(&self) -> u64 {
        self.user_id
    }

    /// Constructs a duration representing the timeframe that the ban will be
    /// active for.
    pub fn active_for(&self) -> Option<Duration> {
        self.duration.map(|d| Duration::nanoseconds(d as i64))
    }

    /// Obtains the IP adddress of the user being banned.
    pub fn address(&self) -> Option<&str> {
        self.ip.as_deref()
    }
}

/// NewBan represents a request to add a ban entry in the database.
#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "bans"]
pub struct NewBan<'a> {
    /// The ID of the user corresponding to this ban
    user_id: u64,

    /// The (optional) number of nanoseconds that this ban will be in effect for
    duration: Option<u64>,

    /// The time at which the ban was issued
    initiated_at: NaiveDateTime,

    /// The IP address of the user being banned
    ip: Option<&'a str>,
}

impl<'a> NewBan<'a> {
    /// Creates a new request to add a ban entry in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user corresponding to this ban
    /// * `duration` - The (optional) number of nanoseconds that this ban will be in effect for
    /// * `initiated_at` - The time at which the ban was issued
    /// * `ip` - The (optional) IP address of the user being banned
    pub fn new(
        user_id: u64,
        duration: Option<u64>,
        initiated_at: NaiveDateTime,
        ip: Option<&'a str>,
    ) -> Self {
        Self {
            user_id,
            duration,
            initiated_at,
            ip,
        }
    }

    /// Determines whether or not the ban is active.
    pub fn active(&self) -> bool {
        self.active_for()
            .map_or(true, |d| Utc::now().naive_utc() < self.initiated_at + d)
    }

    /// Retreieves the ID pertaining to the use who will be band.
    pub fn concerns(&self) -> u64 {
        self.user_id
    }

    /// Constructs a duration representing the timeframe that the ban will be
    /// active for.
    pub fn active_for(&self) -> Option<Duration> {
        self.duration.map(|d| Duration::nanoseconds(d as i64))
    }

    /// Obtains the IP adddress of the user being banned.
    pub fn address(&self) -> Option<&str> {
        self.ip
    }
}
