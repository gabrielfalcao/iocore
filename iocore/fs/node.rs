//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\
use std::fmt::Display;
use std::fs::Permissions;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::string::ToString;

use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::fs::errors::FileSystemError;
use crate::fs::ls_node_type::LsNodeType;
use crate::fs::path_status::PathStatus;
use crate::fs::path_type::PathType;
use crate::fs::timed::DateTimeNode;
use crate::fs::Path;

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

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.path())
    }
}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", &self.path().to_string())
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

    pub fn set_mode(&mut self, mode: u32) -> Result<Node, Error> {
        let path = self.path();
        match std::fs::metadata(&path) {
            Ok(meta) => {
                let mut p = meta.permissions();
                p.set_mode(mode);
                Ok(Node::from_metadata(path, meta))
            },
            Err(e) => Err(Into::<Error>::into((
                FileSystemError::SetMode,
                path,
                format!("Node::set_mode():{} {}", line!(), e),
            ))),
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
        Into::<LsNodeType>::into(self.path_type())
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
