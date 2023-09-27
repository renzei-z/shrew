pub mod request;
pub mod response;
pub mod stream;

use crate::request::RequestError;
use crate::stream::StreamError;

pub enum Error {
    RequestError(RequestError),
    StreamError(StreamError)
}

#[derive(Default)]
pub enum HttpVersion {
    One,
    #[default]
    OnePointOne,
    Two
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::One => "1",
            Self::OnePointOne => "1.1",
            Self::Two => "2"
        })
    }
}

#[derive(Default, Clone, Copy)]
pub enum HttpStatus {
    #[default]
    OK = 200,
    TemporaryRedirect = 307,
    NotImplemented = 501
}

impl std::fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::OK => "OK",
            Self::TemporaryRedirect => "Temporary Redirect",
            Self::NotImplemented => "Not Implemented"
        })
    }
}