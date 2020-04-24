use diesel::{Insertable, dsl::Find};
use serde::{Deserialize, Serialize};
use super::schema::{users, ids};

/// User represents a generic gnome.gg user.
#[derive(Insertable, Identifiable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    /// The user's unique identifier
    id: i32,

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
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub(crate) struct NewUser<'a> {
    /// The username of the user
    username: &'a str,

    /// Whether or not the user has a verified email
    verified: bool,

    /// The country that the user most identifies with
    nationality: &'a str,

    /// Whether or not the user accepts gifts
    accepts_gifts: bool,

    /// The user's minecraft username
    minecraft_name: &'a str
}

/// IDs represents each ID attached to each user in the database.
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "ids"]
pub struct IdMapping {
    /// The nonce of the username => id mapping
    id: i32,

    /// The username of the user
    username: String,

    /// The user ID of the user
    user_id: i32,
}

/// NewIDMapping represents a new entry mapping a username to a user ID.
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "ids"]
pub(crate) struct NewIdMapping<'a> {
    /// The username of the user
    username: &'a str,

    /// The user ID of the user
    user_id: i32
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
    hash: &'a [u8]
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
    /// let reddit_conn = RedditConnection{hash: b"mitta mitt mooo", value: "123456"};
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
    /// use gnomegg::spec::user{RedditConnection, OauthConnection};
    ///
    /// let reddit_conn = RedditConnection{hash: b"mitta mitt mooo", value: "123456"};
    /// assert_eq!(reddit_conn.id_hash(), b"mitta mitt mooo")
    /// ```
    fn id_hash(&self) -> &[u8] {
        self.hash
    }
}
