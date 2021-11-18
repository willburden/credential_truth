//! The crate's error module, for error-related types and implementations.

use std::error;

use std::fmt;
use std::fmt::{Display, Formatter};

/// An Error struct for when things go wrong in subcommands.
/// 
/// It's just a container for a String. Implements From for
/// any other errors that it's convenient to implicitly convert.
#[derive(Debug)]
pub struct Error(
    /// The error message.
    pub String
);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E> From<E> for Error
where E: error::Error {
    fn from(e: E) -> Self {
        Self(e.to_string())
    }
}
