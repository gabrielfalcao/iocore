use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::string::ToString;

use serde::{Deserialize, Serialize};

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
