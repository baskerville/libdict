//! Errors for the Dict dictionary crate.
use std::error;

/// Error type, representing the errors which can be returned by the libdict library.
///
/// This enum represents a handful of custom errors and wraps `io:::Error` and
/// `string::FromUtf8Error`.
#[derive(Debug)]
pub enum DictError {
    /// Invalid character, e.g. within the index file; the error contains the erorneous character,
    /// andd optionally line and position.
    InvalidCharacter(char, Option<usize>, Option<usize>),
    /// Occurs whenever a line in an index file misses a column.
    MissingColumnInIndex(usize),
    /// Invalid file format, contains an explanation an an optional path to the
    /// file with the invalid file format.
    InvalidFileFormat(String, Option<String>),
    /// This reports a malicous / malformed index file, which requests a buffer which is too large.
    MemoryError,
    /// This reports words which are not present in the dictionary.
    WordNotFound(String),
    /// A wrapped io::Error.
    IoError(::std::io::Error),
    /// A wrapped Utf8Error.
    Utf8Error(::std::string::FromUtf8Error),
}

impl ::std::fmt::Display for DictError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DictError::IoError(ref e) => e.fmt(f),
            DictError::Utf8Error(ref e) => e.fmt(f),
            DictError::MemoryError => write!(f, "not enough memory available"),
            DictError::WordNotFound(ref word) => write!(f, "Word not found: {}", word),
            DictError::InvalidCharacter(ref ch, ref line, ref pos) =>
                write!(f, "Invalid character {}{}{}", ch,
                        match *line {
                            Some(ln) => format!(" on line {}", ln),
                            _ => String::new() // ToDo: more leegant solution
                        },
                        match *pos {
                            Some(pos) => format!(" at position {}", pos),
                            _ => String::new() // ToDo: more leegant solution
                        }),
            DictError::MissingColumnInIndex(ref lnum) => write!(f, "line {}: not \
                    enough <tab>-separated columns found, expected 3", lnum),
            DictError::InvalidFileFormat(ref explanation, ref path) =>
                write!(f, "{}{}", path.clone().unwrap_or_else(String::new), explanation)
        }
    }
}

impl error::Error for DictError {
    fn description(&self) -> &str {
        match *self {
            DictError::InvalidCharacter(_, _, _) => "invalid character",
            DictError::MemoryError => "not enough memory available",
            DictError::WordNotFound(_) => "word not found",
            DictError::MissingColumnInIndex(_) =>
                    "not enough <tab>-separated columnss given",
            DictError::InvalidFileFormat(ref _explanation, ref _path) => "could not \
                    determine file format",
            DictError::IoError(ref err) => err.description(),
            DictError::Utf8Error(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DictError::IoError(ref err) => err.cause(),
            DictError::Utf8Error(ref err) => err.cause(),
            _ => None,
        }
    }
}

// allow seamless coercion from::Error 
impl From<::std::io::Error> for DictError {
    fn from(err: ::std::io::Error) -> DictError {
        DictError::IoError(err)
    }
}

impl From<::std::string::FromUtf8Error> for DictError {
    fn from(err: ::std::string::FromUtf8Error) -> DictError {
        DictError::Utf8Error(err)
    }
}

