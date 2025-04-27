/// `Error` represents various possible errors returned within the `iocore` crate
#[derive(Debug, Clone)]
pub enum Error {
    /// `Error::IOError` wraps [`std::io::Error`]
    IOError(String),
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
    WalkDirError(String),
    PathScanningError(String),
    PathDoesNotExist(String),
    MalformedFileName(String),
    ThreadGroupError(String),
    ShellCommandError(String),
    ParseError(String),
    PatternMismatch(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(error) => write!(f, "IOError: {}", error),
            Error::FileSystemError(error) => write!(f, "FileSystemError: {}", error),
            Error::MalformedGlobPattern(error) => write!(f, "MalformedGlobPattern: {}", error),
            Error::HomePathError(error) => write!(f, "HomePathError: {}", error),
            Error::ReadDirError(error) => write!(f, "ReadDirError: {}", error),
            Error::SafetyError(error) => write!(f, "SafetyError: {}", error),
            Error::PathDeserializationError(error) =>
                write!(f, "PathDeserializationError: {}", error),
            Error::IOCoreException(error) => write!(f, "IOCoreException: {}", error),
            Error::SubprocessError(error) => write!(f, "SubprocessError: {}", error),
            Error::SystemError(error) => write!(f, "SystemError: {}", error),
            Error::ChannelError(error) => write!(f, "ChannelError: {}", error),
            Error::PathConversionError(error) => write!(f, "PathConversionError: {}", error),
            Error::EnvironmentVarError(s) => write!(f, "EnvironmentVarError: {}", s),
            Error::UnexpectedPathType(error) => write!(f, "UnexpectedPathType: {}", error),
            Error::WalkDirError(error) => write!(f, "WalkDirError: {}", error),
            Error::PathScanningError(error) => write!(f, "PathScanningError: {}", error),
            Error::PathDoesNotExist(error) => write!(f, "PathDoesNotExist: {}", error),
            Error::MalformedFileName(error) => write!(f, "MalformedFileName: {}", error),
            Error::ThreadGroupError(error) => write!(f, "ThreadGroupError: {}", error),
            Error::ShellCommandError(error) => write!(f, "ShellCommandError: {}", error),
            Error::ParseError(error) => write!(f, "ParseError: {}", error),
            Error::PatternMismatch(error) => write!(f, "PatternMismatch: {}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.to_string())
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
        $crate::Error::$variant(format!("{} [{}:[{}:{}]]\n", $error, name, file!(), line!()))
    }};
    ($variant:ident, $format:literal, $arg:expr  ) => {{
        $crate::traceback!($variant, format!($format, $arg))
    }};
    ($variant:ident, $format:literal, $( $arg:expr ),* ) => {{
        $crate::traceback!($variant, format!($format, $($arg,)*))
    }};
}
