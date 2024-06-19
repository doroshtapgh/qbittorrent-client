use std::{
    fmt,
    error::Error
};

#[derive(Debug)]
pub enum QBittorrentError {
    AuthFailed,
    BadRequest,
    BadInput(String),
    Url(url::ParseError),
    Reqwest(reqwest::Error)
}

impl fmt::Display for QBittorrentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QBittorrentError::AuthFailed => write!(f, "failed to log in"),
            QBittorrentError::BadRequest => write!(f, "bad request http error occured"),
            QBittorrentError::BadInput(ref err) => write!(f, "bad input error occured: {}", err),
            QBittorrentError::Url(ref err) => write!(f, "url error occured: {}", err),
            QBittorrentError::Reqwest(ref err) => write!(f, "reqwest error occured: {}", err)
        }
    }
}

impl Error for QBittorrentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            QBittorrentError::AuthFailed => None,
            QBittorrentError::BadRequest => None,
            QBittorrentError::BadInput(..) => None,
            QBittorrentError::Url(ref err) => Some(err),
            QBittorrentError::Reqwest(ref err) => Some(err)
        }
    }
}

impl From<url::ParseError> for QBittorrentError {
    fn from(err: url::ParseError) -> QBittorrentError {
        QBittorrentError::Url(err)
    }
}

impl From<reqwest::Error> for QBittorrentError {
    fn from(err: reqwest::Error) -> QBittorrentError {
        QBittorrentError::Reqwest(err)
    }
}

