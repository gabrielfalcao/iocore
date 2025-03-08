//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\
use std::string::ToString;

use serde::{Deserialize, Serialize};

use crate::fs::path_type::PathType;

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
