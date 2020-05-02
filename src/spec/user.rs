use super::schema::{ids, users};
use diesel::Insertable;
use serde::{Deserialize, Serialize};

use std::default::Default;

/// User represents a generic gnome.gg user.
#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
    /// The user's unique identifier
    id: u64,

    /// The username of the user
    username: String,

    /// Whether or not the user has a verified email
    verified: bool,

    /// The country that the user most identifies with
    nationality: String,

    /// Whether or not the user accepts gifts
    accepts_gifts: bool,

    /// The user's minecraft username
    minecraft_name: String,
}

/// NewUser represents a request to create a new user.
#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug, Default)]
#[table_name = "users"]
pub struct NewUser<'a> {
    /// The username of the user
    username: &'a str,

    /// Whether or not the user has a verified email
    verified: bool,

    /// The country that the user most identifies with
    nationality: &'a str,

    /// Whether or not the user accepts gifts
    accepts_gifts: bool,

    /// The user's minecraft username
    minecraft_name: &'a str,
}

impl<'a> NewUser<'a> {
    /// Creates a completely filled user primitive instance with the given
    /// field values. A default implementation for NewUser is provided, and
    /// can be used in conjunction with the provided builder API, should a non-
    /// completed version of NewUser wish to be created.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user that will be stored in the
    /// created primitive instance
    /// * `verified` - Whether or not the user has a verified email
    /// * `nationality` - The country that the user most identifies with
    /// * `accepts_gifts` - Whether or not donations can be made to this user
    /// * `minecraft_name` - The user's name in minecraft
    pub fn new(username: &'a str, verified: bool, nationality: &'a str, accepts_gifts: bool, minecraft_name: &'a str) -> Self {
        Self {
            username,
            verified,
            nationality,
            accepts_gifts,
            minecraft_name
        }
    }

    /// Consumes an existing instance of the NewUser, and modifies it according to
    /// the provided username.
    ///
    /// # Arguments
    ///
    /// * `username` - The username that should be attached to the returned
    /// user primitive instance
    pub fn with_username(mut self, username: &'a str) -> Self {
        self.username = username;

        self
    }

    /// Consumes an existing instance of the NewUser, and modifies it according to
    /// the provided "verified" status.
    ///
    /// # Arugments
    ///
    /// * `verified` - Whether or not the user has a verified email
    pub fn with_verified(mut self, verified: bool) -> Self {
        self.verified = verified;

        self
    }

    /// Consumes an existing instance of the NewUser, and modifies it according to
    /// the provided nationality.
    ///
    /// # Arugments
    ///
    /// * `nationality` - The country that the user most identifies with
    pub fn with_nationality(mut self, nationality: &'a str) -> Self {
        self.nationality = nationality;

        self
    }

    /// Consumes an existing instance of the NewUser, and modifies it according to
    /// the provided "accepts gifts" status.
    ///
    /// # Arugments
    ///
    /// * `accepts_gifts` - Whther or not the user accepts gifts
    pub fn with_accepts_gifts(mut self, accepts_gifts: bool) -> Self {
        self.accepts_gifts = accepts_gifts;

        self
    }

    /// Consumes an existing instance of the NewUser, and modifies it according to
    /// the provided minecraft username.
    ///
    /// # Arugments
    ///
    /// * `accepts_gifts` - Whther or not the user accepts gifts
    pub fn with_minecraft_name(mut self, minecraft_name: &'a str) -> Self {
        self.minecraft_name = minecraft_name;

        self
    }
}

/// IDs represents each ID attached to each user in the database.
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "ids"]
pub struct IdMapping {
    /// The nonce of the username => id mapping
    id: u64,

    /// The username of the user
    username: String,

    /// The user ID of the user
    user_id: u64,
}

/// NewIDMapping represents a new entry mapping a username to a user ID.
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "ids"]
pub(crate) struct NewIdMapping<'a> {
    /// The username of the user
    username: &'a str,

    /// The user ID of the user
    user_id: u64,
}

impl<'a> NewIdMapping<'a> {
    /// Creates a new username <-> id mapping.
    ///
    /// # Arguments
    ///
    /// * `username` - The username that should be mapped to the ID
    /// * `user_id` - The ID to which the username should be mapped
    pub fn new(username: &'a str, user_id: u64) -> Self {
        Self { username, user_id }
    }
}

/// OauthConnection represents a generic connection to an oauth provider for a
/// gnomegg user.
pub trait OauthConnection {
    /// Retreives the identifier assigned to the gnomegg user by the oauth
    /// provider.
    fn id(&self) -> &str;

    /// Retreives a hash of the identifier associated with the provider.
    fn id_hash(&self) -> &[u8];
}

/// RedditConnection represents an oauth connection to Reddit for a gnomegg
/// user.
pub struct RedditConnection<'a> {
    /// The ID assigned to the user
    value: &'a str,

    /// The hash associated with the user
    hash: blake3::Hash,
}

impl<'a> RedditConnection<'a> {
    /// Creates a new instance of the reddit connection primitive.
    ///
    /// # Arguments
    ///
    /// * `reddit_id` - The unique identified assigned by reddit to this user
    pub fn new(reddit_id: &'a str) -> Self {
        Self {
            value: reddit_id,
            hash: blake3::hash(reddit_id.as_bytes()),
        }
    }
}

impl<'a> OauthConnection for RedditConnection<'a> {
    /// Retreives the identifier assigned to the gnomegg user by the oauth
    /// provider.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::user::{RedditConnection, OauthConnection};
    ///
    /// let reddit_conn = RedditConnection::new("123456");
    /// assert_eq!(reddit_conn.id(), "123456")
    /// ```
    fn id(&self) -> &str {
        self.value
    }

    /// Retreives a hash of the identifier associated with the provider.
    ///
    /// # Example
    ///
    /// ```
    /// use gnomegg::spec::user::{RedditConnection, OauthConnection};
    ///
    /// let reddit_conn = RedditConnection::new("123456");
    /// ```
    fn id_hash(&self) -> &[u8] {
        self.hash.as_bytes()
    }
}
