use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Node {
    source: String,
}

impl Node {
    pub fn from(source: &str) -> Self {
        Node {
            source: source.to_string(),
        }
    }
    pub fn to(&self, ancestor: &str) -> String {
        self.source
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
pub trait StringPath: Clone {
    fn relative_to(&self, ancestor: &str) -> String;
}

pub trait PathRelative: StringPath {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf;
}

impl StringPath for Node {
    fn relative_to(&self, ancestor: &str) -> String {
        self.to(ancestor)
    }
}

impl PathRelative for Node {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        self.to(&format!("{}", ancestor.display())).into()
    }
}

impl StringPath for String {
    fn relative_to(&self, ancestor: &str) -> String {
        Node::from(self).to(ancestor)
    }
}
impl PathRelative for String {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        Node::from(self)
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for &str {
    fn relative_to(&self, ancestor: &str) -> String {
        Node::from(self).to(ancestor)
    }
}

impl PathRelative for &str {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        Node::from(self)
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for PathBuf {
    fn relative_to(&self, ancestor: &str) -> String {
        Node::from(&format!("{}", self.display())).to(ancestor)
    }
}
impl PathRelative for PathBuf {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        Node::from(&format!("{}", self.display()))
            .to(&format!("{}", ancestor.display()))
            .into()
    }
}

impl StringPath for &Path {
    fn relative_to(&self, ancestor: &str) -> String {
        Node::from(&format!("{}", self.display())).to(ancestor)
    }
}
impl PathRelative for &Path {
    fn relative_wherewith(&self, ancestor: &Path) -> PathBuf {
        Node::from(&format!("{}", self.display()))
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
