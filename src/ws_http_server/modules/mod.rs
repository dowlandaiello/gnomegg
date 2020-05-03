use diesel::{result::Error as DieselError, mysql::MysqlConnection};
use redis::{Connection, RedisError};
use serde_json::Error as SerdeError;

use std::{error::Error, fmt};

pub mod bans;
pub mod mutes;
pub mod name_resolver;
pub mod oauth;
pub mod roles;

/// ProviderError represents any error emitted by a ban backend.
#[derive(Debug)]
pub enum ProviderError {
    RedisError(RedisError),
    SerdeError(SerdeError),
    DieselError(DieselError),
    MissingArgument { arg: &'static str },
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RedisError(err) => write!(f, "the provider encountered a redis error: {}", err),
            Self::SerdeError(err) => {
                write!(f, "the provider encountered a serialization error: {}", err)
            }
            Self::DieselError(err) => {
                write!(f, "the provider encountered a database error: {}", err)
            }
            Self::MissingArgument { arg } => {
                write!(f, "malformed query; missing argument: {}", arg)
            }
        }
    }
}

impl Error for ProviderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RedisError(e) => Some(e),
            Self::SerdeError(e) => Some(e),
            Self::DieselError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<RedisError> for ProviderError {
    /// Constructs a provider error from the given redis error.
    ///
    /// # Arguments
    ///
    /// * `e` - The redis error that should be wrapped in the ProviderError
    fn from(e: RedisError) -> Self {
        Self::RedisError(e)
    }
}

impl From<SerdeError> for ProviderError {
    /// Constructs a provider error from the given serde error.
    ///
    /// # Arguments
    ///
    /// * `e` - The serde error that should be wrapped in the ProviderError
    fn from(e: SerdeError) -> Self {
        Self::SerdeError(e)
    }
}

impl From<DieselError> for ProviderError {
    /// Cosntructs a provider error from the given diesel error.
    ///
    /// # Arguments
    ///
    /// * `e` - The diesel error that should be wrapped in the ProviderError
    fn from(e: DieselError) -> Self {
        Self::DieselError(e)
    }
}

/// Cache is a connection helper to a redis database running remotely or
/// locally.
pub struct Cache<'a> {
    connection: &'a mut Connection,
}

impl<'a> Cache<'a> {
    /// Creates a new cache connection with the given remote database address.
    ///
    /// # Arguments
    ///
    /// * `database_address` - The address corresponding to the remote redis
    /// session, formatted as such: 127.0.0.1:6379
    pub fn new(connection: &'a mut Connection) -> Self {
        Self { connection }
    }
}

/// Persistent is a mysql-based persistence layer for the gnomegg bans backend.
pub struct Persistent<'a> {
    connection: &'a MysqlConnection,
}

impl<'a> Persistent<'a> {
    /// Creates a new connection to the mysql backend, and provides
    pub fn new(connection: &'a MysqlConnection) -> Self {
        Self { connection }
    }
}
