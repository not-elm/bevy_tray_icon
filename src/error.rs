//! Defines the library result and error.

use thiserror::Error;

/// The library result
pub type Result<V = ()> = std::result::Result<V, Error>;

/// The library error
#[derive(Debug, Eq, PartialEq, Error, Clone)]
pub enum Error {
    /// If the menu registered in the system tray is not found
    #[error("not found menu id: {0}")]
    NotFoundMenu(String),
}
