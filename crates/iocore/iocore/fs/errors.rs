use crate::io::error::Code;
use crate::Path;

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
    PathDoesNotExist(Path, String),
    PathIsNotSymlink(Path, String),
    CreateFileError(Path, String),
    CreateSymlinkError(Path, String),
    NoAncestorPathError(Path, String),
    OpenFileError(Path, String),
    AppendFileError(Path, String),
    NonWritablePathError(Path, String),
    NonReadablePathError(Path, String),
    NonExecutablePathError(Path, String),
    WriteFileError(Path, String),
    WriteFlushError(Path, String),
    SetModeError(Path, String),
    MoveFileError(Path, String),
    ReadFileError(Path, String),
    DeleteFileError(Path, String),
    DeleteDirectoryError(Path, String),
    UnexpectedPathType(Path, String),
    AbsolutePathError(Path, String),
    CanonicalPathError(Path, String),
    ReadSymlinkError(Path, String),
    CreateDirectoryError(Path, String),
    UnsafeFileContentError(Path, String),
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

impl From<(FileSystemError, Path, String)> for FileSystemException {
    fn from(fp: (FileSystemError, Path, String)) -> FileSystemException {
        let (error, path, e) = fp;
        let error_description = format!("{} error on {:#?}: {}", error, path, e);
        match error {
            FileSystemError::UnexpectedPathType =>
                FileSystemException::UnexpectedPathType(path, error_description),
            FileSystemError::PathDoesNotExist =>
                FileSystemException::PathDoesNotExist(path, error_description),
            FileSystemError::PathIsNotSymlink =>
                FileSystemException::PathIsNotSymlink(path, error_description),
            FileSystemError::CreateFile =>
                FileSystemException::CreateFileError(path, error_description),
            FileSystemError::CreateSymlink =>
                FileSystemException::CreateSymlinkError(path, error_description),
            FileSystemError::OpenFile =>
                FileSystemException::OpenFileError(path, error_description),
            FileSystemError::AppendFile =>
                FileSystemException::AppendFileError(path, error_description),
            FileSystemError::SetMode => FileSystemException::SetModeError(path, error_description),
            FileSystemError::NonWritablePath =>
                FileSystemException::NonWritablePathError(path, error_description),
            FileSystemError::NonReadablePath =>
                FileSystemException::NonReadablePathError(path, error_description),
            FileSystemError::NonExecutablePath =>
                FileSystemException::NonExecutablePathError(path, error_description),
            FileSystemError::WriteFile =>
                FileSystemException::WriteFileError(path, error_description),
            FileSystemError::WriteFlush =>
                FileSystemException::WriteFlushError(path, error_description),
            FileSystemError::MoveFile =>
                FileSystemException::MoveFileError(path, error_description),
            FileSystemError::ReadFile =>
                FileSystemException::ReadFileError(path, error_description),
            FileSystemError::DeleteFile =>
                FileSystemException::DeleteFileError(path, error_description),
            FileSystemError::DeleteDirectory =>
                FileSystemException::DeleteDirectoryError(path, error_description),
            FileSystemError::AbsolutePath =>
                FileSystemException::AbsolutePathError(path, error_description),
            FileSystemError::CanonicalPath =>
                FileSystemException::CanonicalPathError(path, error_description),
            FileSystemError::ReadSymlink =>
                FileSystemException::ReadSymlinkError(path, error_description),
            FileSystemError::CreateDirectory =>
                FileSystemException::CreateDirectoryError(path, error_description),
            FileSystemError::UnsafeFileContent =>
                FileSystemException::UnsafeFileContentError(path, error_description),
            FileSystemError::NoAncestorPath =>
                FileSystemException::NoAncestorPathError(path, error_description),
        }
    }
}

impl From<(FileSystemError, Path)> for FileSystemException {
    fn from(fp: (FileSystemError, Path)) -> FileSystemException {
        let (error, path) = fp;
        let error_description = format!("{} error on {:#?}", error, path);
        match error {
            FileSystemError::UnexpectedPathType =>
                FileSystemException::UnexpectedPathType(path, error_description),
            FileSystemError::PathDoesNotExist =>
                FileSystemException::PathDoesNotExist(path, error_description),
            FileSystemError::PathIsNotSymlink =>
                FileSystemException::PathIsNotSymlink(path, error_description),
            FileSystemError::CreateFile =>
                FileSystemException::CreateFileError(path, error_description),
            FileSystemError::CreateSymlink =>
                FileSystemException::CreateSymlinkError(path, error_description),
            FileSystemError::OpenFile =>
                FileSystemException::OpenFileError(path, error_description),
            FileSystemError::AppendFile =>
                FileSystemException::AppendFileError(path, error_description),
            FileSystemError::SetMode => FileSystemException::SetModeError(path, error_description),
            FileSystemError::NonWritablePath =>
                FileSystemException::NonWritablePathError(path, error_description),
            FileSystemError::NonReadablePath =>
                FileSystemException::NonReadablePathError(path, error_description),
            FileSystemError::NonExecutablePath =>
                FileSystemException::NonExecutablePathError(path, error_description),
            FileSystemError::WriteFile =>
                FileSystemException::WriteFileError(path, error_description),
            FileSystemError::WriteFlush =>
                FileSystemException::WriteFlushError(path, error_description),
            FileSystemError::MoveFile =>
                FileSystemException::MoveFileError(path, error_description),
            FileSystemError::ReadFile =>
                FileSystemException::ReadFileError(path, error_description),
            FileSystemError::DeleteFile =>
                FileSystemException::DeleteFileError(path, error_description),
            FileSystemError::DeleteDirectory =>
                FileSystemException::DeleteDirectoryError(path, error_description),
            FileSystemError::AbsolutePath =>
                FileSystemException::AbsolutePathError(path, error_description),
            FileSystemError::CanonicalPath =>
                FileSystemException::CanonicalPathError(path, error_description),
            FileSystemError::ReadSymlink =>
                FileSystemException::ReadSymlinkError(path, error_description),
            FileSystemError::CreateDirectory =>
                FileSystemException::CreateDirectoryError(path, error_description),
            FileSystemError::UnsafeFileContent =>
                FileSystemException::UnsafeFileContentError(path, error_description),
            FileSystemError::NoAncestorPath =>
                FileSystemException::NoAncestorPathError(path, error_description),
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
            FileSystemException::UnexpectedPathType(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::PathDoesNotExist(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::PathIsNotSymlink(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::CreateFileError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::CreateSymlinkError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::OpenFileError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::AppendFileError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::SetModeError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::NonWritablePathError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::NonReadablePathError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::NonExecutablePathError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::WriteFileError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::WriteFlushError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::MoveFileError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::ReadFileError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::DeleteFileError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::DeleteDirectoryError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::AbsolutePathError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::CanonicalPathError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::ReadSymlinkError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::CreateDirectoryError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::UnsafeFileContentError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
            FileSystemException::NoAncestorPathError(path, error_description) =>
                write!(f, "{:#?}: {} {}", path, self.error(), error_description),
        }
    }
}

impl std::error::Error for FileSystemException {}
