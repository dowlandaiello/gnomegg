use super::schema::{ids, roles, users};
use diesel::Insertable;
use serde::{Deserialize, Serialize};

use std::{default::Default, fmt, str::FromStr, error::Error};

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
    pub fn new(
        username: &'a str,
        verified: bool,
        nationality: &'a str,
        accepts_gifts: bool,
        minecraft_name: &'a str,
    ) -> Self {
        Self {
            username,
            verified,
            nationality,
            accepts_gifts,
            minecraft_name,
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

/// Role represents an exclusive, individual role.
#[derive(Copy, Clone, PartialEq)]
pub enum Role {
    Administrator,
    Moderator,
    VIP,
    Protected,
    Subscriber,
    Bot,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Role {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Administrator => "administrator",
            Self::Moderator => "moderator",
            Self::VIP => "vip",
            Self::Protected => "protected",
            Self::Subscriber => "subscriber",
            Self::Bot => "bot",
        }
    }
}

/// ParseRoleError represents an error encountered while converting a string
/// to a role.
#[derive(Debug)]
pub enum ParseRoleError {
    NoMatchingRole
}

impl fmt::Display for ParseRoleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no role matches the provided string")
    }
}

impl Error for ParseRoleError {}

impl FromStr for Role {
    type Err = ParseRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "administrator" => Ok(Self::Administrator),
            "moderator" => Ok(Self::Moderator),
            "vip" => Ok(Self::VIP),
            "protected" => Ok(Self::Protected),
            "subscriber" => Ok(Self::Subscriber),
            "bot" => Ok(Self::Bot),
            _ => Err(ParseRoleError::NoMatchingRole)
        }
    }
}

/// RoleEntry represents a non-exclusionary role pertaining to a given user (i.e.,
/// a user may have no roles, or all possible roles).
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "roles"]
pub struct RoleEntry {
    /// A unique identifier assigned to the role
    id: u64,

    /// The ID of the user associated with the role
    user_id: u64,

    /// Whether or not this user is an administrator
    administrator: bool,

    /// Whether or not this user is a moderator
    moderator: bool,

    /// Whether or not this user is a VIP
    vip: bool,

    /// Whether or not this user is protected
    protected: bool,

    /// Whether or not this user is a subscriber
    subscriber: bool,

    /// Whether or not this user is a bot
    bot: bool,
}

impl RoleEntry {
    /// Gets the ID of the user that the role entry is concerning.
    pub fn concerns(&self) -> u64 {
        self.user_id
    }

    /// Gets the identifier associated with the unique role entry.
    pub fn entry_id(&self) -> u64 {
        self.id
    }

    /// Determines whether or not the role entry has a given role.
    ///
    /// # Arguments
    ///
    /// * `role` - The role that should exist inside the role entry.
    pub fn has_role(&self, role: &Role) -> bool {
        match role {
            Role::Administrator => self.administrator,
            Role::Moderator => self.moderator,
            Role::VIP => self.vip,
            Role::Protected => self.protected,
            Role::Subscriber => self.subscriber,
            Role::Bot => self.bot,
        }
    }
}

/// NewRoleEntry represents a request to create a new role entry for some user.
#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug, Default)]
#[table_name = "roles"]
pub struct NewRoleEntry {
    /// The ID of the user associated with the role
    user_id: u64,

    /// Whether or not this user is an administrator
    administrator: bool,

    /// Whether or not this user is a moderator
    moderator: bool,

    /// Whether or not this user is a VIP
    vip: bool,

    /// Whether or not this user is protected
    protected: bool,

    /// Whether or not this user is a subscriber
    subscriber: bool,

    /// Whether or not this user is a bot
    bot: bool,
}

impl NewRoleEntry {
    /// Creates a new role entry with the given user and roles.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user who shall be targeted by this role
    /// entry
    /// * `roles` - The roles that should be assigned to the user
    pub fn new(user_id: u64, roles: &[Role]) -> Self {
        Self {
            user_id,
            administrator: roles.contains(&Role::Administrator),
            moderator: roles.contains(&Role::Moderator),
            vip: roles.contains(&Role::VIP),
            protected: roles.contains(&Role::Protected),
            subscriber: roles.contains(&Role::Subscriber),
            bot: roles.contains(&Role::Bot),
        }
    }

    /// Determines whether or not the role entry has a given role.
    ///
    /// # Arguments
    ///
    /// * `role` - The role that should exist inside the role entry.
    pub fn has_role(&self, role: &Role) -> bool {
        match role {
            Role::Administrator => self.administrator,
            Role::Moderator => self.moderator,
            Role::VIP => self.vip,
            Role::Protected => self.protected,
            Role::Subscriber => self.subscriber,
            Role::Bot => self.bot,
        }
    }
}
