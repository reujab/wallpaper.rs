use std::{io, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("I/O Error: {0}")]
    IOError(#[from] io::Error),

    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] FromUtf8Error),

    #[cfg(all(unix, not(target_os = "macos")))]
    #[error("Invalid INI: {0}")]
    InvalidIni(#[from] ini::Error),

    #[cfg(unix)]
    #[error("Enquote error: {0}")]
    Enquote(#[from] enquote::Error),

    #[error("{command} exited with status code {code}")]
    CommandFailed { command: String, code: i32 },

    #[error("Could not find config directory")]
    NoConfigDir,

    #[error("No {0} image found")]
    NoImage(&'static str),

    #[cfg(all(unix, not(target_os = "macos")))]
    #[error("No desktops found")]
    XfceNoDesktops,

    #[error("Unsupported Desktop")]
    UnsupportedDesktop,

    #[error("Invalid path")]
    InvalidPath,

    #[error("FromUtf16Error: {0}")]
    FromUtf16Error(#[from] std::string::FromUtf16Error),

    #[cfg(feature = "from_url")]
    #[error("reqwest::Error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("{0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}

impl From<&String> for Error {
    fn from(s: &String) -> Self {
        Error::Other(s.to_string())
    }
}
