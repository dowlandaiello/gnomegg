use super::schema::{ids, roles, users};
use diesel::{
    expression::BoxableExpression,
    mysql::Mysql,
    query_builder::SqlQuery,
    sql_types::{Bool, Nullable},
    Insertable,
};
use serde::{Deserialize, Serialize};

use std::{convert::Into, default::Default, error::Error, fmt, str::FromStr};

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

    /// Constructs a raw SQL query for the Role with the given role status.
    ///
    /// # Arguments
    ///
    /// * `has_role` - Whether or not the user has the rol
    pub(crate) fn construct_give_role_statement(&self, user_id: u64, has_role: bool) -> SqlQuery {
        diesel::sql_query(format!(
            "IF EXISTS (SELECT * FROM roles WHERE user_id = {})
                UPDATE roles SET {} = {} WHERE user_id = {}
            ELSE
                INSERT INTO roles(user_id, {}) VALUES({}, {})
            END",
            user_id,
            self.to_str(),
            has_role,
            user_id,
            self.to_str(),
            user_id,
            has_role
        ))
    }
}

impl From<&Role> for Box<dyn BoxableExpression<roles::table, Mysql, SqlType = Nullable<Bool>>> {
    fn from(r: &Role) -> Self {
        match r {
            Role::Administrator => Box::new(roles::dsl::administrator),
            Role::Moderator => Box::new(roles::dsl::moderator),
            Role::VIP => Box::new(roles::dsl::vip),
            Role::Protected => Box::new(roles::dsl::protected),
            Role::Subscriber => Box::new(roles::dsl::subscriber),
            Role::Bot => Box::new(roles::dsl::bot),
        }
    }
}

/// ParseRoleError represents an error encountered while converting a string
/// to a role.
#[derive(Debug)]
pub enum ParseRoleError {
    NoMatchingRole,
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
            _ => Err(ParseRoleError::NoMatchingRole),
        }
    }
}

/// RoleEntry represents a non-exclusionary role pertaining to a given user (i.e.,
/// a user may have no roles, or all possible roles).
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Default)]
#[belongs_to(User)]
#[table_name = "roles"]
pub struct RoleEntry {
    /// A unique identifier assigned to the role
    id: u64,

    /// The ID of the user associated with the role
    user_id: u64,

    /// Whether or not this user is an administrator
    administrator: Option<bool>,

    /// Whether or not this user is a moderator
    moderator: Option<bool>,

    /// Whether or not this user is a VIP
    vip: Option<bool>,

    /// Whether or not this user is protected
    protected: Option<bool>,

    /// Whether or not this user is a subscriber
    subscriber: Option<bool>,

    /// Whether or not this user is a bot
    bot: Option<bool>,
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
            Role::Administrator => self.administrator.unwrap_or(false),
            Role::Moderator => self.moderator.unwrap_or(false),
            Role::VIP => self.vip.unwrap_or(false),
            Role::Protected => self.protected.unwrap_or(false),
            Role::Subscriber => self.subscriber.unwrap_or(false),
            Role::Bot => self.bot.unwrap_or(false),
        }
    }
}

impl From<&RoleEntry> for Vec<Role> {
    fn from(entry: &RoleEntry) -> Self {
        let mut roles = Vec::new();

        if let Some(r) = entry.administrator {
            if r {
                roles.push(Role::Administrator);
            }
        }

        if let Some(r) = entry.moderator {
            if r {
                roles.push(Role::Moderator);
            }
        }

        if let Some(r) = entry.vip {
            if r {
                roles.push(Role::VIP);
            }
        }

        if let Some(r) = entry.protected {
            if r {
                roles.push(Role::Protected);
            }
        }

        if let Some(r) = entry.subscriber {
            if r {
                roles.push(Role::Subscriber);
            }
        }

        if let Some(r) = entry.bot {
            if r {
                roles.push(Role::Bot);
            }
        }

        roles
    }
}

/// NewRoleEntry represents a request to create a new role entry for some user.
#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug, Default)]
#[table_name = "roles"]
pub struct NewRoleEntry {
    /// The ID of the user associated with the role
    user_id: u64,

    /// Whether or not this user is an administrator
    administrator: Option<bool>,

    /// Whether or not this user is a moderator
    moderator: Option<bool>,

    /// Whether or not this user is a VIP
    vip: Option<bool>,

    /// Whether or not this user is protected
    protected: Option<bool>,

    /// Whether or not this user is a subscriber
    subscriber: Option<bool>,

    /// Whether or not this user is a bot
    bot: Option<bool>,
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
            administrator: Some(roles.contains(&Role::Administrator)),
            moderator: Some(roles.contains(&Role::Moderator)),
            vip: Some(roles.contains(&Role::VIP)),
            protected: Some(roles.contains(&Role::Protected)),
            subscriber: Some(roles.contains(&Role::Subscriber)),
            bot: Some(roles.contains(&Role::Bot)),
        }
    }

    /// Determines whether or not the role entry has a given role.
    ///
    /// # Arguments
    ///
    /// * `role` - The role that should exist inside the role entry.
    pub fn has_role(&self, role: &Role) -> bool {
        match role {
            Role::Administrator => self.administrator.unwrap_or(false),
            Role::Moderator => self.moderator.unwrap_or(false),
            Role::VIP => self.vip.unwrap_or(false),
            Role::Protected => self.protected.unwrap_or(false),
            Role::Subscriber => self.subscriber.unwrap_or(false),
            Role::Bot => self.bot.unwrap_or(false),
        }
    }
}
