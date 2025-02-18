//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\
pub mod exceptions;
pub mod opts;
pub mod perms;
pub mod size;
pub mod timed;

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};
use std::fmt::Display;
use std::fs::{File, Permissions};
use std::hash::{Hash, Hasher};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::MAIN_SEPARATOR_STR;
use std::process::Stdio;
use std::str::FromStr;
use std::string::ToString;

pub use exceptions::*;
pub use opts::*;
pub use perms::*;
use sanitation::SString;
use serde::de::Visitor;
use serde::{Deserialize, Serialize};
pub use size::*;
pub use timed::*;

use crate::exceptions::Exception;
pub const FILENAME_MAX: usize = if cfg!(target_os = "macos") { 255 } else { 1024 };

#[derive(Clone, Serialize, Deserialize)]
pub struct Path {
    inner: String,
    path_type: Option<PathType>,
}
pub struct PathVisitor;
impl<'de> Visitor<'de> for PathVisitor {
    type Value = Path;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "array of bytes within the Utf8 range")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Path::new(s.to_string()))
    }
}
pub fn cslashend(s: &str) -> String {
    let regex = regex::Regex::new(r"/+$").unwrap();
    regex.replace_all(s, "").to_string()
}
impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut parts = BTreeSet::<String>::new();
        parts.insert(self.try_absolute().to_string());
        parts.insert(self.kind().to_string());
        parts.insert(self.try_canonicalize().to_string());
        Vec::from_iter(parts.into_iter()).join("%").hash(state);
    }
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.inner_string().eq(&other.inner_string())
    }
}
impl Eq for Path {}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner_string().partial_cmp(&other.inner_string())
    }
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner_string().cmp(&other.inner_string())
    }
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    path: Path,
    pub ino: u64,
    pub gid: u32,
    pub uid: u32,
    pub size: u64,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    accessed: Option<DateTimeNode>,
    created: Option<DateTimeNode>,
    modified: Option<DateTimeNode>,
    pub mode: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum PathType {
    File,
    Symlink,
    Setuid,
    Directory,
    None,
}
impl Hash for PathType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        vec![
            module_path!(),
            "PathType",
            self.to_str()[0..1].to_uppercase().as_str(),
            self.to_str()[1..].to_lowercase().as_str(),
        ]
        .join("::")
        .hash(state);
    }
}

fn nodoubles(p: impl Into<String>) -> String {
    use regex::Regex;
    let e = Regex::new(&format!("[{}]+", MAIN_SEPARATOR_STR)).unwrap();
    let p = p.into();
    e.replace_all(&p, MAIN_SEPARATOR_STR).to_string()
}
impl PathType {
    pub fn to_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symlink => "symlink",
            Self::Setuid => "setuid",
            Self::Directory => "directory",
            Self::None => "none",
        }
    }
}
impl Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
pub trait ToPathType: std::fmt::Display {}
impl<T> ToPathType for T where T: Into<PathType> + std::fmt::Display {}

impl Into<String> for PathType {
    fn into(self) -> String {
        self.to_str().to_string()
    }
}
impl Into<&'static str> for PathType {
    fn into(self) -> &'static str {
        self.to_str()
    }
}

impl From<&str> for PathType {
    fn from(p: &str) -> PathType {
        match p.to_lowercase().as_str() {
            "file" => Self::File,
            "symlink" => Self::Symlink,
            "setuid" => Self::Setuid,
            "directory" => Self::Directory,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum PathStatus {
    None,
    ReadOnlyDirectory,
    ReadOnlyFile,
    ReadOnlySetuid,
    ReadOnlySymlink,
    WritableDirectory,
    WritableFile,
    WritableSetuid,
    WritableSymlink,
}

impl PathStatus {
    pub fn to_str(self) -> &'static str {
        match self {
            Self::ReadOnlyDirectory => "read-only directory",
            Self::ReadOnlyFile => "read-only file",
            Self::ReadOnlySetuid => "read-only setuid",
            Self::ReadOnlySymlink => "read-only symlink",
            Self::WritableFile => "writable file",
            Self::None => "none",
            Self::WritableDirectory => "writable directory",
            Self::WritableSetuid => "writable setuid",
            Self::WritableSymlink => "writable symlink",
        }
    }
}

impl Into<&'static str> for PathStatus {
    fn into(self) -> &'static str {
        self.to_str()
    }
}

