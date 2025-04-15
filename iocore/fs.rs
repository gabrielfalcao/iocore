pub(crate) mod errors;
pub(crate) mod ls_path_type;
pub(crate) mod opts;
pub(crate) mod path_cmp;
pub(crate) mod path_datetime;
pub(crate) mod path_status;
pub(crate) mod path_timestamps;
pub(crate) mod path_type;
pub(crate) mod path_utils;
pub(crate) mod perms;
pub(crate) mod size;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{PathBuf, MAIN_SEPARATOR_STR};
use std::process::Stdio;
use std::str::FromStr;
use std::string::ToString;
use std::sync::RwLock;

use opts::OpenOptions;
use path_cmp::{
    cmp_paths_by_parts, partial_cmp_paths_by_parts, path_ord_split_clamp, path_ord_split_max,
    path_ord_split_min,
};
use path_datetime::PathDateTime;
use path_utils::{
    add_trailing_separator, expand_home_regex, path_str_to_relative_subpath,
    remove_duplicate_separators, remove_start, remove_trailing_slash, repl_beg,
};
use perms::PathPermissions;
use sanitation::SString;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use size::Size;

use crate::errors::Error;
use crate::{FileSystemError, PathStatus, PathTimestamps, PathType};

pub const FILENAME_MAX: usize = if cfg!(target_os = "macos") { 255 } else { 1024 };
pub const ROOT_PATH_STR: &'static str = MAIN_SEPARATOR_STR;

/// `Path` is a data structure representing a path in unix filesystems
/// that has practical methods for otherwise boring tasks, for
/// instance, [`write`] writes bytes to a file, flushes bytes and
/// syncs OS-internal data to the file-system, and if necessary,
/// creates parents directories.
///
/// Example:
///
/// ```
/// use iocore::Path;
/// assert_eq!(
///     Path::cwd().relative_to(&Path::new("tests/doctest-path")).to_string(),
///     "tests/doctest-path"
/// );
/// ```
pub struct Path {
    lock: RwLock<String>,
}
impl Path {
    /// `new` creates a new [`Path`] expanding `~` to the current unix user HOME.
    pub fn new(path: impl std::fmt::Display) -> Path {
        match Path::safe(path) {
            Ok(path) => path,
            Err(message) => panic!("{}", message),
        }
    }

    /// `safe` creates a new [`Path`] expanding `~` to the current unix user HOME
    ///
    /// > NOTE: the current user `HOME` is obtained once at the
    /// > library initialization and stored in the heap.
    pub fn safe(path: impl std::fmt::Display) -> Result<Path, Error> {
        let path = path.to_string();
        let string = remove_duplicate_separators(path);
        let string = if string.starts_with("~/") {
            string.replacen("~/", &crate::TILDE.to_string(), 1)
        } else {
            string.to_string()
        };
        if string.len() > FILENAME_MAX {
            return Err(Error::FileSystemError(format!(
                "{}::Path path too long in {:#?}: {:#?}",
                module_path!(),
                std::env::consts::OS,
                string
            )));
        }
        Ok(Path::raw(string))
    }

    /// `canonical` is a shortcut to [`new`] followed by [`try_canonicalize`]
    ///
    /// Example
    /// ```
    /// use iocore::Path;
    /// assert_eq!(Path::canonical("~"), Path::new("~").try_canonicalize());
    /// ```
    pub fn canonical(path: impl std::fmt::Display) -> Path {
        Path::new(path).try_canonicalize()
    }

    /// `raw` instantiates a [`Path`] with the given string making no validations nor extensions, unlike [`new`] and [`safe`]
    pub fn raw(lock: impl std::fmt::Display) -> Path {
        let string = lock.to_string();
        Path {
            lock: RwLock::new(string),
        }
    }

    /// `from_path_buf` returns a [`Path`] from a [`std::path::PathBuf`]
    pub fn from_path_buf(path_buf: &PathBuf) -> Path {
        Path::raw(path_buf.display())
    }

    /// `from_std_path` returns a [`Path`] from a [`std::path::Path`] reference
    pub fn from_std_path(path: &std::path::Path) -> Path {
        Path::raw(path.display())
    }

