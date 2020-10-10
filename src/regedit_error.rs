use std::array::TryFromSliceError;
use std::fmt;
use std::fmt::Formatter;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

pub struct RegeditError {
    err: String,
}

impl RegeditError {
    pub fn new<T: AsRef<str>>(err: T) -> Self {
        Self {
            err: err.as_ref().to_owned(),
        }
    }
}

impl std::error::Error for RegeditError {}

impl std::fmt::Debug for RegeditError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Display for RegeditError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl From<TryFromSliceError> for RegeditError {
    fn from(e: TryFromSliceError) -> Self {
        Self::new(e.to_string())
    }
}

impl From<TryFromIntError> for RegeditError {
    fn from(e: TryFromIntError) -> Self {
        Self::new(e.to_string())
    }
}

impl From<FromUtf8Error> for RegeditError {
    fn from(e: FromUtf8Error) -> Self {
        Self::new(e.to_string())
    }
}
