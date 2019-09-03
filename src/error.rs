//! Errors 🤷

use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct InvalidFragment;

#[derive(Clone, Copy, Debug)]
pub struct InvalidPath;

/// Convenient wrapper around `std::Result`.
pub type Result<T> = ::std::result::Result<T, Error>;

/// The Error type.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::SerdeJson(ref e) => write!(f, "{}", e),
        }
    }
}

/// The kind of an error.
#[derive(Debug)]
pub enum ErrorKind {
    SerdeJson(serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error {
            kind: ErrorKind::SerdeJson(e),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

/// Just the usual bail macro
#[macro_export]
#[doc(hidden)]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(format!($fmt, $($arg)+).into());
    };
}

/// Exits a function early with an `Error` if the condition is not satisfied.
///
/// Similar to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`,
/// it does not panic.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
}