    /// `cwd` returns a [`Path`] from the working directory of the current unix process.
    pub fn cwd() -> Path {
        Path::new(
            ::std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| ".".to_string()),
        )
        .try_canonicalize()
    }

    /// `tmp` returns a [`Path`] to a temporary directory via `mktemp`
    pub fn tmp() -> Path {
        Path::raw(crate::sh::shell_command_stdout("mktemp -qd", ".").unwrap().trim())
    }

    /// `tmp` returns a [`Path`] to a temporary file via `mktemp`
    pub fn tmp_file() -> Path {
        Path::raw(crate::sh::shell_command_stdout("mktemp -q", ".").unwrap().trim())
    }

    /// `tildify` returns a new [`Path`] where the current unix user HOME is replaced with "~/"
    pub fn tildify(&self) -> Path {
        let t = crate::TILDE.to_string();
        let s = self.to_string();
        if s.starts_with(&t) {
            Path::raw(s.replacen(&t, &format!("~{}", MAIN_SEPARATOR_STR), 1))
        } else {
            self.clone()
        }
    }

    /// `existing` returns a [`Path`] only if the given string points to a valid existing location in the filesystem
    pub fn existing(path: impl std::fmt::Display) -> Result<Path, Error> {
        let path = Path::new(path);
        match path.kind() {
            PathType::None => Err((FileSystemError::PathDoesNotExist, path).into()),
            _ => Ok(path.clone()),
        }
    }

    pub fn kind(&self) -> PathType {
        if self.is_file() {
            PathType::File
        } else if self.is_dir() {
            PathType::Directory
        } else if self.is_symlink() {
            PathType::Symlink
        } else {
            PathType::None
        }
    }

    pub fn inner_string(&self) -> String {
        let lock = self.lock.read().unwrap();
        (*lock).to_string()
    }

    pub fn path<'a>(&self) -> &'a std::path::Path {
        let mut pathbuf = std::path::PathBuf::new();
        for part in self.split() {
            pathbuf.push(part);
        }
        Box::leak(pathbuf.into_boxed_path())
    }

    pub fn contains(&self, content: &str) -> bool {
        self.inner_string().contains(content)
    }

    pub fn relative_to(&self, t: &Path) -> Path {
        let canonical_self = if self.exists() && t.exists() {
            self.try_canonicalize()
        } else {
            self.clone()
        };
        let canonical_t =
            if self.exists() && t.exists() { t.try_canonicalize() } else { t.clone() };
        if canonical_self.to_string() == canonical_t.to_string() {
            return Path::raw("./");
        }

        let s = if canonical_self.exists() {
            canonical_self.to_string()
        } else {
            self.to_string()
        };
        let t = if canonical_t.exists() { canonical_t.to_string() } else { t.to_string() };

        if s.len() > t.len() {
            if s.starts_with(&t) {
                let new_path = repl_beg(&add_trailing_separator(&t), &s, "");
                return Path::new(new_path);
            }
        }

        if t.len() < s.len() {
            if t.starts_with(&s) {
                let new_path = repl_beg(&add_trailing_separator(&s), &t, "");
                return Path::new(new_path);
            }
        }

        if s.len() < t.len() {
            if t.starts_with(&s) {
                let t_without_s =
                    remove_trailing_slash(&remove_start(&add_trailing_separator(&s), &t));
                let sub_path = path_str_to_relative_subpath(&t_without_s);
                return Path::raw(sub_path);
            }
        }
        let new_path = Path::raw(&t);
        return new_path;
    }

    pub fn relative_to_cwd(&self) -> Path {
        let path_canonicalized = self.try_canonicalize().to_string();
        let cwd_canonicalized = Path::cwd().try_canonicalize().to_string();
        if path_canonicalized.starts_with(&cwd_canonicalized) {
            let path_minus_start = repl_beg(
                &add_trailing_separator(Path::cwd()),
                &self.try_canonicalize().to_string(),
                "",
            );
            Path::raw(path_minus_start)
        } else {
            self.relative_to(&Path::cwd())
        }
    }

    pub fn file(path: impl std::fmt::Display) -> Result<Path, Error> {
        let path = Path::canonical(path);

        if path.try_canonicalize().is_file() {
            Ok(path)
        } else {
            Err(Error::UnexpectedPathType(path.clone(), path.kind()))
        }
    }

    pub fn directory(path: impl std::fmt::Display) -> Result<Path, Error> {
        let path = Path::canonical(path);
        if path.is_directory() {
            Ok(path)
        } else {
            Err(Error::UnexpectedPathType(path.clone(), path.kind()))
        }
    }

    pub fn writable_file(path: impl Into<Path>) -> Result<Path, Error> {
        let path = path.into();
        match path.status() {
            PathStatus::WritableFile => Ok(path),
            PathStatus::None => path
                .mkdir_parents()
                .map_err(|e| (FileSystemError::NonWritablePath, path, e.to_string()).into()),
            status => Err((
                FileSystemError::NonWritablePath,
                path,
                format!("path (exists as {})", status.to_string()),
            )
                .into()),
        }
    }

    pub fn readable_file(path: impl Into<Path>) -> Result<Path, Error> {
        let path = path.into();
        if !path.readable() {
            Err((FileSystemError::NonReadablePath, path).into())
        } else {
            Ok(path)
        }
    }

    pub fn writable_directory(path: impl Into<Path>) -> Result<Path, Error> {
        let path = path.into();
        match path.status() {
            PathStatus::WritableDirectory => Ok(path),
            PathStatus::None => path
                .mkdir_parents()
                .map_err(|e| (FileSystemError::NonWritablePath, path, e.to_string()).into()),
            status => Err((
                FileSystemError::NonWritablePath,
                path,
                format!("path (exists as {})", status.to_string()),
            )
                .into()),
        }
    }

    pub fn writable_symlink(path: impl Into<Path>) -> Result<Path, Error> {
        let path = path.into();
        match path.status() {
            PathStatus::WritableSymlink => Ok(path),
            PathStatus::None => path.mkdir_parents().map_err(|e| {
                Into::<Error>::into((FileSystemError::NonWritablePath, path, e.to_string()))
            }),
            status => Err((
                FileSystemError::NonWritablePath,
                path,
                format!("path (exists as {})", status.to_string()),
            )
                .into()),
        }
    }

    pub fn create(&self) -> Result<File, Error> {
        match self.status() {
            PathStatus::WritableFile | PathStatus::None => {
                if !self.exists() {
                    self.mkdir_parents()?;
                }
                match File::create(&self.path()) {
                    Ok(file) => Ok(file),
                    Err(e) => Err((FileSystemError::CreateFile, self, format!("{}", e)).into()),
                }
            },
            _ => Err((FileSystemError::CreateFile, self, format!("path exists ({})", self.kind()))
                .into()),
        }
    }

    /// `write` writes bytes to file under path, truncates existing
    /// files, creates parent directories if necessary
    ///
    /// Example
    ///
    /// ```
    /// use iocore::Path;
    /// Path::new("tests/doctest-example").write(b"test").unwrap();
    /// assert_eq!(Path::raw("tests/doctest-example").read().unwrap(), "test");
    /// ```
    pub fn write(&self, contents: &[u8]) -> Result<Path, Error> {
        self.mkdir_parents()?;
        let mut file = self.open(OpenOptions::new().write(true).create(true)).map_err(|e| {
            (FileSystemError::OpenFile, self, format!("Path::write():{} {}", line!(), e))
        })?;
        file.set_len(0)?;
        file.write_all(contents).map_err(|error| {
            Error::FileSystemError(format!("writing bytes to {:#?}: {}", self.to_string(), error))
        })?;
        file.flush().map_err(|error| {
            Error::FileSystemError(format!("flushing bytes to {:#?}: {}", self.to_string(), error))
        })?;
        file.sync_all().map_err(|error| {
            Error::FileSystemError(format!(
                "syncing all OS-internal file content to {:#?}: {}",
                self.to_string(),
                error
            ))
        })?;
        Ok(self.clone())
    }

    pub fn append(&self, contents: &[u8]) -> Result<usize, Error> {
        let mut file = if self.is_writable_file() {
            let mut file =
                self.open(OpenOptions::new().read(true).append(true).write(true).create(true))?;
            if self.exists() {
                // seek to the end of file if exists
                file.seek(SeekFrom::End(0))?;
            }
            file
        } else {
            return Err((
                FileSystemError::AppendFile,
                self.clone(),
                format!("not writable {}", self.path_type()),
            )
                .into());
        };
        let bytes = contents.len();
        match file.write_all(contents) {
            Ok(_) => match file.flush() {
                Ok(_) => {},
                Err(e) =>
                    return Err((FileSystemError::WriteFlush, self.clone(), format!("{}", e)).into()),
            },
            Err(e) =>
                return Err((
                    FileSystemError::WriteFile,
                    self.clone(),
                    format!("{} {}", contents.len(), e),
                )
                    .into()),
        };
        Ok(bytes)
    }

    pub fn with_filename(&self, name: impl std::fmt::Display) -> Path {
        let name = name.to_string();
        self.parent().map(|p| p.join(&name)).unwrap_or_else(|| Path::new(&name))
    }

    pub fn rename(
        &self,
        to: impl std::fmt::Display,
        create_missing_parents_at_target: bool,
    ) -> Result<Path, Error> {
        let to = Path::raw(to.to_string());
        let to = match to.parent() {
            Some(_) => to.clone(),
            None => match self.parent() {
                Some(parent) => parent.join(to.name()),
                None =>
                    return Err((
                        FileSystemError::MoveFile,
                        self.clone(),
                        format!("{} neither files seem to have a parent", to),
                    )
                        .into()),
            },
        };
        if create_missing_parents_at_target {
            to.mkdir_parents()?;
        }
        match std::fs::rename(self.path(), to.path()) {
            Ok(_) => Ok(to),
            Err(e) =>
                return Err(Error::FileSystemError(format!(
                    "{} moving {:#?} to {:#?}",
                    e, self, &to
                ))),
        }
    }

    pub fn delete(&self) -> Result<Path, Error> {
        if self.is_directory() {
            for child in self.list()? {
                match child.delete() {
                    Ok(_) => {},
                    Err(_) => {},
                };
            }
            match std::fs::remove_dir(self.path()) {
                Ok(_) => {},
                Err(e) =>
                    return Err(
                        (FileSystemError::DeleteDirectory, self.clone(), format!("{}", e)).into()
                    ),
            }
        } else if self.exists() {
            match std::fs::remove_file(self.path()) {
                Ok(_) => {},
                Err(e) =>
                    return Err((FileSystemError::DeleteFile, self.clone(), format!("{}", e)).into()),
            }
        }
        Ok(self.clone())
    }

    pub fn open(&self, open_options: &mut OpenOptions) -> Result<File, Error> {
        open_options.open(self.path())
    }

    pub fn to_stdio(&self, open_options: &mut OpenOptions) -> Result<Stdio, Error> {
        Ok(Into::<Stdio>::into(self.open(open_options)?))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    pub fn read_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut file = self.open(OpenOptions::new().read(true))?;
        let mut bytes = Vec::<u8>::new();
        match file.read_to_end(&mut bytes) {
            Ok(_) => {},
            Err(e) => return Err((FileSystemError::ReadFile, self.clone(), e.to_string()).into()),
        }
        Ok(bytes)
    }

    pub fn read(&self) -> Result<String, Error> {
        let bytes = self.read_bytes()?;
        SString::new(&bytes)
            .safe()
            .map_err(|e| (FileSystemError::UnsafeFileContent, self.clone(), e.to_string()).into())
    }

    pub fn size(&self) -> Result<Size, Error> {
        let metadata = self.path_metadata().map_err(|error| {
            Error::FileSystemError(format!(
                "error checking size of {:#?}: {}",
                self.to_string(),
                error
            ))
        })?;
        Ok(Size::from(metadata.len()))
    }

    pub fn is_absolute(&self) -> bool {
        self.inner_string().starts_with(ROOT_PATH_STR)
    }

    pub fn is_writable_file(&self) -> bool {
        match self.status() {
            PathStatus::WritableFile | PathStatus::None => true,
            _ => false,
        }
    }

    pub fn is_writable_directory(&self) -> bool {
        match self.status() {
            PathStatus::WritableDirectory | PathStatus::None => true,
            _ => false,
        }
    }

    pub fn is_writable_symlink(&self) -> bool {
        match self.status() {
            PathStatus::WritableSetuid | PathStatus::None => true,
            _ => false,
        }
    }

    pub fn check_permissions(&self) -> Result<PathPermissions, Error> {
        let metadata = self.path_metadata().map_err(|error| {
            Error::FileSystemError(format!(
                "error checking permissions of {:#?}: {}",
                self.to_string(),
                error
            ))
        })?;
        Ok(PathPermissions::from_u32(metadata.mode())?)
    }

    pub fn permissions(&self) -> PathPermissions {
        self.check_permissions().unwrap()
    }

    pub fn mode(&self) -> u32 {
        self.permissions().into_u32()
    }

    pub fn owner_executable(&self) -> bool {
        self.mode() & 0b001000000 == 1
    }

    pub fn owner_writable(&self) -> bool {
        self.mode() & 0b010000000 == 1
    }

    pub fn owner_readable(&self) -> bool {
        self.mode() & 0b100000000 == 1
    }

    pub fn group_executable(&self) -> bool {
        self.mode() & 0b000001000 == 1
    }

    pub fn group_writable(&self) -> bool {
        self.mode() & 0b000010000 == 1
    }

    pub fn group_readable(&self) -> bool {
        self.mode() & 0b000100000 == 1
    }

    pub fn others_executable(&self) -> bool {
        self.mode() & 0b000000001 == 1
    }

    pub fn others_writable(&self) -> bool {
        self.mode() & 0b000000010 == 1
    }

    pub fn others_readable(&self) -> bool {
        self.mode() & 0b000000100 == 1
    }

    pub fn executable(&self) -> bool {
        self.owner_executable() || self.group_executable()
    }

    pub fn writable(&self) -> bool {
        self.owner_writable() || self.group_writable()
    }

    pub fn readable(&self) -> bool {
        self.owner_readable() || self.group_readable()
    }

    pub fn set_mode(&mut self, mode: u32) -> Result<Path, Error> {
        let path = self.clone();
        let meta = std::fs::metadata(self.path()).map_err(|error| {
            Error::FileSystemError(format!(
                "obtaining metadata of {:#?}: {}",
                self.to_string(),
                error
            ))
        })?;
        let mut permissions = meta.permissions();
        permissions.set_mode(mode);
        std::fs::set_permissions(self.path(), permissions).map_err(|error| {
            Error::FileSystemError(format!(
                "setting permissions {:o} of {:#?}: {}",
                mode,
                self.to_string(),
                error
            ))
        })?;
        Ok(path)
    }

    pub fn timestamps(&self) -> Result<PathTimestamps, Error> {
        let metadata = self.path_metadata().map_err(|error| {
            Error::FileSystemError(format!(
                "error getting timestamps of {:#?}: {}",
                self.to_string(),
                error
            ))
        })?;
        Ok(PathTimestamps::from_path(self, &metadata)?)
    }

    pub fn is_file(&self) -> bool {
        match self.meta() {
            Ok(meta) => meta.is_file(),
            Err(_) => false,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.is_directory()
    }

    pub fn is_hidden(&self) -> bool {
        self.name().starts_with(".")
    }

    pub fn is_directory(&self) -> bool {
        match self.meta() {
            Ok(meta) => meta.is_dir(),
            Err(_) => false,
        }
    }

    pub fn is_symlink(&self) -> bool {
        match self.meta() {
            Ok(meta) => meta.is_symlink(),
            Err(_) => false,
        }
    }

    pub fn exists(&self) -> bool {
        match self.meta() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn file_size(&self) -> Size {
        match self.meta() {
            Ok(meta) => Size::from(meta.size()),
            Err(_) => 0.into(),
        }
    }

    pub fn read_lines(&self) -> Result<Vec<String>, Error> {
        Ok(self.read()?.lines().map(|c| c.to_string()).collect::<Vec<String>>())
    }

    pub fn join(&self, path: impl std::fmt::Display) -> Path {
        let path = remove_duplicate_separators(path.to_string());
        if path.starts_with(MAIN_SEPARATOR_STR) {
            return Path::raw(path);
        }
        let mut self_parts = self.split();
        for part in path.split(MAIN_SEPARATOR_STR) {
            self_parts.push_back(part.to_string());
        }
        let new_path_string = Vec::from(self_parts).join(MAIN_SEPARATOR_STR);
        Path::raw(remove_duplicate_separators(new_path_string))
    }

    pub fn split_extension(&self) -> (String, Option<String>) {
        let name = self.name();
        let parts = name.split('.').map(|a| a.to_string()).collect::<Vec<String>>();
        if parts.len() > 1 {
            (
                parts[..parts.len() - 1].to_vec().join("."),
                Some(parts[parts.len() - 1].to_string()),
            )
        } else {
            (parts.join("."), None)
        }
    }

    pub fn join_extension(name: impl std::fmt::Display, extension: Option<String>) -> String {
        match extension {
            None => name.to_string(),
            Some(extension) => format!("{}.{}", name, extension),
        }
    }

    pub fn extension(&self) -> Option<String> {
        self.path()
            .extension()
            .map(|e| SString::new(e.as_encoded_bytes()).unchecked_safe())
            .map(|s| format!(".{}", s))
    }

    pub fn without_extension(&self) -> Path {
        let mut parts = self
            .extension()
            .map(|e| self.name().split(&e.to_string()).map(String::from).collect::<Vec<String>>())
            .unwrap_or_else(|| vec![self.name(), String::new()]);
        parts.pop();
        self.parent().unwrap().join(parts.join(&self.extension().unwrap_or_default()))
    }

    pub fn with_extension(&self, extension: impl ::std::fmt::Display) -> Path {
        let extension = extension.to_string();
        let extension = extension
            .starts_with(".")
            .then_some(extension.clone())
            .unwrap_or_else(|| format!(".{}", &extension));
        Path::new(format!("{}{}", self.without_extension(), extension))
    }

    pub fn hidden(&self) -> Path {
        let name = self.name();
        if name.starts_with(".") {
            self.clone()
        } else {
            self.with_filename(format!(".{}", name))
        }
    }

    pub fn expand(&self) -> Result<Path, Error> {
        if self.to_string().starts_with("~") {
            Ok(Path::raw(expand_home_regex(&self.to_string(), &crate::TILDE.to_string())))
        } else {
            Ok(self.clone())
        }
    }

    pub fn try_expand(&self) -> Path {
        self.expand()
            .unwrap_or_else(|_| Path::raw(expand_home_regex(&self.to_string(), &crate::TILDE)))
    }

    pub fn absolute(&self) -> Result<Path, Error> {
        let name = self.name();
        if self.kind() == PathType::Symlink {
            if let Some(ancestor) = self.parent() {
                Ok(ancestor.canonicalize().unwrap_or_else(|_| ancestor).join(name))
            } else {
                Err((
                    FileSystemError::AbsolutePath,
                    self.clone(),
                    "does not have an ancestor".to_string(),
                )
                    .into())
            }
        } else {
            match self.path().canonicalize() {
                Ok(path) => Ok(Path::from(path)),
                Err(e) =>
                    Err((FileSystemError::AbsolutePath, self.clone(), format!("{}", e)).into()),
            }
        }
    }

    pub fn try_absolute(&self) -> Path {
        self.absolute().unwrap_or_else(|_| self.clone())
    }

    /// `canonicalize` returns a canonical path, resolving symlinks.
    pub fn canonicalize(&self) -> Result<Path, Error> {
        let name = self.name();
        match self.expand()?.path().canonicalize() {
            Ok(path) => Ok(Path::from(path)),
            Err(e) =>
                if let Some(ancestor) = self.parent() {
                    Ok(ancestor.absolute().unwrap_or_else(|_| ancestor).join(name))
                } else {
                    Err((FileSystemError::CanonicalPath, self.clone(), format!("{}", e)).into())
                },
        }
    }

    /// `try_canonicalize` returns a canonical path, resolving
    /// symlinks, returns the caller path in case of error.
    pub fn try_canonicalize(&self) -> Path {
        match self.canonicalize() {
            Ok(path) => path,
            Err(_) => self.clone(),
        }
    }

    pub fn try_read_symlink(&self) -> Path {
        match self.read_symlink() {
            Ok(path) => path,
            Err(_) => self.clone(),
        }
    }

    pub fn read_symlink(&self) -> Result<Path, Error> {
        if self.kind() != PathType::Symlink {
            return Err((FileSystemError::PathIsNotSymlink, self.clone()).into());
        }
        match std::fs::read_link(self) {
            Ok(path) => Ok(Path::from(path)),
            Err(e) => Err((FileSystemError::ReadSymlink, self.clone(), format!("{}", e)).into()),
        }
    }

    pub fn create_symlink(&self, to: impl Into<Path>) -> Result<Path, Error> {
        let from = self.canonicalize().map_err(|e| {
            Into::<Error>::into((FileSystemError::CreateSymlink, self.clone(), e.to_string()))
        })?;
        let to = to.into();
        let to: Path = match to.status() {
            PathStatus::WritableSymlink | PathStatus::WritableFile | PathStatus::None => to.into(),
            status =>
                return Err((
                    FileSystemError::CreateSymlink,
                    self.clone(),
                    format!("to {} path exists as a {}", to, status),
                )
                    .into()),
        };

        match ::std::os::unix::fs::symlink(from, &to) {
            Ok(_) => Ok(to),
            Err(e) =>
                Err((FileSystemError::CreateSymlink, self.clone(), format!("to {} {}", to, e))
                    .into()),
        }
    }

    pub fn name(&self) -> String {
        match self.path().file_name() {
            Some(ext) => SString::new(ext.as_encoded_bytes()).unchecked_safe(),
            None => String::new(),
        }
    }

    pub fn parent(&self) -> Option<Path> {
        let parent = self
            .path()
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| String::new());
        let path = Path::from(if parent.trim().is_empty() {
            format!(".{}", MAIN_SEPARATOR_STR)
        } else {
            parent
        });
        Some(path)
    }

    pub fn split(&self) -> VecDeque<String> {
        let string = self.to_string();
        let mut parts = Vec::<String>::new();
        for part in if string.starts_with(MAIN_SEPARATOR_STR) {
            parts.push(format!("/"));
            string
                .split(MAIN_SEPARATOR_STR)
                .filter(|part| part.len() > 0)
                .map(String::from)
                .collect::<Vec<String>>()
                .to_vec()
        } else {
            string
                .split(MAIN_SEPARATOR_STR)
                .filter(|part| part.len() > 0)
                .map(String::from)
                .collect::<Vec<String>>()
        } {
            parts.push(part);
        }
        VecDeque::from(parts)
    }

    pub fn is_parent_of(&self, other: impl Into<Path>) -> bool {
        let mut this = self.try_canonicalize().split();
        let mut other = Into::<Path>::into(other).try_canonicalize().split();
        if this.len() >= other.len() {
            return false;
        } else {
            while this.len() > 0 {
                let op = other.pop_front().unwrap();
                let tp = this.pop_front().unwrap();
                if op != tp {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn parent_status(&self) -> PathStatus {
        match self.parent() {
            Some(parent) => parent.status(),
            None => PathStatus::None,
        }
    }

    pub fn parent_name(&self) -> String {
        match self.parent() {
            Some(parent) => parent.name(),
            None => String::new(),
        }
    }

    pub fn to_path_buf(&self) -> std::path::PathBuf {
        self.path().to_path_buf()
    }

    pub fn get_or_create_parent_dir(&self) -> Result<Path, Error> {
        Ok(self.mkdir_parents()?.parent().unwrap())
    }

    pub fn mkdir(&self) -> Result<Path, Error> {
        if self.is_directory() {
            return Ok(self.clone());
        }
        let mut path = self.clone();
        if !path.exists() {
            match std::fs::create_dir_all(&path) {
                Ok(_) => {
                    path.set_mode(0o0700).map(|_| ()).unwrap_or_default();
                },
                Err(e) =>
                    return Err((
                        FileSystemError::CreateDirectory,
                        path,
                        format!("Path::mkdir():{} {}", line!(), e),
                    )
                        .into()),
            }
        } else
        // else: folder exists, no problem at all but set permissions to 0700 for cybersecurity's sake
        {
            path.set_mode(0o0700).map(|_| ()).unwrap_or_default();
        }
        Ok(path)
    }

    pub fn list(&self) -> Result<Vec<Path>, Error> {
        if !self.try_canonicalize().is_dir() {
            return Err(Error::ReadDirError(format!("{} is not a folder", &self)));
        }
        let mut paths: Vec<Path> = std::fs::read_dir(&self)?
            .filter(|dir_entry| dir_entry.is_ok())
            .map(|dir_entry| dir_entry.unwrap())
            .map(|dir_entry| Path::from(dir_entry))
            .collect();
        if paths.len() >= 2 {
            paths.sort();
        }
        Ok(paths)
    }

    pub fn set_access_time(&mut self, new_access_time: &PathDateTime) -> Result<Path, Error> {
        let mut timestamps = self.timestamps()?;
        timestamps.set_access_time(new_access_time)?;
        Ok(self.clone())
    }

    pub fn set_modified_time(&mut self, new_modified_time: &PathDateTime) -> Result<Path, Error> {
        let mut timestamps = self.timestamps()?;
        timestamps.set_modified_time(new_modified_time)?;
        Ok(self.clone())
    }

    fn path_metadata(&self) -> Result<std::fs::Metadata, Error> {
        Ok(std::fs::metadata(self.path()).map_err(|error| {
            let io_error = Error::IOError(error.kind()).to_string();
            Error::FileSystemError(format!(
                "error obtaining metadata of of {:#?}: {}",
                self.to_string(),
                io_error
            ))
        })?)
    }

    pub fn accessed(&self) -> Option<PathDateTime> {
        self.timestamps().map(|t| t.accessed).ok()
    }

    pub fn created(&self) -> Option<PathDateTime> {
        self.timestamps().map(|t| t.created).ok()
    }

    pub fn modified(&self) -> Option<PathDateTime> {
        self.timestamps().map(|t| t.modified).ok()
    }

    pub fn fs_permissions(&self) -> std::fs::Permissions {
        std::fs::Permissions::from_mode(self.mode())
    }

    pub fn status(&self) -> PathStatus {
        let permissions = self.fs_permissions();
        let readonly = permissions.readonly();

        match self.path_type() {
            PathType::Directory =>
                if readonly {
                    PathStatus::ReadOnlyDirectory
                } else {
                    PathStatus::WritableDirectory
                },
            PathType::File =>
                if readonly {
                    PathStatus::ReadOnlyFile
                } else {
                    PathStatus::WritableFile
                },
            PathType::Symlink =>
                if readonly {
                    PathStatus::ReadOnlySymlink
                } else {
                    PathStatus::WritableSymlink
                },
            PathType::Setuid =>
                if readonly {
                    PathStatus::ReadOnlySetuid
                } else {
                    PathStatus::WritableSetuid
                },
            PathType::None => PathStatus::None,
        }
    }

    pub fn meta(&self) -> Result<std::fs::Metadata, Error> {
        let metadata = self.path_metadata().map_err(|error| {
            Error::FileSystemError(format!(
                "obtaining std::fs::Metadata of {:#?}: {}",
                self.to_string(),
                error
            ))
        })?;
        Ok(metadata)
    }

    pub fn path_type(&self) -> PathType {
        match self.meta() {
            Err(_) => PathType::None,
            Ok(meta) =>
                if meta.is_file() {
                    PathType::File
                } else if meta.is_dir() {
                    PathType::Directory
                } else if meta.is_symlink() {
                    PathType::Symlink
                } else {
                    PathType::None
                },
        }
    }

    pub fn uid(&self) -> u32 {
        self.meta().expect("Path::uid").uid()
    }

    pub fn gid(&self) -> u32 {
        self.meta().expect("Path::gid").gid()
    }

    pub fn mkdir_parents(&self) -> Result<Path, Error> {
        match self.parent() {
            Some(parent) => {
                parent.mkdir()?;
            },
            None => {},
        }
        Ok(self.clone())
    }

    pub fn mkdir_unchecked(&self) -> Path {
        self.mkdir().map(|_| ()).unwrap_or_default();
        self.clone()
    }

    pub fn mkdir_parents_unchecked(&self) -> Path {
        self.mkdir_parents().map(|_| ()).unwrap_or_default();
        self.clone()
    }

    pub fn write_unchecked(&self, contents: &[u8]) -> Path {
        self.write(contents).map(|_| ()).unwrap_or_default();
        self.clone()
    }

    pub fn delete_unchecked(&self) -> Path {
        self.delete().map(|_| ()).unwrap_or_default();
        self.clone()
    }

    /// Matcheses the path with given regex pattern
    ///
    /// Example
    /// ```
    /// let path = Path::raw("/Users/stevejobs/Library/Preferences");
    /// let result = path.matches_regex(r"^/Users/(?<user>[^/]+)/");
    /// assert_eq!(result, Ok(true))
    /// ```
    pub fn matches_regex(&self, pattern: &str) -> crate::Result<bool> {
        let re = regex::Regex::new(pattern)?;
        Ok(re.is_match(&self.to_string()))
    }

    /// Searches the path with given regex pattern
    ///
    /// Example
    /// ```
    /// let path = Path::raw("/Users/stevejobs/Library/Preferences");
    /// let result = path.search_regex(r"^/Users/(?<user>[^/]+)/");
    /// assert!(result.is_ok());
    /// let (full, parts) = result.unwrap();
    /// assert_eq!(full, "/Users/stevejobs/");
    /// assert_eq!(parts, vec!["stevejobs"],);
    /// ```
    pub fn search_regex(&self, pattern: &str) -> crate::Result<(String, Vec<String>)> {
        let re = regex::Regex::new(pattern)?;
        let haystack = self.to_string();
        match re.captures(&haystack) {
            Some(caps) => {
                let mut parts = Vec::<String>::new();
                let full = caps.get(0).unwrap().as_str().to_string();
                for j in 1..(caps.len()) {
                    parts.push(caps.get(j).unwrap().as_str().to_string());
                }
                Ok((full, parts))
            },
            None => Err(Error::PatternMismatch(format!(
                "regex '{}' does not match path '{}'",
                pattern, &haystack
            ))),
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.exists() == other.exists()
            && self.is_directory() == other.is_directory()
            && self.is_file() == other.is_file()
            && self.try_canonicalize().to_string() == other.try_canonicalize().to_string()
    }

    fn ne(&self, other: &Self) -> bool {
        self.exists() != other.exists()
            && self.is_directory() != other.is_directory()
            && self.is_file() != other.is_file()
            && self.try_canonicalize().to_string() != other.try_canonicalize().to_string()
    }
}
impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        partial_cmp_paths_by_parts(self, other)
    }

    fn lt(&self, other: &Path) -> bool {
        match cmp_paths_by_parts(self, other) {
            Ordering::Less => true,
            _ => false,
        }
    }

    fn le(&self, other: &Path) -> bool {
        match cmp_paths_by_parts(self, other) {
            Ordering::Less => true,
            _ => false,
        }
    }

    fn gt(&self, other: &Path) -> bool {
        match cmp_paths_by_parts(self, other) {
            Ordering::Greater => true,
            _ => false,
        }
    }

    fn ge(&self, other: &Path) -> bool {
        match cmp_paths_by_parts(self, other) {
            Ordering::Greater => true,
            _ => false,
        }
    }
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_paths_by_parts(self, other)
    }

    fn max(self, other: Path) -> Path {
        path_ord_split_max(self, other)
    }

    fn min(self, other: Path) -> Path {
        path_ord_split_min(self, other)
    }

    fn clamp(self, min: Path, max: Path) -> Path {
        path_ord_split_clamp(self, min, max)
    }
}
impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:#?}]{}", core::ptr::from_ref(self), &self.relative_to_cwd().to_string())
    }
}
impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner_string())
    }
}
impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.try_canonicalize().to_string().hash(state)
    }
}

