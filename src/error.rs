use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidUserRef(String),
    Io(std::io::Error),
    Json(serde_json::Error),
    Other(&'static str),
    Reqwest(reqwest::Error),
    UnconsistentData(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidUserRef(s) => write!(f, "invalid user ref: {s:?}"),
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::Json(e) => write!(f, "Json error: {e}"),
            Self::Other(s) => write!(f, "error: {}", s),
            Self::Reqwest(e) => write!(f, "reqwest error: {e}"),
            Self::UnconsistentData(s) => write!(f, "unconsistent data: {s}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