impl Into<String> for PathStatus {
    fn into(self) -> String {
        self.to_str().to_string()
    }
}

impl Display for PathStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum LsNodeType {
    File,
    Symlink,
    Setuid,
    Directory,
    None,
}

impl Into<PathType> for LsNodeType {
    fn into(self) -> PathType {
        match self {
            Self::File => PathType::File,
            Self::Symlink => PathType::Symlink,
            Self::Setuid => PathType::Setuid,
            Self::Directory => PathType::Directory,
            Self::None => PathType::None,
        }
    }
}
impl From<PathType> for LsNodeType {
    fn from(p: PathType) -> Self {
        match p {
            PathType::File => Self::File,
            PathType::Symlink => Self::Symlink,
            PathType::Setuid => Self::Setuid,
            PathType::Directory => Self::Directory,
            PathType::None => Self::None,
        }
    }
}
impl LsNodeType {
    fn into_char(self) -> char {
        match self {
            Self::File => '-',
            Self::Symlink => 'l',
            Self::Setuid => 's',
            Self::Directory => 'd',
            Self::None => '?',
        }
    }
}

impl Into<char> for LsNodeType {
    fn into(self) -> char {
        self.into_char()
    }
}
impl Into<String> for LsNodeType {
    fn into(self) -> String {
        String::from(self.into_char())
    }
}
impl ToString for LsNodeType {
    fn to_string(&self) -> String {
        String::from(self.clone().into_char())
    }
}

pub struct NodeStack {
    stack: VecDeque<Node>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.path())
    }
}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", &self.path())
    }
}

impl Path {
    pub fn new(path: impl Into<String>) -> Path {
        let inner = nodoubles(path);
        let inner = if inner.starts_with("~/") {
            inner.replacen("~/", &crate::TILDE.to_string(), 1)
        } else {
            inner.to_string()
        };
        Path {
            inner: inner,
            path_type: None,
        }
    }

    pub fn safe(path: impl Into<String>) -> Result<Path, Exception> {
        let inner = nodoubles(path);
        let inner = if inner.starts_with("~/") {
            inner.replacen("~/", &crate::TILDE.to_string(), 1)
        } else {
            inner.to_string()
        };
        if inner.len() > FILENAME_MAX {
            return Err(Exception::FileSystemError(format!(
                "{}::Path path too long in {:#?}: {:#?}",
                module_path!(),
                std::env::consts::OS,
                inner
            )));
        }
        Ok(Path {
            inner: inner,
            path_type: None,
        })
    }

