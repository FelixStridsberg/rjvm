use crate::error::ErrorKind::{RuntimeError, IO};
use crate::error::Repr::Simple;
use std::fmt::{Display, Formatter};
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

/// Convenience macro for returning runtime error.
/// Works the same way as `panic!` but returns a `Result` with error kind `RuntimeError`.
#[macro_export]
macro_rules! runtime_error(
    ($($fmt:tt)*) => {
        Err(crate::error::Error::runtime(format!($($fmt)*)))
    }
);

/// Represents errors that may occur when reading or writing osm.
#[derive(Debug)]
pub struct Error {
    repr: Repr,
    message: Option<String>,
}

/// It will make it possible to change internals without breaking change.
#[derive(Debug)]
enum Repr {
    Simple(ErrorKind),
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Parse error. E.g. invalid data in file.
    ParseError,

    /// IO error. E.g. file not found, permission denied.
    IO(io::Error),

    /// Error during runtime.
    RuntimeError,
}

impl Error {
    pub fn new(kind: ErrorKind, message: Option<String>) -> Self {
        Error {
            repr: Simple(kind),
            message,
        }
    }

    pub fn runtime(message: String) -> Self {
        Error {
            repr: Simple(RuntimeError),
            message: Some(message),
        }
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    /// Returns reference to error kind.
    pub fn kind(&self) -> &ErrorKind {
        match &self.repr {
            Simple(e) => &e,
        }
    }
}

impl ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IO(e) => Some(e),
            _ => None,
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ErrorKind::ParseError => write!(f, "Unknown parse error occurred.")?,
            IO(io_error) => match io_error.kind() {
                io::ErrorKind::UnexpectedEof => write!(f, "Unexpected end of file.")?,
                _ => write!(f, "IO error: {}", io_error)?,
            },
            ErrorKind::RuntimeError => write!(f, "Runtime error.")?,
        };
        Ok(())
    }
}

impl Repr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Simple(kind) => kind.source(),
        }
    }
}

impl Display for Repr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let Simple(kind) = self;
        kind.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.repr.source()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match &self.message {
            Some(message) => write!(f, "{}", message),
            None => self.repr.fmt(f),
        }
    }
}

/// Errors can not really be compared. This is to allow the results to be compared when they are Ok.
impl PartialEq for Error {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            repr: Simple(IO(e)),
            message: None,
        }
    }
}
