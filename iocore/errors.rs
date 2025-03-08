//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\

#[derive(Debug, Clone)]
pub enum Error {
    IOError(std::io::ErrorKind),
    FileSystemError(String),
    MalformedGlobPattern(String),
    HomePathError(String),
    ReadDirError(String),
    SafetyError(String),
    EnvironmentVarError(String),
    IOCoreException(String),
    SubprocessError(String),
    SystemError(String),
    ChannelError(String),
    PathConversionError(String),
    PathDeserializationError(String),
    WalkDirInterrupt(String, crate::Node, usize),
    UnexpectedPathType(crate::Path, crate::PathType),
    WalkDirError(String, crate::Node),
    WalkDirInterrupted(String, crate::Node, usize),
    NondirWalkAttempt(crate::Node),
    PathDoesNotExist(crate::Path),
    MalformedFileName(String),
    ThreadGroupError(String),
    ShellCommandError(String),
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
            Error::WalkDirInterrupt(e, node, depth) => {
                write!(f, "WalkDirInterrupt {} ({} depth): {}", node, depth, e)
            },
            Error::UnexpectedPathType(path, ptype) => {
                write!(f, "UnexpectedPathType: {} is not a {}", path, ptype)
            },
            Error::WalkDirInterrupted(e, node, depth) => {
                write!(f, "WalkDirInterrupt {} (depth: {:#?}): {}", node, depth, e)
            },
            Error::WalkDirError(e, node) => write!(f, "WalkDirError {}: {}", e, node),
            Error::NondirWalkAttempt(node) => write!(f, "NondirWalkAttempt: {}", node),
            Error::PathDoesNotExist(path) => write!(f, "PathDoesNotExist: {}", path),
            Error::MalformedFileName(e) => write!(f, "MalformedFileName: {}", e),
            Error::ThreadGroupError(e) => write!(f, "ThreadGroupError: {}", e),
            Error::ShellCommandError(e) => write!(f, "ShellCommandError: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.kind())
    }
}
impl From<crate::fs::FileSystemException> for Error {
    fn from(e: crate::fs::FileSystemException) -> Self {
        Error::FileSystemError(format!("{}", e))
    }
}

impl From<(crate::fs::FileSystemError, crate::Path, String)> for Error {
    fn from(t3: (crate::fs::FileSystemError, crate::Path, String)) -> Error {
        let exc: crate::fs::FileSystemException = t3.into();
        exc.into()
    }
}
impl From<(crate::fs::FileSystemError, &crate::Path, String)> for Error {
    fn from(t3: (crate::fs::FileSystemError, &crate::Path, String)) -> Error {
        let (e, p, s) = t3;
        let exc: crate::fs::FileSystemException = (e, p.clone(), s).into();
        exc.into()
    }
}
impl From<(crate::fs::FileSystemError, crate::Path, &str)> for Error {
    fn from(t3: (crate::fs::FileSystemError, crate::Path, &str)) -> Error {
        let (e, p, s) = t3;
        let exc: crate::fs::FileSystemException = (e, p, s.to_string()).into();
        exc.into()
    }
}
impl From<(crate::fs::FileSystemError, &crate::Path, &str)> for Error {
    fn from(t3: (crate::fs::FileSystemError, &crate::Path, &str)) -> Error {
        let (e, p, s) = t3;
        let exc: crate::fs::FileSystemException = (e, p.clone(), s.to_string()).into();
        exc.into()
    }
}

impl From<(crate::fs::FileSystemError, &crate::Path)> for Error {
    fn from(t2: (crate::fs::FileSystemError, &crate::Path)) -> Error {
        let (e, p) = t2;
        let exc: crate::fs::FileSystemException = (e, p.clone()).into();
        exc.into()
    }
}
impl From<(crate::fs::FileSystemError, crate::Path)> for Error {
    fn from(t3: (crate::fs::FileSystemError, crate::Path)) -> Error {
        let (e, p) = t3;
        let exc: crate::fs::FileSystemException = (e, p).into();
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
