use crate::{Depth, Path};

/// `Error` represents various possible errors returned within the `iocore` crate
#[derive(Debug, Clone)]
pub enum Error {
    /// `Error::IOError` wraps [`std::io::Error`]
    IOError(std::io::ErrorKind),
    /// `Error::FileSystemError` represents filesystem-related errors
    FileSystemError(String),
    MalformedGlobPattern(String),
    HomePathError(String),
    ReadDirError(String),
    SafetyError(String),
    /// `Error::EnvironmentVarError` represents error while obtaining a environment variable
    EnvironmentVarError(String),
    IOCoreException(String),
    SubprocessError(String),
    SystemError(String),
    ChannelError(String),
    PathConversionError(String),
    PathDeserializationError(String),
    UnexpectedPathType(String),
    WalkDirError(String, Path, Depth),
    PathScanningError(String),
    PathDoesNotExist(Path),
    MalformedFileName(String),
    ThreadGroupError(String),
    ShellCommandError(String),
    ParseError(String),
    PatternMismatch(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "IOError: {}", e),
            Error::FileSystemError(e) => write!(f, "FileSystemError: {}", e),
            Error::MalformedGlobPattern(e) => write!(f, "MalformedGlobPattern: {}", e),
            Error::HomePathError(e) => write!(f, "HomePathError: {}", e),
            Error::ReadDirError(e) => write!(f, "ReadDirError: {}", e),
            Error::SafetyError(e) => write!(f, "SafetyError: {}", e),
            Error::PathDeserializationError(e) => write!(f, "PathDeserializationError: {}", e),
            Error::IOCoreException(e) => write!(f, "IOCoreException: {}", e),
            Error::SubprocessError(e) => write!(f, "SubprocessError: {}", e),
            Error::SystemError(e) => write!(f, "SystemError: {}", e),
            Error::ChannelError(e) => write!(f, "ChannelError: {}", e),
            Error::PathConversionError(e) => write!(f, "PathConversionError: {}", e),
            Error::EnvironmentVarError(s) => write!(f, "EnvironmentVarError: {}", s),
            Error::UnexpectedPathType(e) => write!(f, "UnexpectedPathType: {}", e),
            Error::WalkDirError(e, path, depth) =>
                write!(f, "WalkDirError {}(depth={}): {}", path, depth, e),
            Error::PathScanningError(path) => write!(f, "PathScanningError: {}", path),
            Error::PathDoesNotExist(path) => write!(f, "PathDoesNotExist: {}", path),
            Error::MalformedFileName(e) => write!(f, "MalformedFileName: {}", e),
            Error::ThreadGroupError(e) => write!(f, "ThreadGroupError: {}", e),
            Error::ShellCommandError(e) => write!(f, "ShellCommandError: {}", e),
            Error::ParseError(e) => write!(f, "ParseError: {}", e),
            Error::PatternMismatch(e) => write!(f, "PatternMismatch: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.kind())
    }
}
impl From<thread_groups::Error> for Error {
    fn from(e: thread_groups::Error) -> Self {
        Error::ThreadGroupError(e.to_string())
    }
}
impl From<sanitation::Error<'_>> for Error {
    fn from(e: sanitation::Error<'_>) -> Self {
        Error::SafetyError(e.to_string())
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseError(e.to_string())
    }
}
impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::ParseError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for Error {}

#[macro_export]
macro_rules! traceback {
    ($variant:ident, $error:expr ) => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name.strip_suffix("::f").unwrap();
        $crate::Error::$variant(format!("{} [{}:[{}:{}]]", $error, name, file!(), line!()))
    }};
    ($variant:ident, $format:literal, $arg:expr  ) => {{
        $crate::traceback!($variant, format!($format, $arg))
    }};
    ($variant:ident, $format:literal, $( $arg:expr ),* ) => {{
        $crate::traceback!($variant, format!($format, $($arg,)*))
    }};
}
