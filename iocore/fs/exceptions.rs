//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\
use crate::io::error::Code;

#[derive(Copy, Clone)]
pub enum FileSystemError {
    PathDoesNotExist,
    PathIsNotSymlink,
    CreateFile,
    CreateSymlink,
    NoAncestorPath,
    OpenFile,
    AppendFile,
    NonWritablePath,
    NonReadablePath,
    NonExecutablePath,
    WriteFile,
    WriteFlush,
    SetMode,
    MoveFile,
    ReadFile,
    DeleteFile,
    DeleteDirectory,
    UnexpectedPathType,
    AbsolutePath,
    CanonicalPath,
    ReadSymlink,
    CreateDirectory,
    UnsafeFileContent,
}
impl FileSystemError {
    pub fn errno(self) -> u8 {
        self.code().to_u8()
    }

    pub fn code_desc(self) -> &'static str {
        self.code().desc()
    }

    pub fn code(self) -> Code {
        match self {
            FileSystemError::UnexpectedPathType => Code::EBADF,
            FileSystemError::PathDoesNotExist => Code::ENOENT,
            FileSystemError::PathIsNotSymlink => Code::EXDEV,
            FileSystemError::CreateFile => Code::EPERM,
            FileSystemError::CreateSymlink => Code::EPERM,
            FileSystemError::OpenFile => Code::EPERM,
            FileSystemError::AppendFile => Code::EPERM,
            FileSystemError::SetMode => Code::EPERM,
            FileSystemError::NonWritablePath => Code::EPERM,
            FileSystemError::NonReadablePath => Code::EPERM,
            FileSystemError::NonExecutablePath => Code::EPERM,
            FileSystemError::WriteFile => Code::EPERM,
            FileSystemError::WriteFlush => Code::EPERM,
            FileSystemError::MoveFile => Code::EPERM,
            FileSystemError::ReadFile => Code::EPERM,
            FileSystemError::DeleteFile => Code::EPERM,
            FileSystemError::DeleteDirectory => Code::EPERM,
            FileSystemError::AbsolutePath => Code::EPERM,
            FileSystemError::CanonicalPath => Code::EPERM,
            FileSystemError::ReadSymlink => Code::EPERM,
            FileSystemError::CreateDirectory => Code::EPERM,
            FileSystemError::UnsafeFileContent => Code::EIEIO,
            FileSystemError::NoAncestorPath => Code::EDIED,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            FileSystemError::UnexpectedPathType => "unexpected-path-type",
            FileSystemError::PathDoesNotExist => "path-does-not-exist",
            FileSystemError::PathIsNotSymlink => "path-is-not-symlink",
            FileSystemError::CreateFile => "create-file",
            FileSystemError::CreateSymlink => "create-symlink",
            FileSystemError::OpenFile => "open-file",
            FileSystemError::AppendFile => "append-file",
            FileSystemError::SetMode => "set-mode",
            FileSystemError::NonWritablePath => "non-writable-path",
            FileSystemError::NonReadablePath => "non-readable-path",
            FileSystemError::NonExecutablePath => "non-executable-path",
            FileSystemError::WriteFile => "write-file",
            FileSystemError::WriteFlush => "write-flush",
            FileSystemError::MoveFile => "move-file",
            FileSystemError::ReadFile => "read-file",
            FileSystemError::DeleteFile => "delete-file",
            FileSystemError::DeleteDirectory => "delete-directory",
            FileSystemError::AbsolutePath => "absolute-path",
            FileSystemError::CanonicalPath => "canonical-path",
            FileSystemError::ReadSymlink => "read-symlink",
            FileSystemError::CreateDirectory => "create-directory",
            FileSystemError::UnsafeFileContent => "unsafe-file-content",
            FileSystemError::NoAncestorPath => "no-ancestor-path",
        }
    }

    pub fn desc(self) -> &'static str {
        match self {
            FileSystemError::UnexpectedPathType => "expecting to be a",
            FileSystemError::PathDoesNotExist => "file does not exist",
            FileSystemError::PathIsNotSymlink => "file ain't no symlink",
            FileSystemError::CreateFile => "creating path",
            FileSystemError::CreateSymlink => "creating symlink",
            FileSystemError::OpenFile => "opening file",
            FileSystemError::AppendFile => "appending bytes to file",
            FileSystemError::SetMode => "setting mode on file",
            FileSystemError::NonWritablePath => "non-writable file",
            FileSystemError::NonReadablePath => "non-readable file",
            FileSystemError::NonExecutablePath => "non-executable file",
            FileSystemError::WriteFile => "writing bytes to path",
            FileSystemError::WriteFlush => "write flushing to file",
            FileSystemError::MoveFile => "moving file",
            FileSystemError::ReadFile => "reading file",
            FileSystemError::DeleteFile => "deleting file",
            FileSystemError::DeleteDirectory => "deleting directory",
            FileSystemError::AbsolutePath => "resolving absolute path",
            FileSystemError::CanonicalPath => "canonicalizing path",
            FileSystemError::ReadSymlink => "obtaining origin of symlink",
            FileSystemError::CreateDirectory => "creating directories",
            FileSystemError::UnsafeFileContent => "unsafe file content",
            FileSystemError::NoAncestorPath => "does not have ancestors",
        }
    }
}
impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} (error:{})", self.desc(), self.errno())
    }
}
impl std::fmt::Debug for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.as_str(), self.errno())
    }
}

