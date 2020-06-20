use std::fmt::{self, Display, Formatter};

mod key_error;
mod type_error;
mod value_error;

pub use type_error::TypeError;
pub use key_error::KeyError;
pub use value_error::ValueError;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
	/// Something internal that shouldn't have occured
	Internal(&'static str),
	/// Something that we don't have an error type for yet
	Messaged(String),
	/// An invalid key was requested
	KeyError(KeyError),
	/// An invalid type was supplied somewhere
	TypeError(TypeError),
	/// An invalid value was supplied somewhere
	ValueError(ValueError),
}

impl From<String> for Error {
	fn from(err: String) -> Self { Error::Messaged(err) }
}

#[deprecated]
impl From<crate::types::Text> for Error {
	fn from(err: crate::types::Text) -> Self { Error::Messaged(err.into()) }
}

#[deprecated]
impl From<&'_ str> for Error {
	fn from(err: &'_ str) -> Self { Error::Messaged(err.into()) }
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Error::Internal(err) => write!(f, "internal error: {}", err),
			Error::Messaged(err) => Display::fmt(&err, f),
			Error::KeyError(err) => Display::fmt(&err, f),
			Error::TypeError(err) => Display::fmt(&err, f),
			Error::ValueError(err) => Display::fmt(&err, f),
		}
	}
}

impl std::error::Error for Error {
	// there's no cause because the sub errors are just instances of this error.
}

#[must_use]
pub type Result<T> = ::std::result::Result<T, Error>;