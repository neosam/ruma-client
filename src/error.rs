use hyper::Error as HyperError;
use serde_json::Error as SerdeJsonError;
use url::ParseError;
use ruma_identifiers::Error as RumaIdentifiersError;

/// An error that occurs during client operations.
#[derive(Debug)]
pub enum Error {
    /// An error at the HTTP layer.
    Hyper(HyperError),
    /// An error when parsing a string as a URL.
    Url(ParseError),
    /// An error when serializing or deserializing a value.
    SerdeJson(SerdeJsonError),
    /// An error accured while parsing the ruma identifiers.
    RumaIdentifiers(RumaIdentifiersError)
}

impl From<HyperError> for Error {
    fn from(error: HyperError) -> Error {
        Error::Hyper(error)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Error {
        Error::Url(error)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(error: SerdeJsonError) -> Error {
        Error::SerdeJson(error)
    }
}

impl From<RumaIdentifiersError> for Error {
    fn from(error: RumaIdentifiersError) -> Error {
        Error::RumaIdentifiers(error)
    }
}

