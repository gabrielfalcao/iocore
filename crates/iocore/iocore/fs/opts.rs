use std::fmt::Display;
use std::os::unix::fs::OpenOptionsExt;

use crate::Error;

#[derive(Debug)]
pub struct OpenOptions {
    f_read: bool,
    f_write: bool,
    f_create: bool,
    f_append: bool,
    f_mode: u32,
}
impl Display for OpenOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut opts = Vec::<String>::new();
        if self.f_read {
            opts.push(String::from("read"));
        }
        if self.f_write {
            opts.push(String::from("write"));
        }
        if self.f_create {
            opts.push(String::from("create"));
        }
        if self.f_append {
            opts.push(String::from("append"));
        }
        if self.f_mode > 0o0000 {
            opts.push(format!("mode:{:40o}", self.f_mode));
        }
        write!(f, "[{}]", opts.join(","))
    }
}
impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            f_read: bool::default(),
            f_create: bool::default(),
            f_append: bool::default(),
            f_write: bool::default(),
            f_mode: 0o0600,
        }
    }

    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.f_read = read;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.f_create = create;
        self
    }

    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.f_append = append;
        self
    }

    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.f_write = write;
        self
    }

    pub fn mode(&mut self, mode: u32) -> &mut OpenOptions {
        self.f_mode = mode;
        self
    }

    pub fn open<T: Into<crate::fs::Path>>(&self, path: T) -> Result<std::fs::File, Error> {
        let path = path.into();
        Ok(if path.exists() && self.f_mode > 0o0000 {
            std::fs::OpenOptions::new()
                .create(self.f_create)
                .read(self.f_read)
                .write(self.f_write)
                .append(self.f_append)
                .mode(self.f_mode)
                .open(&path)
        } else {
            std::fs::OpenOptions::new()
                .create(self.f_create)
                .read(self.f_read)
                .write(self.f_write)
                .append(self.f_append)
                .open(&path)
        }
        .map_err(|e| {
            Error::FileSystemError(format!(
                "{}:{} {:#?}: {}",
                file!(),
                line!(),
                Into::<crate::Path>::into(path).to_string(),
                e.to_string()
            ))
        })?)
    }
}
