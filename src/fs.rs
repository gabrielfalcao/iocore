use crate::coreio::absolute_path;
use crate::exceptions::Exception;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FSNode {
    pub path: FilePath,
    pub mode: u32,
    pub accessed: Option<DateTime<Utc>>,
    pub modified: Option<DateTime<Utc>>,
    pub created: Option<DateTime<Utc>>,
    pub size: u64,
    pub is_file: bool,
    pub is_dir: bool,
    pub is_symlink: bool,
}

impl FSNode {
    pub fn new(path: PathBuf) -> FSNode {
        match std::fs::metadata(&path) {
            Ok(meta) => {
                let path: FilePath = path.into();
                let accessed: Option<DateTime<Utc>> = match meta.accessed() {
                    Ok(s) => Some(s.into()),
                    Err(_) => None,
                };
                let modified: Option<DateTime<Utc>> = match meta.modified() {
                    Ok(s) => Some(s.into()),
                    Err(_) => None,
                };
                let created: Option<DateTime<Utc>> = match meta.created() {
                    Ok(s) => Some(s.into()),
                    Err(_) => None,
                };
                let ft = meta.file_type();
                FSNode {
                    accessed: accessed,
                    created: created,
                    modified: modified,
                    is_file: ft.is_file(),
                    is_dir: ft.is_dir(),
                    is_symlink: ft.is_symlink(),
                    mode: meta.mode(),
                    path: path,
                    size: meta.len(),
                }
            }
            Err(_) => FSNode {
                accessed: None,
                created: None,
                modified: None,
                is_dir: false,
                is_file: false,
                is_symlink: false,
                mode: 0,
                path: path.into(),
                size: 0,
            },
        }
    }
}

impl From<PathBuf> for FSNode {
    fn from(p: PathBuf) -> FSNode {
        FSNode::new(p)
    }
}

impl From<&Path> for FSNode {
    fn from(p: &Path) -> FSNode {
        FSNode::new(p.to_path_buf())
    }
}

impl From<FilePath> for FSNode {
    fn from(p: FilePath) -> FSNode {
        FSNode::new(p.to_path_buf())
    }
}

impl From<&str> for FSNode {
    fn from(p: &str) -> FSNode {
        FSNode::new(match absolute_path(p) {
            Ok(p) => p,
            Err(_) => Path::new(&p).to_path_buf(),
        })
    }
}

impl From<String> for FSNode {
    fn from(p: String) -> FSNode {
        FSNode::new(p.into())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub path: FilePath,
    pub meta: Option<FSNode>,
}
impl File {
    pub fn new(path: &str) -> Result<File, Exception> {
        let data = FSNode::new(absolute_path(path)?);
        Ok(File {
            path: data.path.clone(),
            meta: Some(data),
        })
    }
}

impl StringPath for File {
    fn relative_to(&self, ancestor: &str) -> String {
        self.path.to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self.path)
    }
}

impl PathRelative for File {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        self.path.to(&format!("{}", ancestor.display())).into()
    }
}

impl From<PathBuf> for File {
    fn from(p: PathBuf) -> File {
        File::new(&format!("{}", p.display()))
            .expect(&format!("could not resolve std::path::PathBuf {:?} to a iocore::fs::File", p))
    }
}

impl From<&str> for File {
    fn from(p: &str) -> File {
        File::new(&match absolute_path(p) {
            Ok(p) => format!("{}", p.display()),
            Err(_) => p.to_string(),
        })
            .expect(&format!("could not resolve &str {} to a iocore::fs::File", p))
    }
}

impl From<String> for File {
    fn from(p: String) -> File {
        File::new(&p).expect(&format!("could not resolve String {} to a iocore::fs::File", p))
    }
}