impl Into<String> for Path {
    fn into(self) -> String {
        self.inner_string()
    }
}

impl Clone for Path {
    fn clone(&self) -> Path {
        Path::raw(self.inner_string())
    }
}
impl Into<::std::path::PathBuf> for Path {
    fn into(self) -> ::std::path::PathBuf {
        self.to_path_buf()
    }
}
impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        self.path()
    }
}

impl From<&Path> for Path {
    fn from(path: &Path) -> Path {
        path.clone()
    }
}

impl From<&mut str> for Path {
    fn from(p: &mut str) -> Path {
        Path::new(p)
    }
}

impl From<&mut String> for Path {
    fn from(p: &mut String) -> Path {
        Path::new(p.clone())
    }
}

impl From<Option<Path>> for Path {
    fn from(p: Option<Path>) -> Path {
        match p {
            Some(p) => Path::new(p.to_string()),
            None => Path::new(""),
        }
    }
}
impl From<std::fs::DirEntry> for Path {
    fn from(p: std::fs::DirEntry) -> Path {
        Path::from(p.path())
    }
}
impl From<Cow<'_, str>> for Path {
    fn from(p: Cow<'_, str>) -> Path {
        Path::new(p.to_string().as_str())
    }
}

impl From<&str> for Path {
    fn from(p: &str) -> Path {
        Path::new(p)
    }
}
impl From<&&str> for Path {
    fn from(p: &&str) -> Path {
        Path::new(*p)
    }
}
impl From<&String> for Path {
    fn from(p: &String) -> Path {
        Path::new(p.as_str())
    }
}
impl From<&&String> for Path {
    fn from(p: &&String) -> Path {
        Path::new(p.as_str())
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path::raw(s.to_string()))
    }
}

