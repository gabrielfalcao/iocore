//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\

use std::fs;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

use crate::exceptions::Exception;
use crate::fs::Path;

pub fn expand_path(path: &str) -> Result<std::path::PathBuf, Exception> {
    Ok(Path::new(path).expand()?.to_path_buf())
}

pub fn absolute_path(path: &str) -> Result<std::path::PathBuf, Exception> {
    Ok(Path::new(path).absolute()?.to_path_buf())
}

pub fn canonical_path(path: &str) -> Result<std::path::PathBuf, Exception> {
    Ok(Path::new(path).canonicalize()?.to_path_buf())
}

pub fn ensure_dir_exists(path: &str) -> Result<std::path::PathBuf, Exception> {
    let path = absolute_path(path)?;
    if let Ok(_) = fs::create_dir_all(path.clone()) {
        return Ok(path);
    } else if !path.is_dir() {
        return Err(Exception::FileSystemError(format!(
            "{} exists and is not a directory",
            path.to_string_lossy()
        )));
    }
    Ok(path)
}

pub fn absolutely_current_path() -> Result<std::path::PathBuf, Exception> {
    let path = std::env::current_dir()?;
    match path.to_str() {
        Some(path) => Ok(absolute_path(path)?),
        None => Err(Exception::FileSystemError(format!("current path seems irretrievable"))),
    }
}

pub fn resolved_path(path: &str) -> Result<String, Exception> {
    Ok(absolute_path(path)?
        .to_string_lossy()
        .replace(&crate::sys::home()?, "~")
        .to_string())
}

pub fn get_or_create_parent_dir(path: &str) -> Result<String, Exception> {
    let path = std::path::Path::new(&path);
    match path.parent() {
        Some(parent) => {
            std::fs::create_dir_all(parent)?;
            Ok(format!("{}", parent.display()))
        },
        None => Err(Exception::FileSystemError(format!(
            "base path does not have an ancestor {}",
            path.display()
        ))),
    }
}

fn open(path: Path, file: &OpenOptions) -> Result<std::fs::File, Exception> {
    match file.open(path.path()) {
        Ok(f) => Ok(f),
        Err(e) => Err(Exception::FileSystemError(format!("opening {}: {}", path, e))),
    }
}

pub fn open_write(target: &str) -> Result<std::fs::File, Exception> {
    open(Path::from(target), OpenOptions::new().create(true).write(true).mode(0o0600))
}

pub fn open_append(target: &str) -> Result<std::fs::File, Exception> {
    open(
        Path::from(target),
        OpenOptions::new().create(true).write(true).append(true).mode(0o0600),
    )
}

pub fn open_read(target: &str) -> Result<std::fs::File, Exception> {
    open(Path::from(target), OpenOptions::new().read(true))
}

pub fn write_file(target: &str, contents: &[u8]) -> Result<Path, Exception> {
    let path = Path::from(target);
    path.write(contents)?;
    Ok(path)
}
pub fn read_file_bytes(target: &str) -> Result<Vec<u8>, Exception> {
    Path::from(target).read_bytes()
}
pub fn read_file(target: &str) -> Result<String, Exception> {
    Path::from(target).read()
}