    pub fn cwd() -> Path {
        Path::new(
            ::std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or(".".to_string()),
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

    pub fn new_with_kind<S: Into<String>>(path: S) -> Path {
        let mut path = Path::new(path);
        path.with_kind().clone()
    }

    pub fn existing<S: Into<String>>(path: S) -> Result<Path, Exception> {
        let path = Path::new_with_kind(path);
        match path.kind() {
            PathType::None => Err((FileSystemError::PathDoesNotExist, path).into()),
            _ => Ok(path.clone()),
        }
    }

    pub fn update_kind(&mut self) -> PathType {
        let kind = self.query_type();
        self.path_type = Some(kind);
        kind
    }

    pub fn update_unset_kind(&mut self) -> Option<PathType> {
        match self.path_type {
            None | Some(PathType::None) => {
                self.path_type = Some(self.query_type());
            },
            _ => {},
        }
        self.path_type
    }

    pub fn with_kind(&mut self) -> &Path {
        self.update_unset_kind();
        self
    }

    pub fn typed(&self) -> Path {
        let mut path = self.clone();
        path.update_unset_kind();
        path
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
        match self.path_type {
            Some(kind) => kind,
            None => self.query_type(),
        }
    }

    pub fn inner_string(&self) -> String {
        nodoubles(&self.inner)
    }

    pub fn as_str(&self) -> &'static str {
        self.inner_string().leak()
    }

    pub fn path(&self) -> &std::path::Path {
        std::path::Path::new(self.inner.as_str())
    }

    pub fn contains(&self, content: &str) -> bool {
        self.inner_string().contains(content)
    }

    pub fn relative_to(&self, t: impl Into<Path>) -> Path {
        let t = t.into();
        if self.to_string() == t.to_string() {
            return t;
        }

        // if t.is_parent_of(self) {
        //     return self.relative_to_parent(t)
        // }
        let mut s = if self.is_dir() {
            format!("{}/", self.try_absolute().to_string().trim_end_matches('/'))
        } else {
            self.parent().unwrap().to_string()
        };
        let (t, n) = if t.is_dir() {
            (
                format!("{}/", t.try_absolute().to_string().trim_end_matches('/')),
                String::new(),
            )
        } else {
            let n = t.name();
            let t = t.try_absolute().parent().unwrap().to_string();
            (format!("{}/", t.trim_end_matches('/')), n)
        };

        if s.starts_with(&t) {
            s = s.replacen(&t, "", 1);
        }

        s = s
            .trim_end_matches('/')
            .split(MAIN_SEPARATOR_STR)
            .map(|_| "..".to_string())
            .collect::<Vec<_>>()
            .join(MAIN_SEPARATOR_STR);
        if n.len() > 0 {
            s = [s, n].join(MAIN_SEPARATOR_STR);
        }
        Path::new(nodoubles(s))
    }

    pub fn relative_to_cwd(&self) -> Path {
        let cwd = Path::cwd();
        if self.to_string() == cwd.to_string() {
            return Path::new("./");
        }

        let s = nodoubles(self.try_absolute().to_string());
        let cwd = cwd.try_canonicalize().to_string();

        let s = nodoubles(if s.starts_with(&cwd) { s.replacen(&cwd, "./", 1) } else { s });
        let s = if s.len() > 2 && s.starts_with("./") {
            cslashend(&s.replacen("./", "", 1))
        } else {
            s
        };
        Path::new(if !s.starts_with("./") { cslashend(&s) } else { s })
    }

    // fn relative_to_parent(&self, certain_parent: &Path) -> Path {
    //     if self.to_string() == certain_parent.to_string() {
    //         return Path::new("./");
    //     }

    //     let s = nodoubles(self.try_absolute().to_string());
    //     let certain_parent = certain_parent.try_canonicalize().to_string();

    //     let s = nodoubles(if s.starts_with(&certain_parent) { s.replacen(&certain_parent, "./", 1) } else { s });
    //     let s = if s.len() > 2 && s.starts_with("./") {
    //         cslashend(&s.replacen("./", "", 1))
    //     } else {
    //         s
    //     };
    //     Path::new(if !s.starts_with("./") { cslashend(&s) } else { s })
    // }

    pub fn file<S: Into<String>>(path: S) -> Result<Path, Exception> {
        let path = Path::new(path);

        if path.canonicalize()?.is_file() {
            Ok(path)
        } else {
            Err(Exception::UnexpectedPathType(path, PathType::File))
        }
    }

    pub fn directory<S: Into<String>>(path: S) -> Result<Path, Exception> {
        let path = Path::new(path);
        if path.canonicalize()?.is_dir() {
            Ok(path)
        } else {
            Err(Exception::UnexpectedPathType(path, PathType::Directory))
        }
    }

    pub fn writable_file(path: impl Into<Path>) -> Result<Path, Exception> {
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

    pub fn readable_file(path: impl Into<Path>) -> Result<Path, Exception> {
        let path = path.into();
        if !path.readable() {
            Err((FileSystemError::NonReadablePath, path).into())
        } else {
            Ok(path)
        }
    }

    pub fn writable_directory(path: impl Into<Path>) -> Result<Path, Exception> {
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

    pub fn writable_symlink(path: impl Into<Path>) -> Result<Path, Exception> {
        let path = path.into();
        match path.status() {
            PathStatus::WritableSymlink => Ok(path),
            PathStatus::None => path.makedirs().map_err(|e| {
                Into::<Exception>::into((FileSystemError::NonWritablePath, path, e.to_string()))
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

    pub fn create(&self) -> Result<File, Exception> {
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

    pub fn write(&self, contents: &[u8]) -> Result<Path, Exception> {
        self.makedirs()?;
        let mut file = self.open(OpenOptions::new().write(true).create(true))?;
        file.set_len(0)?;
        let len = contents.len();
        match file.write_all(contents) {
            Ok(_) => match file.flush() {
                Ok(_) => {},
                Err(e) =>
                    return Err((FileSystemError::WriteFlush, self.clone(), format!("{}", e)).into()),
            },
            Err(e) =>
                return Err(
                    (FileSystemError::WriteFile, self.clone(), format!("{} {}", len, e)).into()
                ),
        };
        Ok(self.clone())
    }

    pub fn append(&self, contents: &[u8]) -> Result<usize, Exception> {
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

    pub fn with_filename(&self, name: impl Into<String>) -> Path {
        let name = name.into();
        self.parent().map(|p| p.join(&name)).unwrap_or(Path::new(&name))
    }

    pub fn rename(
        &self,
        to: &Path,
        create_missing_parents_at_target: bool,
    ) -> Result<Path, Exception> {
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
                return Err(
                    (FileSystemError::MoveFile, self.clone(), format!("to {} {}", to, e)).into()
                ),
        }
    }

    pub fn delete(&self) -> Result<Path, Exception> {
        let node = self.node();
        if node.is_dir {
            for child in self.list()? {
                match child.delete() {
                    Ok(_) => {},
                    Err(_) => {}
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

    pub fn open(&self, open_options: &mut OpenOptions) -> Result<File, Exception> {
        open_options.open(self.path())
    }

    pub fn to_stdio(&self, open_options: &mut OpenOptions) -> Result<Stdio, Exception> {
        Ok(Into::<Stdio>::into(self.open(open_options)?))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    pub fn read_bytes(&self) -> Result<Vec<u8>, Exception> {
        let mut file = self.open(OpenOptions::new().read(true))?;
        let mut bytes = Vec::<u8>::new();
        match file.read_to_end(&mut bytes) {
            Ok(_) => {},
            Err(e) => return Err((FileSystemError::ReadFile, self.clone(), e.to_string()).into()),
        }
        Ok(bytes)
    }

    pub fn read(&self) -> Result<String, Exception> {
        let bytes = self.read_bytes()?;
        SString::new(&bytes)
            .safe()
            .map_err(|e| (FileSystemError::UnsafeFileContent, self.clone(), e.to_string()).into())
    }

    pub fn size(&self) -> Size {
        Size::from(self.node().size)
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

    pub fn set_mode(&mut self, mode: u32) -> Result<Path, Exception> {
        if self.exists() {
            Ok(self.node().set_mode(mode)?.path())
        } else {
            Err(Into::<Exception>::into((
                FileSystemError::SetMode,
                self.clone(),
                format!("setting mode {:o}", mode),
            )))
        }
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

    pub fn read_lines(&self) -> Result<Vec<String>, Exception> {
        Ok(self.read()?.lines().map(|c| c.to_string()).collect::<Vec<String>>())
    }

    pub fn join(&self, path: impl Into<String>) -> Path {
        Path::from(self.path().join(path.into()))
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
            .map(|e| self.to_string().split(e.as_str()).map(String::from).collect::<Vec<String>>())
            .unwrap_or(vec![self.to_string(), String::new()]);
        parts.pop();
        Path::new(parts.join(self.extension().unwrap_or_default().as_str()))
    }

    pub fn with_extension(&self, extension: impl ::std::fmt::Display) -> Path {
        let extension = extension.to_string();
        let extension = extension
            .starts_with(".")
            .then_some(extension.clone())
            .unwrap_or(format!(".{}", &extension));
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

    pub fn expand(&self) -> Result<Path, Exception> {
        Ok(Path::new(if self.to_string().starts_with("~") {
            self.to_string().replacen('~', crate::sys::home()?.as_str(), 1)
        } else {
            self.to_string()
        }))
    }

    pub fn try_expand(&self) -> Path {
        self.expand()
            .unwrap_or(Path::from(self.to_string().replacen('~', &crate::TILDE, 1)))
    }

    pub fn absolute(&self) -> Result<Path, Exception> {
        let name = self.name();
        if self.kind() == PathType::Symlink {
            if let Some(ancestor) = self.parent() {
                Ok(ancestor.absolute().unwrap_or(ancestor).join(name))
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
        self.absolute().unwrap_or(self.clone())
    }

    pub fn canonicalize(&self) -> Result<Path, Exception> {
        let name = self.name();
        match self.expand()?.path().canonicalize() {
            Ok(path) => Ok(Path::from(path)),
            Err(e) =>
                if let Some(ancestor) = self.parent() {
                    Ok(ancestor.absolute().unwrap_or(ancestor).join(name))
                } else {
                    Err((FileSystemError::CanonicalPath, self.clone(), format!("{}", e)).into())
                },
        }
    }

    pub fn try_canonicalize(&self) -> Path {
        match self.canonicalize() {
            Ok(path) => path,
            Err(_) => self.try_absolute(),
        }
        .try_expand()
    }

    pub fn try_read_symlink(&self) -> Path {
        match self.read_symlink() {
            Ok(path) => path,
            Err(_) => self.clone(),
        }
    }

    pub fn read_symlink(&self) -> Result<Path, Exception> {
        if self.kind() != PathType::Symlink {
            return Err((FileSystemError::PathIsNotSymlink, self.clone()).into());
        }
        match std::fs::read_link(self) {
            Ok(path) => Ok(Path::from(path)),
            Err(e) => Err((FileSystemError::ReadSymlink, self.clone(), format!("{}", e)).into()),
        }
    }

    pub fn create_symlink(&self, to: impl Into<Path>) -> Result<Path, Exception> {
        let from = self.canonicalize().map_err(|e| {
            Into::<Exception>::into((FileSystemError::CreateSymlink, self.clone(), e.to_string()))
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
        let parent = self.path().parent().map(|p| p.display().to_string()).unwrap_or(String::new());
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

    pub fn get_or_create_parent_dir(&self) -> Result<Path, Exception> {
        Ok(self.makedirs()?.parent().unwrap())
    }

    pub fn mkdir(&self) -> Result<Path, Exception> {
        let path = self.canonicalize()?;
        if !path.exists() || path.is_dir() {
            match std::fs::create_dir_all(&path) {
                Ok(_) => {
                    path.node().set_mode(0o0700)?;
                    Ok(path.clone())
                },
                Err(e) => Err((FileSystemError::CreateDirectory, path, format!("{}", e)).into()),
            }
        } else {
            Err((
                FileSystemError::CreateDirectory,
                path.clone(),
                format!("({}) exists", path.kind()),
            )
                .into())
        }
    }

    pub fn makedirs(&self) -> Result<Path, Exception> {
        self.parent()
            .ok_or_else(|| {
                Into::<Exception>::into((
                    FileSystemError::CreateDirectory,
                    self.clone(),
                    format!("ain't got no parents"),
                ))
            })?
            .mkdir()?;
        Ok(self.clone())
    }

    pub fn list(&self) -> Result<Vec<Path>, Exception> {
        if !self.try_canonicalize().is_dir() {
            return Err(Exception::ReadDirError(format!("{} is not a folder", &self)));
        }
        Ok(std::fs::read_dir(&self)?
            .filter(|dir_entry| dir_entry.is_ok())
            .map(|dir_entry| dir_entry.unwrap())
            .map(|dir_entry| Path::from(dir_entry))
            .collect())
    }
}
impl NodeStack {
    pub fn new() -> NodeStack {
        NodeStack {
            stack: VecDeque::<Node>::new(),
        }
    }

    pub fn push(&mut self, node: &Node) -> usize {
        self.stack.push_front(node.clone());
        self.len()
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.stack.pop_front()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &nodoubles(&self.inner))
    }
}
impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", &nodoubles(&self.inner))
    }
}

impl Into<String> for Path {
    fn into(self) -> String {
        self.inner.clone()
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
    type Err = Exception;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path::new(s))
    }
}

impl From<String> for Path {
    fn from(p: String) -> Path {
        Path::new(p.as_str())
    }
}

impl From<std::path::PathBuf> for Path {
    fn from(p: std::path::PathBuf) -> Path {
        Path::new(&format!("{}", p.display()))
    }
}
impl From<&std::path::PathBuf> for Path {
    fn from(p: &std::path::PathBuf) -> Path {
        Path::new(&format!("{}", p.display()))
    }
}

impl From<&std::path::Path> for Path {
    fn from(p: &std::path::Path) -> Path {
        Path::new(&format!("{}", p.display()))
    }
}

impl From<std::fs::DirEntry> for Node {
    fn from(p: std::fs::DirEntry) -> Node {
        Node::from(p.path())
    }
}

impl From<std::path::PathBuf> for Node {
    fn from(p: std::path::PathBuf) -> Node {
        Node::new(p)
    }
}

impl From<&std::path::Path> for Node {
    fn from(p: &std::path::Path) -> Node {
        Node::new(p)
    }
}
impl AsRef<std::path::Path> for Node {
    fn as_ref(&self) -> &std::path::Path {
        self.path.path()
    }
}
impl From<Path> for Node {
    fn from(p: Path) -> Node {
        Node::new(p.to_path_buf())
    }
}

impl From<&Path> for Node {
    fn from(p: &Path) -> Node {
        Node::new(p.to_path_buf())
    }
}

impl From<&str> for Node {
    fn from(p: &str) -> Node {
        Node::new(Path::new(p).to_path_buf())
    }
}
impl From<&String> for Node {
    fn from(p: &String) -> Node {
        Node::new(Path::new(p).to_path_buf())
    }
}

impl From<String> for Node {
    fn from(p: String) -> Node {
        Node::new(Path::new(&p).to_path_buf())
    }
}

impl Node {
    pub fn permissions(&self) -> Permissions {
        Permissions::from_mode(self.mode)
    }

    pub fn set_mode(&mut self, mode: u32) -> Result<Node, Exception> {
        let path = self.path();
        match std::fs::metadata(&path) {
            Ok(meta) => {
                let mut p = meta.permissions();
                p.set_mode(mode);
                Ok(Node::from_metadata(path, meta))
            },
            Err(e) => Err(Into::<Exception>::into((FileSystemError::SetMode, path, e.to_string()))),
        }
    }

    pub fn accessed(&self) -> Option<DateTimeNode> {
        self.accessed.clone()
    }

    pub fn created(&self) -> Option<DateTimeNode> {
        self.created.clone()
    }

    pub fn modified(&self) -> Option<DateTimeNode> {
        self.modified.clone()
    }

    pub fn path_type(&self) -> PathType {
        if self.is_file {
            PathType::File
        } else if self.is_dir {
            PathType::Directory
        } else if self.is_symlink {
            PathType::Symlink
        } else {
            PathType::None
        }
    }

    pub fn path_status(&self) -> PathStatus {
        let permissions = self.permissions();
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

    pub fn lst(&self) -> LsNodeType {
        self.path_type().into()
    }

    pub fn path(&self) -> Path {
        self.path.clone()
    }

    pub fn filename(&self) -> String {
        self.path().name()
    }

    pub fn is_writable_file(&self) -> bool {
        match self.path_status() {
            PathStatus::WritableFile | PathStatus::None => true,
            _ => false,
        }
    }

    pub fn is_writable_directory(&self) -> bool {
        match self.path_status() {
            PathStatus::WritableFile | PathStatus::None => true,
            _ => false,
        }
    }

    pub fn is_writable_symlink(&self) -> bool {
        match self.path_status() {
            PathStatus::WritableFile | PathStatus::None => true,
            _ => false,
        }
    }

    pub fn exists(&self) -> bool {
        self.path_status() != PathStatus::None
    }

    pub fn from_metadata(path: impl Into<Path>, meta: std::fs::Metadata) -> Node {
        let accessed: Option<DateTimeNode> = match meta.accessed() {
            Ok(s) => Some(s.into()),
            Err(_) => None,
        };
        let modified: Option<DateTimeNode> = match meta.modified() {
            Ok(s) => Some(s.into()),
            Err(_) => None,
        };
        let created: Option<DateTimeNode> = match meta.created() {
            Ok(s) => Some(s.into()),
            Err(_) => None,
        };
        let ft = meta.file_type();
        Node {
            ino: meta.ino(),
            gid: meta.gid(),
            uid: meta.uid(),
            size: meta.size(),
            accessed: accessed,
            created: created,
            modified: modified,
            is_file: ft.is_file(),
            is_dir: ft.is_dir(),
            is_symlink: ft.is_symlink(),
            mode: meta.mode(),
            path: path.into(),
        }
    }

    pub fn new(path: impl Into<Path>) -> Node {
        let path = path.into();
        match std::fs::symlink_metadata(&path) {
            Ok(meta) => Node::from_metadata(path, meta),
            Err(_) => Node {
                ino: u64::MAX,
                gid: u32::MAX,
                uid: u32::MAX,
                accessed: None,
                created: None,
                modified: None,
                is_dir: false,
                is_file: false,
                is_symlink: false,
                mode: u32::MAX,
                path: path.into(),
                size: u64::MAX,
            },
        }
    }
}
