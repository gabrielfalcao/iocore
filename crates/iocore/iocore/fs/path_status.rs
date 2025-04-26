use std::fmt::Display;
use std::string::ToString;

use serde::{Deserialize, Serialize};

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
