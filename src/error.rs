extern crate csv;
extern crate serde_json;
use std::{io, string, fmt, error};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Encoding(string::FromUtf8Error),
    JSON(serde_json::Error),
    CSV(csv::Error),
    Missing(&'static str),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err)       => err.description(),
            Error::Encoding(ref err) => err.description(),
            Error::JSON(ref err)     => err.description(),
            Error::CSV(ref err)      => err.description(),
            Error::Missing(_)        => "Option returned None",
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err)       => Some(err),
            Error::Encoding(ref err) => Some(err),
            Error::JSON(ref err)     => Some(err),
            Error::CSV(ref err)      => Some(err),
            _                        => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err)       => write!(f, "I/O error: {}", err),
            Error::Encoding(ref err) => write!(f, "Encoding error: {}", err),
            Error::JSON(ref err)     => write!(f, "JSON error: {}", err),
            Error::CSV(ref err)      => write!(f, "CSV error: {}", err),
            Error::Missing(ref err)  => write!(f, "{}() returned None.", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::Encoding(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Error {
        Error::CSV(err)
    }
}
