//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\

#[derive(Debug, Clone)]
pub enum Exception {
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
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Exception::IOError(e) => write!(f, "IOError: {}", e),
            Exception::FileSystemError(e) => write!(f, "FileSystemError: {}", e),
            Exception::MalformedGlobPattern(e) => write!(f, "MalformedGlobPattern: {}", e),
            Exception::HomePathError(e) => write!(f, "HomePathError: {}", e),
            Exception::ReadDirError(e) => write!(f, "ReadDirError: {}", e),
            Exception::SafetyError(e) => write!(f, "SafetyError: {}", e),
            Exception::PathDeserializationError(e) => write!(f, "PathDeserializationError: {}", e),
            Exception::IOCoreException(e) => write!(f, "IOCoreException: {}", e),
            Exception::SubprocessError(e) => write!(f, "SubprocessError: {}", e),
            Exception::SystemError(e) => write!(f, "SystemError: {}", e),
            Exception::ChannelError(e) => write!(f, "ChannelError: {}", e),
            Exception::PathConversionError(e) => write!(f, "PathConversionError: {}", e),
            Exception::EnvironmentVarError(s) => write!(f, "EnvironmentVarError: {}", s),
            Exception::WalkDirInterrupt(e, node, depth) => {
                write!(f, "WalkDirInterrupt {} ({} depth): {}", node, depth, e)
            },
            Exception::UnexpectedPathType(path, ptype) => {
                write!(f, "UnexpectedPathType: {} is not a {}", path, ptype)
            },
            Exception::WalkDirInterrupted(e, node, depth) => {
                write!(f, "WalkDirInterrupt {} (depth: {:#?}): {}", node, depth, e)
            },
            Exception::WalkDirError(e, node) => write!(f, "WalkDirError {}: {}", e, node),
            Exception::NondirWalkAttempt(node) => write!(f, "NondirWalkAttempt: {}", node),
            Exception::PathDoesNotExist(path) => write!(f, "PathDoesNotExist: {}", path),
            Exception::MalformedFileName(e) => write!(f, "MalformedFileName: {}", e),
            Exception::ThreadGroupError(e) => write!(f, "ThreadGroupError: {}", e),
        }
    }
}

impl std::error::Error for Exception {}

impl From<std::io::Error> for Exception {
    fn from(e: std::io::Error) -> Self {
        Exception::IOError(e.kind())
    }
}
impl From<crate::fs::FileSystemException> for Exception {
    fn from(e: crate::fs::FileSystemException) -> Self {
        Exception::FileSystemError(format!("{}", e))
    }
}

impl From<(crate::fs::FileSystemError, crate::Path, String)> for Exception {
    fn from(t3: (crate::fs::FileSystemError, crate::Path, String)) -> Exception {
        let exc: crate::fs::FileSystemException = t3.into();
        exc.into()
    }
}
impl From<(crate::fs::FileSystemError, &crate::Path, String)> for Exception {
    fn from(t3: (crate::fs::FileSystemError, &crate::Path, String)) -> Exception {
        let (e, p, s) = t3;
        let exc: crate::fs::FileSystemException = (e, p.clone(), s).into();
        exc.into()
    }
}
impl From<(crate::fs::FileSystemError, crate::Path, &str)> for Exception {
    fn from(t3: (crate::fs::FileSystemError, crate::Path, &str)) -> Exception {
        let (e, p, s) = t3;
        let exc: crate::fs::FileSystemException = (e, p, s.to_string()).into();
        exc.into()
    }
}
impl From<(crate::fs::FileSystemError, &crate::Path, &str)> for Exception {
    fn from(t3: (crate::fs::FileSystemError, &crate::Path, &str)) -> Exception {
        let (e, p, s) = t3;
        let exc: crate::fs::FileSystemException = (e, p.clone(), s.to_string()).into();
        exc.into()
    }
}

impl From<(crate::fs::FileSystemError, &crate::Path)> for Exception {
    fn from(t2: (crate::fs::FileSystemError, &crate::Path)) -> Exception {
        let (e, p) = t2;
        let exc: crate::fs::FileSystemException = (e, p.clone()).into();
        exc.into()
    }
}
impl From<(crate::fs::FileSystemError, crate::Path)> for Exception {
    fn from(t3: (crate::fs::FileSystemError, crate::Path)) -> Exception {
        let (e, p) = t3;
        let exc: crate::fs::FileSystemException = (e, p).into();
        exc.into()
    }
}
impl From<thread_group::Error> for Exception {
    fn from(e: thread_group::Error) -> Self {
        Exception::ThreadGroupError(e.to_string())
    }
}
impl From<sanitation::Error<'_>> for Exception {
    fn from(e: sanitation::Error<'_>) -> Self {
        Exception::SafetyError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Exception>;