impl From<String> for Path {
    fn from(p: String) -> Path {
        Path::raw(p)
    }
}

impl From<std::path::PathBuf> for Path {
    fn from(path_buf: std::path::PathBuf) -> Path {
        Path::from_path_buf(&path_buf)
    }
}
impl From<&std::path::PathBuf> for Path {
    fn from(path_buf: &std::path::PathBuf) -> Path {
        Path::from_path_buf(path_buf)
    }
}

impl From<&std::path::Path> for Path {
    fn from(path: &std::path::Path) -> Path {
        Path::from_std_path(path)
    }
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Path, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PathVisitor)
    }
}
struct PathVisitor;

impl<'de> Visitor<'de> for PathVisitor {
    type Value = Path;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(E::custom(format!("cannot deserialize i8: {:#?}", value)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(E::custom(format!("cannot deserialize i8: {:#?}", value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(E::custom(format!("cannot deserialize i8: {:#?}", value)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Path::raw(value))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Path::raw(value.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Path;
    #[test]
    fn test_path_relative_to() {
        let iocore_fs_path = Path::raw(file!());
        let iocore_lib_path = Path::raw(file!()).parent().unwrap();
        assert_eq!(iocore_fs_path.relative_to(&iocore_lib_path).to_string(), "fs.rs");
        assert_eq!(iocore_lib_path.relative_to(&iocore_fs_path).to_string(), "../");
    }
    #[test]
    fn test_path_relative_to_cwd() {
        let iocore_fs_path = Path::raw(file!());
        assert_eq!(iocore_fs_path.relative_to_cwd().to_string(), "iocore/fs.rs");
    }

    #[test]
    fn test_serialize_and_deserialize() {
        let path = Path::raw(file!()).relative_to_cwd();
        let serialized = serde_json::to_string(&path).unwrap();
        assert_eq!(serialized, "\"iocore/fs.rs\"");
        assert_eq!(serde_json::from_str::<Path>(serialized.as_str()).unwrap(), path);
    }

    #[test]
    fn test_matches_regex() {
        let path = Path::raw("/Users/stevejobs/Library/Preferences");
        let result = path.matches_regex(r"^/Users/(?<user>[^/]+)/");
        assert_eq!(result, Ok(true))
    }
    #[test]
    fn test_search_regex() {
        let path = Path::raw("/Users/stevejobs/Library/Preferences");
        let result = path.search_regex(r"^/Users/(?<user>[^/]+)/");
        assert!(result.is_ok());
        let (full, parts) = result.unwrap();
        assert_eq!(full, "/Users/stevejobs/");
        assert_eq!(parts, vec!["stevejobs"],);
    }
}