impl From<&Path> for File {
    fn from(p: &Path) -> File {
        File::new(&format!("{}", p.display())).expect(&format!("could not resolve std::path::Path {:?} to a iocore::fs::File", p))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FilePath(String);

impl FilePath {
    pub fn path(&self) -> &Path {
        Path::new(&self.0)
    }
    pub fn to_path_buf(&self) -> PathBuf {
        self.path().to_path_buf()
    }
}

impl From<PathBuf> for FilePath {
    fn from(p: PathBuf) -> FilePath {
        FilePath(format!("{}", p.display()))
    }
}

impl From<&str> for FilePath {
    fn from(p: &str) -> FilePath {
        FilePath(match absolute_path(p) {
            Ok(p) => format!("{}", p.display()),
            Err(_) => p.to_string(),
        })
    }
}

impl From<String> for FilePath {
    fn from(p: String) -> FilePath {
        FilePath(p)
    }
}

impl From<&Path> for FilePath {
    fn from(p: &Path) -> FilePath {
        FilePath(format!("{}", p.display()))
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FilePath {
    pub fn from(source: &str) -> Self {
        FilePath(source.to_string())
    }
    pub fn to(&self, ancestor: &str) -> String {
        self.0
            .replacen(
                &if ancestor.ends_with("/") {
                    ancestor.to_string()
                } else {
                    format!("{}/", ancestor)
                },
                "",
                1,
            )
            .to_string()
    }
}

pub trait StringPath: Clone + Sized {
    fn relative_to(&self, ancestor: &str) -> String;
    fn tostring(&self) -> String;
}

// impl <T>From<PathBuf> for StringPath<T> {
//     fn from(e: StringPath) -> Self {
//         Exception::IOError(format!("{}", e))
//     }
// }

pub trait PathRelative: StringPath {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf;
}

impl StringPath for FilePath {
    fn relative_to(&self, ancestor: &str) -> String {
        self.to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self.0)
    }
}

impl PathRelative for FilePath {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        self.to(&format!("{}", ancestor.display())).into()
    }
}

impl StringPath for String {
    fn relative_to(&self, ancestor: &str) -> String {
        FilePath::from(self).to(ancestor)
    }
    fn tostring(&self) -> String {
        self.clone()
    }
}

impl PathRelative for String {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        FilePath::from(self)
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for &str {
    fn relative_to(&self, ancestor: &str) -> String {
        FilePath::from(self).to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self)
    }
}

impl PathRelative for &str {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        FilePath::from(self)
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for PathBuf {
    fn relative_to(&self, ancestor: &str) -> String {
        FilePath::from(&format!("{}", self.display())).to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self.display())
    }
}

impl PathRelative for PathBuf {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        FilePath::from(&format!("{}", self.display()))
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for &Path {
    fn relative_to(&self, ancestor: &str) -> String {
        FilePath::from(&format!("{}", self.display())).to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self.display())
    }
}

impl PathRelative for &Path {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        FilePath::from(&format!("{}", self.display()))
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

#[cfg(test)]
mod string_path_tests {
    use crate::fs::StringPath;
    use crate::absolute_path;
    use crate::Exception;

    #[test]
    fn test_path_relative_to() -> Result<(), Exception> {
        let path = absolute_path("~/Music/Ableton")?;
        let ancestor = absolute_path("~/Music/")?;
        assert_eq!(
            path.relative_to(&format!("{}", ancestor.display())),
            "Ableton"
        );
        Ok(())
    }

    #[test]
    fn test_tostring() -> Result<(), Exception> {
        let path = absolute_path("~/Music/Ableton")?;
        assert_eq!(
            path.tostring(),
            format!("{}/Music/Ableton", std::env::var("HOME")?)
        );
        Ok(())
    }
}

#[cfg(test)]
mod nodemeta_integration_tests {
    use crate::fs::FSNode;
    use crate::fs::FilePath;
    use crate::Exception;


    fn get_test_file_path() -> Result<FilePath, Exception> {
        let cwd = std::env::current_dir()?;
        Ok(cwd.join("tests/file.txt").into())
    }

    #[test]
    fn test_filemetadata() -> Result<(), Exception> {
        let readme = get_test_file_path()?;
        let data = FSNode::new(readme.to_path_buf());
        assert_eq!(data.path, readme);
        assert_eq!(data.is_file, true);
        assert_eq!(data.is_dir, false);
        assert_eq!(data.is_symlink, false);
        assert_eq!(data.mode, 0o100644);
        assert_eq!(data.size, 94);
        assert_ne!(data.accessed, None);
        assert_ne!(data.created, None);
        assert_ne!(data.modified, None);
        assert_eq!(&data.modified.unwrap().to_string(), "2023-11-09 05:26:35.945788666 UTC");
        assert_eq!(&data.created.unwrap().to_string(), "2023-11-09 05:26:35.945550333 UTC");
        Ok(())
    }
}


#[cfg(test)]
mod path_relative_tests {
    use crate::fs::PathRelative;
    use crate::absolute_path;
    use crate::Exception;
    use std::path::Path;

    #[test]
    fn test_relative_wherewith() -> Result<(), Exception> {
        let path = absolute_path("~/Music/Ableton")?;
        let ancestor = absolute_path("~/Music/")?;
        assert_eq!(
            Path::new(&path).relative_wherewith(&Path::new(&ancestor)),
            Path::new("Ableton").to_path_buf()
        );
        Ok(())
    }
}
