pub mod t;
use thread_groups::ThreadGroup;

use crate::errors::Error;
use crate::fs::path_cmp::cmp_paths_by_parts;
use crate::{Path, WalkProgressHandler};

pub fn walk_dir(
    path: impl Into<Path>,
    mut handler: impl WalkProgressHandler,
    max_depth: Option<usize>,
    depth: Option<usize>,
) -> Result<Vec<Path>, Error> {
    let path = Into::<Path>::into(path);
    if !path.exists() {
        return Err(Error::WalkDirError(format!("{:#?} does not exist", path.to_string()), path));
    }
    if !path.is_directory() {
        return Err(Error::WalkDirError(
            format!("{:#?} is not a directory", path.to_string()),
            path,
        ));
    }
    let max_depth = max_depth.unwrap_or(u8::MAX as usize);
    let depth = depth.unwrap_or(0) + 1;
    let mut result = Vec::<Path>::new();
    let mut threads: ThreadGroup<Result<Vec<Path>, Error>> =
        ThreadGroup::with_id(format!("walk_dir:{}", path));

    if depth - 1 > max_depth {
        return Ok(result);
    }
    if !path.exists() {
        if let Some(error) = handler.error(&path, Error::PathDoesNotExist(path.clone())) {
            return Err(error);
        }
    }
    let path = path.absolute()?;
    for path in path.list()? {
        if path.is_directory() {
            let handler = handler.clone();
            result.push(path.clone());
            threads.spawn(move || {
                walk_dir(&path, handler, Some(max_depth.clone()), Some(depth.clone()))
            })?;
        } else {
            result.push(path);
        }
    }
    for paths in threads
        .results()
        .iter()
        .filter(|path| path.is_ok())
        .map(|path| path.clone().unwrap())
        .flatten()
    {
        for path in paths {
            result.push(path);
        }
    }
    result.sort_by(|a, b| cmp_paths_by_parts(&a, &b));
    Ok(result)
}

pub fn walk_globs(
    globs: Vec<impl std::fmt::Display>,
    handle: impl WalkProgressHandler,
    max_depth: Option<usize>,
) -> Result<Vec<Path>, Error> {
    let mut result = Vec::<Path>::new();
    let filenames = globs.iter().map(|pattern| pattern.to_string());
    if filenames.len() == 0 {
        result.extend_from_slice(&walk_dir(&Path::cwd(), handle.clone(), max_depth, None)?)
    } else {
        for pattern in filenames {
            for path in glob(pattern)? {
                if path.is_directory() {
                    result.extend_from_slice(&walk_dir(&path, handle.clone(), max_depth, None)?);
                } else {
                    result.push(path);
                }
            }
        }
    }
    result.sort();
    Ok(result)
}

pub fn glob(pattern: impl Into<String>) -> Result<Vec<Path>, Error> {
    let mut result = Vec::<Path>::new();
    let pattern = pattern.into();
    for filename in match ::glob::glob(&pattern) {
        Err(e) => return Err(Error::MalformedGlobPattern(format!("{}: {}", pattern, e))),
        Ok(paths) => paths,
    } {
        let path = match filename {
            Ok(filename) => Path::from(filename),
            Err(e) => return Err(Error::FileSystemError(format!("{}: {}", pattern, e))),
        };
        result.push(path);
    }
    Ok(result)
}
