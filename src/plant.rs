use std::path::{Path, PathBuf};
use std::os::unix::fs::MetadataExt;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use crate::coreio::absolute_path;
use crate::exceptions::Exception;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeMeta {
    pub path: NodePath,
    pub mode: u32,
    pub accessed: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub size: u64,
    pub is_file: bool,
    pub is_dir: bool,
    pub is_symlink: bool,
}


impl NodeMeta {
    pub fn new(path: PathBuf) -> NodeMeta {
        match std::fs::metadata(&path) {
            Ok(meta) => {
                let path: NodePath = path.into();
                let accessed: Option<SystemTime> = match meta.accessed() { Ok(s) => Some(s), Err(_) => None};
                let modified: Option<SystemTime> = match meta.modified() { Ok(s) => Some(s), Err(_) => None};
                let created: Option<SystemTime> = match meta.created() { Ok(s) => Some(s), Err(_) => None};
                let ft = meta.file_type();
                NodeMeta {
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
            },
            Err(_) => {
                NodeMeta {
                    accessed: None,
                    created: None,
                    modified: None,
                    is_dir: false,
                    is_file: false,
                    is_symlink: false,
                    mode: 0,
                    path: path.into(),
                    size: 0,
                }
            }
        }
    }
}

impl From<PathBuf> for NodeMeta {
    fn from(p: PathBuf) -> NodeMeta {
        NodeMeta::new(p)
    }
}

impl From<&Path> for NodeMeta {
    fn from(p: &Path) -> NodeMeta {
        NodeMeta::new(p.to_path_buf())
    }
}

impl From<NodePath> for NodeMeta {
    fn from(p: NodePath) -> NodeMeta {
        NodeMeta::new(p.to_path_buf())
    }
}


impl From<&str> for NodeMeta {
    fn from(p: &str) -> NodeMeta {
        NodeMeta::new(match absolute_path(p) {
            Ok(p) => p,
            Err(_) => Path::new(&p).to_path_buf()
        })
    }
}

impl From<String> for NodeMeta {
    fn from(p: String) -> NodeMeta {
        NodeMeta::new(p.into())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub path: NodePath,
    pub meta: Option<NodeMeta>
}
impl Node {
    pub fn new(path: &str) -> Result<Node, Exception> {
        Ok(Node {
            path: absolute_path(path)?.into(),
            meta: None
        })
    }
}

impl StringPath for Node {
    fn relative_to(&self, ancestor: &str) -> String {
        self.path.to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self.path)
    }
}

impl PathRelative for Node {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        self.path.to(&format!("{}", ancestor.display())).into()
    }
}

impl From<PathBuf> for Node {
    fn from(p: PathBuf) -> Node {
        Node::new(&format!("{}", p.display())).unwrap()
    }
}

impl From<&str> for Node {
    fn from(p: &str) -> Node {
        Node::new(&match absolute_path(p) {
            Ok(p) => format!("{}", p.display()),
            Err(_) => p.to_string()
        }).unwrap()
    }
}

impl From<String> for Node {
    fn from(p: String) -> Node {
        Node::new(&p).unwrap()
    }
}

impl From<&Path> for Node {
    fn from(p: &Path) -> Node {
        Node::new(&format!("{}", p.display())).unwrap()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodePath(String);

impl NodePath {
    pub fn path(&self) -> &Path {
        Path::new(&self.0)
    }
    pub fn to_path_buf(&self) -> PathBuf {
        self.path().to_path_buf()
    }
}

impl From<PathBuf> for NodePath {
    fn from(p: PathBuf) -> NodePath {
        NodePath(format!("{}", p.display()))
    }
}

impl From<&str> for NodePath {
    fn from(p: &str) -> NodePath {
        NodePath(match absolute_path(p) {
            Ok(p) => format!("{}", p.display()),
            Err(_) => p.to_string()
        })
    }
}

impl From<String> for NodePath {
    fn from(p: String) -> NodePath {
        NodePath(p)
    }
}

impl From<&Path> for NodePath {
    fn from(p: &Path) -> NodePath {
        NodePath(format!("{}", p.display()))
    }
}

impl std::fmt::Display for NodePath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NodePath {
    pub fn from(source: &str) -> Self {
        NodePath(source.to_string())
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

impl StringPath for NodePath {
    fn relative_to(&self, ancestor: &str) -> String {
        self.to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self.0)
    }
}

impl PathRelative for NodePath {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        self.to(&format!("{}", ancestor.display())).into()
    }
}

impl StringPath for String {
    fn relative_to(&self, ancestor: &str) -> String {
        NodePath::from(self).to(ancestor)
    }
    fn tostring(&self) -> String {
        self.clone()
    }
}
impl PathRelative for String {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        NodePath::from(self)
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for &str {
    fn relative_to(&self, ancestor: &str) -> String {
        NodePath::from(self).to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self)
    }

}

impl PathRelative for &str {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        NodePath::from(self)
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for PathBuf {
    fn relative_to(&self, ancestor: &str) -> String {
        NodePath::from(&format!("{}", self.display())).to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self.display())
    }

}
impl PathRelative for PathBuf {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        NodePath::from(&format!("{}", self.display()))
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for &Path {
    fn relative_to(&self, ancestor: &str) -> String {
        NodePath::from(&format!("{}", self.display())).to(ancestor)
    }
    fn tostring(&self) -> String {
        format!("{}", self.display())
    }

}
impl PathRelative for &Path {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        NodePath::from(&format!("{}", self.display()))
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::PathRelative;
    use super::StringPath;
    use crate::absolute_path;
    use crate::Exception;
    use std::path::Path;

    #[test]
    fn test_path_relative() -> Result<(), Exception> {
        let path = absolute_path("~/Music/Ableton")?;
        let ancestor = absolute_path("~/Music/")?;
        assert_eq!(
            path.relative_to(&format!("{}", ancestor.display())),
            "Ableton"
        );
        assert_eq!(
            Path::new(&path).relative_wherewith(&Path::new(&ancestor)),
            Path::new("Ableton").to_path_buf()
        );
        Ok(())
    }
}
