use crate::plant::PathRelative;
use shellexpand;
use std::fs;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::exceptions::Exception;
pub fn rsvfilematch<M: FnMut(&PathBuf) -> bool>(
    filenames: Vec<String>,
    mut matcher: M,
) -> Result<Vec<PathBuf>, Exception> {
    let mut result = Vec::<PathBuf>::new();
    for filename in if filenames.len() == 0 {
        vec![format!("{}", std::env::current_dir()?.display())]
    } else {
        filenames.clone()
    }
    .iter()
    {
        let path = absolute_path(filename.as_str())?;
        if !path.try_exists()? {
            continue;
        }
        if path.is_dir() {
            for entry in WalkDir::new(path) {
                let entry = entry?.clone();
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }
                if matcher(&path.to_path_buf()) {
                    result.push(path.to_path_buf().relative_wherewith(path));
                }
            }
        } else {
            if matcher(&path.to_path_buf()) {
                result.push(path.to_path_buf());
            }
        }
    }
    Ok(result)
}

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
    Ok(absolute_path(src)?
        .to_string_lossy()
        .replace(&homedir()?, "~")
        .to_string())
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
    Ok(OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o600)
        .open(&abspath)?)
}

pub fn open_read(target: &str) -> Result<std::fs::File, Exception> {
    let abspath = absolute_path(target)?;
    Ok(OpenOptions::new().read(true).open(abspath)?)
}

#[cfg(test)]
mod functests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_ow() -> Result<(), Exception> {
        let filez = [
            "foo/bar/123/45/6.bin",
            "foo/bar/123/44/6.bin",
            "foo/bar/123/43/6.bin",
            "foo/bar/123/42/6.bin",
            "foo/bar/111/30/6.bin",
            "foo/bar/111/222/333.bin",
            "foo/bar/111/333/444.bin",
            "foo/bar/111/444/555.bin",
            "foo/baz/123/45/6.bin",
            "foo/baz/123/44/6.bin",
            "foo/baz/123/43/6.bin",
            "foo/baz/123/42/6.bin",
            "foo/baz/111/30/6.bin",
            "foo/baz/111/222/333.bin",
            "foo/baz/111/333/444.bin",
            "foo/baz/111/444/555.bin",
        ]
        .iter()
        .map(|p| {
            match open_write(p) {
                Ok(mut f) => match f.write_all(b"test1234") {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("cannot write {}: {}", p, e);
                    }
                },
                Err(e) => {
                    eprintln!("cannot write {}: {}", p, e);
                }
            };
            p.to_string()
        })
        .collect::<Vec<String>>();

        assert_eq!(
            filez,
            [
                "foo/bar/123/45/6.bin",
                "foo/bar/123/44/6.bin",
                "foo/bar/123/43/6.bin",
                "foo/bar/123/42/6.bin",
                "foo/bar/111/30/6.bin",
                "foo/bar/111/222/333.bin",
                "foo/bar/111/333/444.bin",
                "foo/bar/111/444/555.bin",
                "foo/baz/123/45/6.bin",
                "foo/baz/123/44/6.bin",
                "foo/baz/123/43/6.bin",
                "foo/baz/123/42/6.bin",
                "foo/baz/111/30/6.bin",
                "foo/baz/111/222/333.bin",
                "foo/baz/111/333/444.bin",
                "foo/baz/111/444/555.bin"
            ]
            .to_vec()
        );
        let matches = rsvfilematch(vec![format!(".")], |path| {
            path.starts_with(&homedir().unwrap()) && path.ends_with(".bin")
        })?;
        assert_eq!(matches, Vec::<PathBuf>::new());
        Ok(())
    }
}
