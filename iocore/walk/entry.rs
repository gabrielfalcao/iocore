//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::fs::{Node, Path, PathType, Size};
use crate::walk::Info;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Entry {
    Directory(Info),
    EmptyDirectory(Info),
    File(Info),
    None(Info),
    Setuid(Info),
    Symlink(Info),
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.variant_unit_id(), self.path().to_string())
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}

impl Entry {
    pub fn from_info(info: &Info) -> Entry {
        let p = info.path();
        use PathType::*;
        match p.kind() {
            File => Entry::File(info.clone()),
            Symlink => Entry::Symlink(info.clone()),
            Setuid => Entry::Setuid(info.clone()),
            Directory => Entry::Directory(info.clone()),
            None => Entry::None(info.clone()),
        }
    }

    pub fn from_path(path: &Path) -> Entry {
        let info = Info::of(path);
        Entry::from_info(&info)
    }

    pub fn path(&self) -> Path {
        self.info().path()
    }

    pub fn node(&self) -> Node {
        self.path().node()
    }

    pub fn info(&self) -> Info {
        use Entry::*;
        match self {
            EmptyDirectory(info) => info.clone(),
            File(info) => info.clone(),
            Symlink(info) => info.clone(),
            Setuid(info) => info.clone(),
            None(info) => info.clone(),
            Directory(info) => info.clone(),
        }
    }

    pub fn size(&self) -> Size {
        self.info().size()
    }

    pub fn increment_size(&mut self, size: Size) {
        if let Entry::Directory(info) = self {
            info.increment_size(size);
        }
    }

    pub fn variant_unit_id(&self) -> String {
        use Entry::*;
        format!(
            "{}::Entry::{}",
            module_path!(),
            match self {
                EmptyDirectory(_) => "EmptyDirectory",
                File(_) => "File",
                Symlink(_) => "Symlink",
                Setuid(_) => "Setuid",
                Directory(_) => "Directory",
                None(_) => "None",
            }
        )
    }

    pub fn id(&self) -> String {
        let mut parts = vec![self.variant_unit_id()];
        use Entry::*;
        match self {
            EmptyDirectory(p) => parts.push(p.to_string()),
            File(p) => parts.push(p.to_string()),
            Symlink(p) => parts.push(p.to_string()),
            Setuid(p) => parts.push(p.to_string()),
            Directory(p) => parts.push(p.to_string()),
            None(p) => parts.push(p.to_string()),
        };
        parts.join("%")
    }
}
impl Hash for Entry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl From<Path> for Entry {
    fn from(p: Path) -> Entry {
        Entry::from_path(&p)
    }
}

impl From<&Entry> for Entry {
    fn from(e: &Entry) -> Entry {
        e.clone()
    }
}

impl From<Info> for Entry {
    fn from(info: Info) -> Entry {
        Entry::from_info(&info)
    }
}

impl Into<Path> for Entry {
    fn into(self) -> Path {
        self.path()
    }
}

impl Into<Info> for Entry {
    fn into(self) -> Info {
        self.info()
    }
}
