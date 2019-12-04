use std::str::FromStr;
use std::error::Error;
use std::fmt::{self, Display};

/// A header name.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct HeaderName {
    inner: &'static str,
}

impl HeaderName {
    // /// Create a new `HeaderName`.
    // pub fn from_ascii(name: bytes: Vec<u8>) -> Self {
    //     Self { inner: name }
    // }

    /// Converts a vector of bytes to a `HeaderName` without checking that the string contains
    /// valid ASCII.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed to it are valid
    /// ASCII. If this constraint is violated, it may cause memory unsafety issues with future
    /// users of the HeaderName, as the rest of the library assumes that Strings are valid ASCII.
    pub const unsafe fn from_ascii_unchecked(bytes: Vec<u8>) -> Self {
        unimplemented!();
    }
}

/// An error returned when failing to convert into a header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseError {
    _private: (),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Error parsing a string into a status code".fmt(f)
    }
}

impl FromStr for HeaderName {
    type Err = ParseError;

    /// Create a new `HeaderName`.
    /// 
    /// This checks it's valid ASCII, and lowercases it.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseError { _private: () })
        }
        Ok(Self { inner: &s.to_ascii_lowercase() })
    }
}
