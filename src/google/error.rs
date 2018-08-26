use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Http(hyper::Error),
    Serialization(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err)
    }
}