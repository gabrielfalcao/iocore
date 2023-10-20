use std::net::AddrParseError;
use std::string::FromUtf8Error;

#[derive(Debug, Clone)]
pub enum Exception {
    IOError(String),
    WalkDirError(String),
    FileSystemError(String),
    InvalidUtf8(FromUtf8Error),
    AddrParseError(AddrParseError),
    IOCoreException(String),
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Exception::IOError(e) => write!(f, "I/O Exception: {}", e),
            Exception::WalkDirError(e) => write!(f, "WalkDirError: {}", e),
            Exception::FileSystemError(e) => write!(f, "FileSystemError: {}", e),
            Exception::IOCoreException(e) => write!(f, "IOCoreException: {}", e),
            Exception::InvalidUtf8(s) => write!(f, "InvalidUtf8: {}", s),
            Exception::AddrParseError(s) => write!(f, "Invalid Network Address: {}", s),
        }
    }
}

impl std::error::Error for Exception {}

impl From<std::io::Error> for Exception {
    fn from(e: std::io::Error) -> Self {
        Exception::IOError(format!("{}", e))
    }
}
impl From<walkdir::Error> for Exception {
    fn from(e: walkdir::Error) -> Self {
        Exception::WalkDirError(format!("{}", e))
    }
}
impl From<FromUtf8Error> for Exception {
    fn from(e: FromUtf8Error) -> Self {
        Exception::InvalidUtf8(e)
    }
}
impl From<AddrParseError> for Exception {
    fn from(e: AddrParseError) -> Self {
        Exception::AddrParseError(e)
    }
}
