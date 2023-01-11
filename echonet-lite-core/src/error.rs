use crate::lib::{Box, String, ToString};

use crate::io;
use crate::lib::{fmt, str::Utf8Error};

/// The result of a serialization or deserialization operation.
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error that can be produced during (de)serializing.
pub type Error = Box<ErrorKind>;

/// The kind of error that can be produced during a serialization or deserialization.
#[derive(Debug)]
pub enum ErrorKind {
    /// If the error stems from the reader/writer that is being used
    /// during (de)serialization, that error will be stored and returned here.
    Io(io::Error),
    /// Returned if the deserializer attempts to deserialize a string that is not valid utf8
    InvalidUtf8Encoding(Utf8Error),
    /// Returned if the deserializer attempts to deserialize a bool that was
    /// not encoded as either a 1 or a 0
    InvalidBoolEncoding(u8),
    /// Returned if the deserializer attempts to deserialize a char that is not in the correct format.
    InvalidCharEncoding,
    /// Returned if the deserializer attempts to deserialize the tag of an enum that is
    /// not in the expected ranges
    InvalidTagEncoding(usize),
    /// Serde has a deserialize_any method that lets the format hint to the
    /// object which route to take in deserializing.
    DeserializeAnyNotSupported,
    /// If (de)serializing a message takes more than the provided size limit, this
    /// error is returned.
    SizeLimit,
    /// echonet-lite-rs can not encode sequences of unknown length (like iterators).
    SequenceMustHaveLength,
    /// A custom error message from Serde.
    Custom(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Io(ref ioerr) => write!(fmt, "io error: {}", ioerr),
            ErrorKind::InvalidUtf8Encoding(ref e) => write!(fmt, "InvalidUtf8Encoding: {}", e),
            ErrorKind::InvalidBoolEncoding(b) => {
                write!(fmt, "InvalidBoolEncoding, expected 0 or 1, found {}", b)
            }
            ErrorKind::InvalidCharEncoding => write!(fmt, "InvalidCharEncoding"),
            ErrorKind::InvalidTagEncoding(tag) => {
                write!(fmt, "InvalidTagEncoding, found {}", tag)
            }
            ErrorKind::SequenceMustHaveLength => {
                write!(fmt, "SequenceMustHaveLength")
            }
            ErrorKind::SizeLimit => write!(fmt, "SizeLimit"),
            ErrorKind::DeserializeAnyNotSupported => {
                write!(
                    fmt,
                    "EchonetLite-rs does not support the serde::Deserializer::deserialize_any method"
                )
            }
            ErrorKind::Custom(ref s) => s.fmt(fmt),
        }
    }
}

impl serde::de::StdError for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn serde::de::StdError + 'static)> {
        match **self {
            ErrorKind::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(desc: T) -> Error {
        ErrorKind::Custom(desc.to_string()).into()
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ErrorKind::Custom(msg.to_string()).into()
    }
}
