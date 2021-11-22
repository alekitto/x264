use crate::fmt::{Debug, Display, Formatter};
use crate::result;

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
/// Indicates that something bad happened.
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self, f)
    }
}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for Error {}

/// A specialized Result type for video encoding operations.
pub type Result<T> = result::Result<T, Error>;

#[cfg(feature = "anyhow")]
impl From<Error> for anyhow::Error {
    fn from(e: Error) -> Self {
        anyhow::Error::msg(e)
    }
}
