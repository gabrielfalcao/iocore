use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::Size;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Info {
    path: crate::fs::Path,
    size: Size,
}

impl PartialOrd for Info {
    fn partial_cmp(&self, other: &Info) -> Option<Ordering> {
        self.path().partial_cmp(&other.path())
    }
}
impl Ord for Info {
    fn cmp(&self, other: &Info) -> Ordering {
        self.path().cmp(&other.path())
    }
}

impl Hash for Info {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.size.hash(state);
    }
}

impl Info {
    pub fn of(path: &crate::fs::Path) -> Info {
        let path = path.clone();
        let size = path.size();
        Info { path, size }
    }

    pub fn path(&self) -> crate::fs::Path {
        self.path.clone()
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn increment_size(&mut self, size: impl Into<Size>) -> Size {
        self.size += size.into();
        self.size
    }
}

impl From<crate::fs::Path> for Info {
    fn from(path: crate::fs::Path) -> Info {
        Info::of(&path)
    }
}
impl Display for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl Debug for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.path, self.size)
    }
}
