use serde::{Deserialize, Serialize};

use crate::fs::path_type::PathType;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsPathType {
    File,
    Symlink,
    Setuid,
    Directory,
    None,
}

impl Into<PathType> for LsPathType {
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
impl From<PathType> for LsPathType {
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
impl LsPathType {
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

impl Into<char> for LsPathType {
    fn into(self) -> char {
        self.into_char()
    }
}
impl std::fmt::Display for LsPathType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.into_char())
    }
}
#[cfg(test)]
mod tests {
    use crate::{LsPathType, PathType};

    #[test]
    fn test_ls_path_type_from_path_type() {
        assert_eq!(LsPathType::from(PathType::File), LsPathType::File);
        assert_eq!(LsPathType::from(PathType::Symlink), LsPathType::Symlink);
        assert_eq!(LsPathType::from(PathType::Setuid), LsPathType::Setuid);
        assert_eq!(LsPathType::from(PathType::Directory), LsPathType::Directory);
        assert_eq!(LsPathType::from(PathType::None), LsPathType::None);
    }
    #[test]
    fn test_ls_path_type_into_char() {
        assert_eq!(LsPathType::File.into_char(), '-');
        assert_eq!(LsPathType::Symlink.into_char(), 'l');
        assert_eq!(LsPathType::Setuid.into_char(), 's');
        assert_eq!(LsPathType::Directory.into_char(), 'd');
        assert_eq!(LsPathType::None.into_char(), '?');
    }
    #[test]
    fn test_ls_path_type_to_string() {
        assert_eq!(LsPathType::File.to_string(), "-");
        assert_eq!(LsPathType::Symlink.to_string(), "l");
        assert_eq!(LsPathType::Setuid.to_string(), "s");
        assert_eq!(LsPathType::Directory.to_string(), "d");
        assert_eq!(LsPathType::None.to_string(), "?");
    }
}
