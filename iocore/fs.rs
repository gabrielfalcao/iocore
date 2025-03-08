//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\
pub mod errors;
pub mod ls_node_type;
pub mod node;
pub mod opts;
pub mod path_status;
pub mod path_type;
pub mod path_utils;
pub mod perms;
pub mod size;
pub mod timed;

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};
use std::fmt::{Debug, Display};
use std::fs::{File, Permissions};
use std::hash::{Hash, Hasher};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::MAIN_SEPARATOR_STR;
use std::process::Stdio;
use std::str::FromStr;
use std::string::ToString;

pub use errors::*;
pub use ls_node_type::*;
pub use node::*;
pub use opts::*;
pub use path_status::*;
pub use path_type::*;
pub use path_utils::*;
pub use perms::*;
use sanitation::SString;
use serde::{Deserialize, Serialize};
pub use size::*;
pub use timed::*;

use crate::errors::Error;

pub const FILENAME_MAX: usize = if cfg!(target_os = "macos") { 255 } else { 1024 };
pub const ROOT_PATH_STR: &'static str = MAIN_SEPARATOR_STR;

#[derive(Clone, Serialize, Deserialize)]
pub struct Path {
    inner: String,
}

impl Path {
    pub fn new(path: impl std::fmt::Display) -> Path {
        match Path::safe(path) {
            Ok(path) => path,
            Err(message) => panic!("{}", message)
        }
    }
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
        Ok(Path { inner: string })
    }

    pub fn raw(inner: impl std::fmt::Display) -> Path {
        let inner = inner.to_string();
        Path { inner }
    }

    pub fn from_path_buf(path_buf: &std::path::PathBuf) -> Path {
        Path::raw(path_buf.display())
    }

    pub fn from_std_path(path: &std::path::Path) -> Path {
        Path::raw(path.display())
    }

    pub fn cwd() -> Path {
        Path::new(
            ::std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| ".".to_string()),
        )
        .try_canonicalize()
    }

    pub fn tildify(&self) -> Path {
        let t = crate::TILDE.to_string();
        let s = self.to_string();
        if s.starts_with(&t) {
            Path::new(s.replacen(&t, "~", 1)) //UGF0aDo6bmV3KHMucmVwbGFjZW4oJnQsICZmb3JtYXQhKCJ+e30iLCBNQUlOX1NFUEFSQVRPUl9TVFIpLCAxKSk=
        } else {
            self.clone()
        }
    }

    pub fn existing(path: impl std::fmt::Display) -> Result<Path, Error> {
        let path = Path::new(path);
        match path.kind() {
            PathType::None => Err((FileSystemError::PathDoesNotExist, path).into()),
            _ => Ok(path.clone()),
        }
    }

    pub fn query_type(&self) -> PathType {
        let node = self.node();
        if node.is_file {
            PathType::File
        } else if node.is_dir {
            PathType::Directory
        } else if node.is_symlink {
            PathType::Symlink
        } else {
            PathType::None
        }
    }

    pub fn kind(&self) -> PathType {
        self.query_type()
    }

    pub fn inner_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn as_str(&self) -> &'static str {
        self.inner_string().leak()
    }

    pub fn path(&self) -> &'static std::path::Path {
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
        // let s = self;
        // dbg!(s, s.exists());
        // dbg!(t, t.exists());
        let canonical_self = self.try_canonicalize();
        let canonical_t = t.try_canonicalize();
        if canonical_self.to_string() == canonical_t.to_string() {
            return Path::raw("./");
        }

        let s = if canonical_self.exists() {
            canonical_self.to_string()
        } else {
            self.to_string()
        };
        let t = if canonical_t.exists() { canonical_t.to_string() } else { t.to_string() };

        // dbg!(s.len() > t.len());
        if s.len() > t.len() {
            if s.starts_with(&t) {
                let new_path = repl_beg(&add_trailing_separator(&t), &s, "");
                // dbg!(new_path);
                return Path::new(new_path);
            }
        }

        // dbg!(t.len() < s.len());
        if t.len() < s.len() {
            if t.starts_with(&s) {
                let new_path = repl_beg(&add_trailing_separator(&s), &t, "");
                // dbg!(new_path);
                return Path::new(new_path);
            }
        }

        // dbg!(s.len() < t.len());
        if s.len() < t.len() {
            // dbg!(&t, &s);
            if t.starts_with(&s) {
                let t_without_s =
                    remove_trailing_slash(&remove_start(&add_trailing_separator(&s), &t));
                let sub_path = path_str_to_relative_subpath(&t_without_s);
                // dbg!(&t_without_s);
                // assert_ne!(&t_without_s, &t);
                // return Path::raw(dbg!(sub_path));
                return Path::raw(sub_path);
            }
        }
        // Path::new(if !s.starts_with("./") { remove_trailing_slash(&s) } else { s })
        let new_path = Path::raw(&t);
        // let without_absolute_part = remove_absolute_path(&new_path);
        // dbg!(s, t, &new_path, &without_absolute_part);
        // dbg!(s, t, &new_path);
        return new_path;
    }

    pub fn relative_to_cwd(&self) -> Path {
        self.relative_to(&Path::cwd())
    }

    // fn relative_to_parent(&self, certain_parent: &Path) -> Path {
    //     if self.to_string() == certain_parent.to_string() {
    //         return Path::new("./");
    //     }

    //     let s = remove_duplicate_separators(self.try_absolute().to_string());
    //     let certain_parent = certain_parent.try_canonicalize().to_string();

    //     let s = remove_duplicate_separators(if s.starts_with(&certain_parent) { s.replacen(&certain_parent, "./", 1) } else { s });
    //     let s = if s.len() > 2 && s.starts_with("./") {
    //         remove_trailing_slash(&s.replacen("./", "", 1))
    //     } else {
    //         s
    //     };
    //     Path::new(if !s.starts_with("./") { remove_trailing_slash(&s) } else { s })
    // }

    pub fn file(path: impl std::fmt::Display) -> Result<Path, Error> {
        let path = Path::new(path);

        if path.canonicalize()?.is_file() {
            Ok(path)
        } else {
            Err(Error::UnexpectedPathType(path, PathType::File))
        }
    }

    pub fn directory(path: impl std::fmt::Display) -> Result<Path, Error> {
        let path = Path::new(path);
        if path.canonicalize()?.is_dir() {
            Ok(path)
        } else {
            Err(Error::UnexpectedPathType(path, PathType::Directory))
        }
    }

    pub fn writable_file(path: impl Into<Path>) -> Result<Path, Error> {
        let path = path.into();
        match path.status() {
            PathStatus::WritableFile => Ok(path),
            PathStatus::None => path
                .makedirs()
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
                .makedirs()
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
            PathStatus::None => path.makedirs().map_err(|e| {
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

    pub fn status(&self) -> PathStatus {
        match self.node().path_status() {
            PathStatus::None =>
                if self.parent_status() == PathStatus::WritableDirectory {
                    PathStatus::WritableFile
                } else {
                    PathStatus::None
                },
            status => status,
        }
    }

    pub fn create(&self) -> Result<File, Error> {
        let node = self.node();
        if node.is_writable_file() {
            self.makedirs()?;
            match File::create(&self.path()) {
                Ok(file) => Ok(file),
                Err(e) => Err((FileSystemError::CreateFile, self, format!("{}", e)).into()),
            }
        } else {
            Err((FileSystemError::CreateFile, self, format!("path exists ({})", self.kind()))
                .into())
        }
    }

    pub fn write(&self, contents: &[u8]) -> Result<Path, Error> {
        self.makedirs()?;
        let mut file = self.open(OpenOptions::new().write(true).create(true)).map_err(|e| {
            (FileSystemError::OpenFile, self, format!("Path::write():{} {}", line!(), e))
        })?;
        file.set_len(0)?;
        let len = contents.len();
        match file.write_all(contents) {
            Ok(_) => match file.flush() {
                Ok(_) => {},
                Err(e) =>
                    return Err((
                        FileSystemError::WriteFlush,
                        self.clone(),
                        format!("Path::write():{} {}", line!(), e),
                    )
                        .into()),
            },
            Err(e) =>
                return Err((
                    FileSystemError::WriteFile,
                    self.clone(),
                    format!("Path::write():{} {} {}", line!(), len, e),
                )
                    .into()),
        };
        Ok(self.clone())
    }

    pub fn append(&self, contents: &[u8]) -> Result<usize, Error> {
        let node = self.node();
        let mut file = if node.is_writable_file() {
            let mut file =
                self.open(OpenOptions::new().read(true).append(true).write(true).create(true))?;
            let exists = node.exists();

            if exists {
                file.seek(SeekFrom::End(0))?;
            }
            file
        } else {
            return Err((
                FileSystemError::AppendFile,
                self.clone(),
                format!("not writable {}", node.path_type()),
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

    pub fn rename(&self, to: &Path, create_missing_parents_at_target: bool) -> Result<Path, Error> {
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
            to.makedirs()?;
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
        let node = self.node();
        if node.is_dir {
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
        } else if node.exists() {
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

    pub fn size(&self) -> Size {
        Size::from(self.node().size)
    }

    pub fn is_absolute(&self) -> bool {
        self.inner_string().starts_with(ROOT_PATH_STR)
    }

    pub fn is_file(&self) -> bool {
        self.node().is_file
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

    pub fn permissions(&self) -> Permissions {
        self.node().permissions()
    }

    pub fn mode(&self) -> u32 {
        self.permissions().mode()
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
        let meta = std::fs::metadata(&self).map_err(|e| {
            (FileSystemError::SetMode, &path, format!("Path::set_mode():{} {}", line!(), e))
        })?;
        let mut p = meta.permissions();
        p.set_mode(mode);
        Ok(path)
    }

    pub fn is_dir(&self) -> bool {
        self.node().is_dir
    }

    pub fn is_hidden(&self) -> bool {
        self.name().starts_with(".")
    }

    pub fn is_directory(&self) -> bool {
        self.node().is_dir
    }

    pub fn is_symlink(&self) -> bool {
        self.node().is_symlink
    }

    pub fn exists(&self) -> bool {
        self.node().exists()
    }

    pub fn file_size(&self) -> u64 {
        self.node().size
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
            .map(|e| self.name().split(e.as_str()).map(String::from).collect::<Vec<String>>())
            .unwrap_or_else(|| vec![self.name(), String::new()]);
        parts.pop();
        self.parent()
            .unwrap()
            .join(parts.join(self.extension().unwrap_or_default().as_str()))
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
            Ok(Path::raw(expand_home_regex(&self.to_string(), crate::sys::home()?.as_str())))
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
                Ok(ancestor.absolute().unwrap_or_else(|_| ancestor).join(name))
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

    pub fn node(&self) -> Node {
        Node::new(self.to_path_buf())
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

    pub fn parents(&self) -> String {
        match self.path().parent() {
            Some(parent) => Path::from(parent).to_string(),
            None => String::new(),
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
        Ok(self.makedirs()?.parent().unwrap())
    }

    pub fn mkdir(&self) -> Result<Path, Error> {
        let mut path = self.clone();
        if !path.exists() || path.is_dir() {
            match std::fs::create_dir_all(&path) {
                Ok(_) => {
                    path.set_mode(0o0700)?;
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
            path.set_mode(0o0700).map_err(|e| {
                (FileSystemError::SetMode, &path, format!("Path::mkdir():{} {}", line!(), e))
            })?;
        }
        Ok(path)
    }

    pub fn makedirs(&self) -> Result<Path, Error> {
        self.parent()
            .ok_or_else(|| {
                Into::<Error>::into((
                    FileSystemError::CreateDirectory,
                    self.clone(),
                    format!("Path::makedirs():{} ain't got no parents", line!()),
                ))
            })?
            .mkdir()?;
        Ok(self.clone())
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
        sort_paths(&mut paths);
        Ok(paths)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.try_canonicalize().inner_string() == other.try_canonicalize().inner_string()
    }
}
impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        partial_cmp_paths_by_length(self, other)
            .partial_cmp(&partial_cmp_paths_by_parts(self, other))
    }
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_paths_by_length(self, other).cmp(&cmp_paths_by_parts(self, other))
    }
}
impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", &self.inner)
    }
}
impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.inner)
    }
}
impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut parts = BTreeSet::<String>::new();
        parts.insert(self.kind().to_string());
        parts.insert(self.try_canonicalize().to_string());
        Vec::from_iter(parts.into_iter()).join("%").hash(state);
    }
}

impl Into<String> for Path {
    fn into(self) -> String {
        self.inner_string()
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

pub(crate) fn partial_cmp_paths_by_parts(a: &Path, b: &Path) -> Option<Ordering> {
    b.split().len().partial_cmp(&a.split().len())
}
pub(crate) fn partial_cmp_paths_by_length(a: &Path, b: &Path) -> Option<Ordering> {
    b.is_dir()
        .partial_cmp(&a.is_dir())
        .partial_cmp(&b.to_string().len().partial_cmp(&a.to_string().len()))
}
pub(crate) fn cmp_paths_by_parts(a: &Path, b: &Path) -> Ordering {
    b.is_dir().cmp(&a.is_dir()).cmp(&a.split().len().cmp(&b.split().len()))
}
pub(crate) fn cmp_paths_by_length(a: &Path, b: &Path) -> Ordering {
    b.is_dir().cmp(&a.is_dir()).cmp(&a.to_string().len().cmp(&b.to_string().len()))
}
pub fn sort_paths(paths: &mut Vec<Path>) {
    paths.sort_by(|a, b| cmp_paths_by_length(&a, &b));
    paths.sort_by(|a, b| cmp_paths_by_parts(&a, &b));
}
