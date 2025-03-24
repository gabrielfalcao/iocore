pub mod t;
use thread_groups::ThreadGroup;

use crate::errors::Error;
use crate::{Path, WalkProgressHandler};

fn iocore_walk_dir(
    path: &Path,
    mut handler: impl WalkProgressHandler,
    max_depth: Option<usize>,
    depth: Option<usize>,
) -> Result<Vec<Path>, Error> {
    let path = Into::<Path>::into(path);
    let max_depth = max_depth.unwrap_or(usize::MAX);
    let depth = depth.unwrap_or(0) + 1;
    if !path.exists() {
        return Err(Error::WalkDirError(
            format!("{:#?} does not exist", path.to_string()),
            path,
            depth,
        ));
    }
    if !path.is_directory() {
        return Err(Error::WalkDirError(
            format!("{:#?} is not a directory", path.to_string()),
            path,
            depth,
        ));
    }
    let mut result = Vec::<Path>::new();
    let mut threads: ThreadGroup<Result<Vec<Path>, Error>> =
        ThreadGroup::with_id(format!("walk_dir:{}", path));

    if depth > max_depth {
        return Ok(result);
    }
    if !path.exists() {
        if let Some(error) = handler.error(&path, Error::PathDoesNotExist(path.clone())) {
            return Err(Error::WalkDirError(error.to_string(), path.clone(), depth));
        }
    }
    let path = path.absolute()?;
    for path in path.list()? {
        if path.is_directory() {
            let mut handler = handler.clone();
            match handler.should_scan_directory(&path.clone()) {
                Ok(should_scan_path) =>
                    if should_scan_path {
                        let sub_path = path.clone();
                        let handler = handler.clone();
                        threads.spawn(move || {
                            iocore_walk_dir(
                                &sub_path,
                                handler,
                                Some(max_depth.clone()),
                                Some(depth.clone()),
                            )
                        })?;
                    },
                Err(error) => match handler.error(&path.clone(), error) {
                    Some(error) => {
                        return Err(Error::WalkDirError(error.to_string(), path.clone(), depth));
                    },
                    None => {},
                },
            }
        }
        match handler.path_matching(&path.clone()) {
            Ok(should_aggregate_result) =>
                if should_aggregate_result {
                    result.push(path);
                },
            Err(error) => match handler.error(&path, error) {
                Some(error) => {
                    return Err(Error::WalkDirError(error.to_string(), path.clone(), depth));
                },
                None => {},
            },
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
    if depth <= 1 {
        result.sort();
    }
    Ok(result)
}
/// `walk_dir` traverses the directory referenced in the `path`
/// argument recursively obeying the protocol by the `handler`
/// argument.
///
/// The `max_depth` optionally sets a max depth to stop the traversal
/// gracefully.
pub fn walk_dir(
    path: impl Into<Path>,
    handler: impl WalkProgressHandler,
    max_depth: Option<usize>,
) -> Result<Vec<Path>, Error> {
    let path = Into::<Path>::into(path);
    Ok(iocore_walk_dir(&path, handler, max_depth, None)?)
}

pub fn walk_globs(
    globs: Vec<impl std::fmt::Display>,
    handle: impl WalkProgressHandler,
    max_depth: Option<usize>,
) -> Result<Vec<Path>, Error> {
    let mut result = Vec::<Path>::new();
    let filenames = globs.iter().map(|pattern| pattern.to_string());
    if filenames.len() == 0 {
        result.extend_from_slice(&walk_dir(&Path::cwd(), handle.clone(), max_depth)?)
    } else {
        for pattern in filenames {
            for path in glob(pattern)? {
                if path.is_directory() {
                    result.extend_from_slice(&walk_dir(&path, handle.clone(), max_depth)?);
                } else {
                    result.push(path);
                }
            }
        }
    }
    if result.len() >= 2 {
        result.sort();
    }
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
