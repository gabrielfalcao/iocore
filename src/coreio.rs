use std::fs;
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

use shellexpand;

use crate::exceptions::Exception;


pub fn absolute_path(src: &str) -> Result<PathBuf, Exception> {
    let expanded = String::from(match shellexpand::full(src) {
        Ok(v) => v,
        Err(_) => shellexpand::tilde(src),
    });
    Ok(Path::new(&expanded).canonicalize()?)
}

pub fn ensure_dir_exists(src: &str) -> Result<PathBuf, Exception> {
    let path = absolute_path(src)?;
    if path.try_exists()? {
        fs::create_dir_all(path.clone())?;
        return Ok(path);
    } else if !path.is_dir() {
        return Err(Exception::FileSystemError(format!(
            "{} exists and is not a directory",
            path.to_string_lossy()
        )));
    }
    Ok(path)
}

pub fn absolutely_current_path() -> Result<PathBuf, Exception> {
    let path = std::env::current_dir()?;
    match path.to_str() {
        Some(path) => Ok(absolute_path(path)?),
        None => Err(Exception::FileSystemError(format!("invalid current path"))),
    }
}

pub fn homedir() -> Result<String, Exception> {
    let path = absolute_path("~")?;
    Ok(String::from(path.to_string_lossy()))
}

pub fn resolved_path(src: &str) -> Result<String, Exception> {
    Ok(absolute_path(src)?.to_string_lossy().replace(&homedir()?, "~").to_string())
}

pub fn get_or_create_parent_dir(path: &str) -> Result<String, Exception> {
    let abspath = absolute_path(path)?;
    let path = Path::new(&abspath);
    match path.parent() {
        Some(parent) => {
            std::fs::create_dir_all(parent)?;
            Ok(format!("{}", parent.display()))
        }
        None => Err(Exception::FileSystemError(format!(
            "base path does not have an ancestor {}",
            path.display()
        ))),
    }
}

pub fn open_write(target: &str) -> Result<std::fs::File, Exception> {
    let abspath = absolute_path(target)?;
    get_or_create_parent_dir(abspath.to_str().unwrap())?;
    Ok(OpenOptions::new().create(true).write(true).mode(0o600).open(target)?)
}