#[derive(Debug, Clone)]
pub enum FileSystemException {
    PathDoesNotExist(crate::fs::Path, String),
    PathIsNotSymlink(crate::fs::Path, String),
    CreateFileError(crate::fs::Path, String),
    CreateSymlinkError(crate::fs::Path, String),
    NoAncestorPathError(crate::fs::Path, String),
    OpenFileError(crate::fs::Path, String),
    AppendFileError(crate::fs::Path, String),
    NonWritablePathError(crate::fs::Path, String),
    NonReadablePathError(crate::fs::Path, String),
    NonExecutablePathError(crate::fs::Path, String),
    WriteFileError(crate::fs::Path, String),
    WriteFlushError(crate::fs::Path, String),
    SetModeError(crate::fs::Path, String),
    MoveFileError(crate::fs::Path, String),
    ReadFileError(crate::fs::Path, String),
    DeleteFileError(crate::fs::Path, String),
    DeleteDirectoryError(crate::fs::Path, String),
    UnexpectedPathType(crate::fs::Path, String),
    AbsolutePathError(crate::fs::Path, String),
    CanonicalPathError(crate::fs::Path, String),
    ReadSymlinkError(crate::fs::Path, String),
    CreateDirectoryError(crate::fs::Path, String),
    UnsafeFileContentError(crate::fs::Path, String),
}
impl FileSystemException {
    fn error(&self) -> FileSystemError {
        match self {
            FileSystemException::UnexpectedPathType(_, _) => FileSystemError::UnexpectedPathType,
            FileSystemException::PathDoesNotExist(_, _) => FileSystemError::PathDoesNotExist,
            FileSystemException::PathIsNotSymlink(_, _) => FileSystemError::PathIsNotSymlink,
            FileSystemException::CreateFileError(_, _) => FileSystemError::CreateFile,
            FileSystemException::CreateSymlinkError(_, _) => FileSystemError::CreateSymlink,
            FileSystemException::OpenFileError(_, _) => FileSystemError::OpenFile,
            FileSystemException::AppendFileError(_, _) => FileSystemError::AppendFile,
            FileSystemException::SetModeError(_, _) => FileSystemError::SetMode,
            FileSystemException::NonWritablePathError(_, _) => FileSystemError::NonWritablePath,
            FileSystemException::NonReadablePathError(_, _) => FileSystemError::NonReadablePath,
            FileSystemException::NonExecutablePathError(_, _) => FileSystemError::NonExecutablePath,
            FileSystemException::WriteFileError(_, _) => FileSystemError::WriteFile,
            FileSystemException::WriteFlushError(_, _) => FileSystemError::WriteFlush,
            FileSystemException::MoveFileError(_, _) => FileSystemError::MoveFile,
            FileSystemException::ReadFileError(_, _) => FileSystemError::ReadFile,
            FileSystemException::DeleteFileError(_, _) => FileSystemError::DeleteFile,
            FileSystemException::DeleteDirectoryError(_, _) => FileSystemError::DeleteDirectory,
            FileSystemException::AbsolutePathError(_, _) => FileSystemError::AbsolutePath,
            FileSystemException::CanonicalPathError(_, _) => FileSystemError::CanonicalPath,
            FileSystemException::ReadSymlinkError(_, _) => FileSystemError::ReadSymlink,
            FileSystemException::CreateDirectoryError(_, _) => FileSystemError::CreateDirectory,
            FileSystemException::UnsafeFileContentError(_, _) => FileSystemError::UnsafeFileContent,
            FileSystemException::NoAncestorPathError(_, _) => FileSystemError::NoAncestorPath,
        }
    }
}

