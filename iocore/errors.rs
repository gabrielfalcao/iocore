use crate::{FileSystemError, FileSystemException, Path, PathType, WalkDirDepth};

/// `Error` represents various possible errors returned within the `iocore` crate
#[derive(Debug, Clone)]
pub enum Error {
    /// `Error::IOError` wraps [`std::io::Error`]
    IOError(std::io::ErrorKind),
    /// `Error::FileSystemException` represents filesystem-related errors
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
    WalkDirInterrupt(String, Path, WalkDirDepth),
    UnexpectedPathType(Path, PathType),
    WalkDirError(String, Path),
    WalkDirInterrupted(String, Path, WalkDirDepth),
    NondirWalkAttempt(Path),
    PathDoesNotExist(Path),
    MalformedFileName(String),
    ThreadGroupError(String),
    ShellCommandError(String),
    ParseError(String),
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
            Error::WalkDirInterrupt(e, path, depth) => {
                write!(f, "WalkDirInterrupt {} ({} depth): {}", path, depth, e)
            },
            Error::UnexpectedPathType(path, ptype) => {
                write!(f, "UnexpectedPathType: {} is not a {}", path, ptype)
            },
            Error::WalkDirInterrupted(e, path, depth) => {
                write!(f, "WalkDirInterrupt {} (depth: {:#?}): {}", path, depth, e)
            },
            Error::WalkDirError(e, path) => write!(f, "WalkDirError {}: {}", e, path),
            Error::NondirWalkAttempt(path) => write!(f, "NondirWalkAttempt: {}", path),
            Error::PathDoesNotExist(path) => write!(f, "PathDoesNotExist: {}", path),
            Error::MalformedFileName(e) => write!(f, "MalformedFileName: {}", e),
            Error::ThreadGroupError(e) => write!(f, "ThreadGroupError: {}", e),
            Error::ShellCommandError(e) => write!(f, "ShellCommandError: {}", e),
            Error::ParseError(e) => write!(f, "ParseError: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.kind())
    }
}
impl From<FileSystemException> for Error {
    fn from(e: FileSystemException) -> Self {
        Error::FileSystemError(format!("{}", e))
    }
}

impl From<(FileSystemError, Path, String)> for Error {
    fn from(t3: (FileSystemError, Path, String)) -> Error {
        let exc: FileSystemException = t3.into();
        exc.into()
    }
}
impl From<(FileSystemError, &Path, String)> for Error {
    fn from(t3: (FileSystemError, &Path, String)) -> Error {
        let (e, p, s) = t3;
        let exc: FileSystemException = (e, p.clone(), s).into();
        exc.into()
    }
}
impl From<(FileSystemError, Path, &str)> for Error {
    fn from(t3: (FileSystemError, Path, &str)) -> Error {
        let (e, p, s) = t3;
        let exc: FileSystemException = (e, p, s.to_string()).into();
        exc.into()
    }
}
impl From<(FileSystemError, &Path, &str)> for Error {
    fn from(t3: (FileSystemError, &Path, &str)) -> Error {
        let (e, p, s) = t3;
        let exc: FileSystemException = (e, p.clone(), s.to_string()).into();
        exc.into()
    }
}

impl From<(FileSystemError, &Path)> for Error {
    fn from(t2: (FileSystemError, &Path)) -> Error {
        let (e, p) = t2;
        let exc: FileSystemException = (e, p.clone()).into();
        exc.into()
    }
}
impl From<(FileSystemError, Path)> for Error {
    fn from(t3: (FileSystemError, Path)) -> Error {
        let (e, p) = t3;
        let exc: FileSystemException = (e, p).into();
        exc.into()
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

pub type Result<T> = std::result::Result<T, Error>;

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for Error {}
