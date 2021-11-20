//! The crate's error module, for error-related types and implementations.

use std::error;

use std::fmt;
use std::fmt::{Display, Formatter};

/// An Error struct for when things go wrong in subcommands.
/// 
/// It's just a container for a String. Implements From for
/// any other errors that it's convenient to implicitly convert.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug)]
pub enum Error {
    Message(String),
    FromError(Box<dyn error::Error>)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(message) => {
                write!(f, "{}", message)
            }
            Error::FromError(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl<E: error::Error + 'static> From<E> for Error {
    fn from(e: E) -> Self {
        Self::FromError(Box::new(e))
    }
}