impl From<(FileSystemError, crate::Path, String)> for FileSystemException {
    fn from(fp: (FileSystemError, crate::Path, String)) -> FileSystemException {
        let (error, path, e) = fp;
        let em = format!("{} error on {:#?}: {}", error, path, e);
        match error {
            FileSystemError::UnexpectedPathType =>
                FileSystemException::UnexpectedPathType(path, em),
            FileSystemError::PathDoesNotExist => FileSystemException::PathDoesNotExist(path, em),
            FileSystemError::PathIsNotSymlink => FileSystemException::PathIsNotSymlink(path, em),
            FileSystemError::CreateFile => FileSystemException::CreateFileError(path, em),
            FileSystemError::CreateSymlink => FileSystemException::CreateSymlinkError(path, em),
            FileSystemError::OpenFile => FileSystemException::OpenFileError(path, em),
            FileSystemError::AppendFile => FileSystemException::AppendFileError(path, em),
            FileSystemError::SetMode => FileSystemException::SetModeError(path, em),
            FileSystemError::NonWritablePath => FileSystemException::NonWritablePathError(path, em),
            FileSystemError::NonReadablePath => FileSystemException::NonReadablePathError(path, em),
            FileSystemError::NonExecutablePath =>
                FileSystemException::NonExecutablePathError(path, em),
            FileSystemError::WriteFile => FileSystemException::WriteFileError(path, em),
            FileSystemError::WriteFlush => FileSystemException::WriteFlushError(path, em),
            FileSystemError::MoveFile => FileSystemException::MoveFileError(path, em),
            FileSystemError::ReadFile => FileSystemException::ReadFileError(path, em),
            FileSystemError::DeleteFile => FileSystemException::DeleteFileError(path, em),
            FileSystemError::DeleteDirectory => FileSystemException::DeleteDirectoryError(path, em),
            FileSystemError::AbsolutePath => FileSystemException::AbsolutePathError(path, em),
            FileSystemError::CanonicalPath => FileSystemException::CanonicalPathError(path, em),
            FileSystemError::ReadSymlink => FileSystemException::ReadSymlinkError(path, em),
            FileSystemError::CreateDirectory => FileSystemException::CreateDirectoryError(path, em),
            FileSystemError::UnsafeFileContent =>
                FileSystemException::UnsafeFileContentError(path, em),
            FileSystemError::NoAncestorPath => FileSystemException::NoAncestorPathError(path, em),
        }
    }
}

impl From<(FileSystemError, crate::Path)> for FileSystemException {
    fn from(fp: (FileSystemError, crate::Path)) -> FileSystemException {
        let (error, path) = fp;
        let em = format!("{} error on {:#?}", error, path);
        match error {
            FileSystemError::UnexpectedPathType =>
                FileSystemException::UnexpectedPathType(path, em),
            FileSystemError::PathDoesNotExist => FileSystemException::PathDoesNotExist(path, em),
            FileSystemError::PathIsNotSymlink => FileSystemException::PathIsNotSymlink(path, em),
            FileSystemError::CreateFile => FileSystemException::CreateFileError(path, em),
            FileSystemError::CreateSymlink => FileSystemException::CreateSymlinkError(path, em),
            FileSystemError::OpenFile => FileSystemException::OpenFileError(path, em),
            FileSystemError::AppendFile => FileSystemException::AppendFileError(path, em),
            FileSystemError::SetMode => FileSystemException::SetModeError(path, em),
            FileSystemError::NonWritablePath => FileSystemException::NonWritablePathError(path, em),
            FileSystemError::NonReadablePath => FileSystemException::NonReadablePathError(path, em),
            FileSystemError::NonExecutablePath =>
                FileSystemException::NonExecutablePathError(path, em),
            FileSystemError::WriteFile => FileSystemException::WriteFileError(path, em),
            FileSystemError::WriteFlush => FileSystemException::WriteFlushError(path, em),
            FileSystemError::MoveFile => FileSystemException::MoveFileError(path, em),
            FileSystemError::ReadFile => FileSystemException::ReadFileError(path, em),
            FileSystemError::DeleteFile => FileSystemException::DeleteFileError(path, em),
            FileSystemError::DeleteDirectory => FileSystemException::DeleteDirectoryError(path, em),
            FileSystemError::AbsolutePath => FileSystemException::AbsolutePathError(path, em),
            FileSystemError::CanonicalPath => FileSystemException::CanonicalPathError(path, em),
            FileSystemError::ReadSymlink => FileSystemException::ReadSymlinkError(path, em),
            FileSystemError::CreateDirectory => FileSystemException::CreateDirectoryError(path, em),
            FileSystemError::UnsafeFileContent =>
                FileSystemException::UnsafeFileContentError(path, em),
            FileSystemError::NoAncestorPath => FileSystemException::NoAncestorPathError(path, em),
        }
    }
}

impl Into<FileSystemError> for FileSystemException {
    fn into(self) -> FileSystemError {
        self.error()
    }
}

impl std::fmt::Display for FileSystemException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileSystemException::UnexpectedPathType(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::PathDoesNotExist(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::PathIsNotSymlink(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::CreateFileError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::CreateSymlinkError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::OpenFileError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::AppendFileError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::SetModeError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::NonWritablePathError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::NonReadablePathError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::NonExecutablePathError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::WriteFileError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::WriteFlushError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::MoveFileError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::ReadFileError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::DeleteFileError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::DeleteDirectoryError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::AbsolutePathError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::CanonicalPathError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::ReadSymlinkError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::CreateDirectoryError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::UnsafeFileContentError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
            FileSystemException::NoAncestorPathError(path, em) =>
                write!(f, "{:#?}: {} {}", path, self.error(), em),
        }
    }
}

impl std::error::Error for FileSystemException {}

// #[derive(Debug, Clone)]
// pub enum PrintableError {
//     BuiltinIOError(Box<&'e std::io::Error>),
//     String(&'e str),
// }

// impl From<&std::io::Error> for PrintableError {
//     fn from(p: &std::io::Error) -> PrintableError {
//         PrintableError::BuiltinIOError::(Box::new(p))
//     }
// }
// impl std::fmt::Display for PrintableError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             PrintableError::BuiltinIOError(p) => write!(f, "{}", p),
//             PrintableError::String(s) => write!(f, "{}", s),
//         }
//     }
// }
// impl From<&str> for PrintableError {
//     fn from(s: &str) -> PrintableError {
//         PrintableError::String(s)
//     }
// }
