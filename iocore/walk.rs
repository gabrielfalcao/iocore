use std::collections::HashSet;

use thread_groups::ThreadGroup;

use crate::{Error, Path};

pub type MaxDepth = usize;
pub type Depth = usize;

fn iocore_walk_dir(
    path: &Path,
    mut handler: impl WalkProgressHandler,
    max_depth: Option<MaxDepth>,
    depth: Option<Depth>,
) -> Result<HashSet<Path>, Error> {
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
    let mut result = HashSet::<Path>::new();
    let mut threads: ThreadGroup<Result<HashSet<Path>, Error>> =
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
                    result.insert(path);
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
        for path in paths.iter() {
            match handler.path_matching(path) {
                Ok(should_aggregate_result) =>
                    if should_aggregate_result {
                        result.insert(path.clone());
                    },
                Err(error) => match handler.error(path, error) {
                    Some(error) => {
                        return Err(Error::WalkDirError(error.to_string(), path.clone(), depth));
                    },
                    None => {},
                },
            }
        }
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
    let mut result = Vec::<Path>::from_iter(
        iocore_walk_dir(&path, handler, max_depth, None)?
            .iter()
            .map(|path| path.clone()),
    );
    result.sort();
    Ok(result)
}

pub fn walk_globs(
    globs: Vec<impl std::fmt::Display>,
    handle: impl WalkProgressHandler,
    max_depth: Option<MaxDepth>,
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

pub fn glob(pattern: impl std::fmt::Display) -> Result<Vec<Path>, Error> {
    let mut result = Vec::<Path>::new();
    let pattern = pattern.to_string();
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

/// `WalkProgressHandler` trait defines a protocol outlining the
/// behavior of [`iocore::walk_dir`] in terms of whether to
/// aggregate paths in the final result, whether to scan a directory
/// and whether an error should cause [`iocore::walk_dir`] to
/// fail.
pub trait WalkProgressHandler: Send + Sync + 'static + Clone {
    /// `path_matching` is called to determine whether
    /// [`iocore::walk_dir`] should aggregate the given `path`
    /// argument in its final result.
    ///
    /// If the implementor returns [`std::result::Result::Err`] then
    /// the error is handled by [`iocore::walkProgressHandler::error`] which
    /// cascades the error all the way up to the initial call if
    /// [`Some(error)`] is returned.
    ///
    /// If the implementor returns [`Ok(false)`] the given `path` will
    /// not be aggregated in the final result.
    fn path_matching(&mut self, path: &Path) -> std::result::Result<bool, Error>;

    /// `should_scan_directory` is only called when `path` argument is a directory.
    ///
    /// Implementors return [`Ok(false)`] to indicate that
    /// [`iocore::walk_dir`] shall skip scanning directory.
    ///
    /// If the implementor returns [`std::result::Result::Err`] then
    /// the error is handled by [`iocore::walkProgressHandler::error`] which
    /// cascades the error all the way up to the initial call if
    /// [`Some(error)`] is returned.
    ///
    /// Default implementation always returns [`Ok(true)`].
    ///
    ///
    /// > NOTE: [`iocore::walk_dir`] spawns (i.e.:
    /// > [`thread_groups::ThreadGroup::spawn`]) a sub thread calling
    /// > [`iocore::walk_dir`] (in assynchronously recursively
    /// > fashion) with the directory referenced in the `path` argument
    /// > which is then "joined" (via
    /// > [`thread_groups::ThreadGroup::results`]) at the end of each
    /// > `walk_dir` function.
    fn should_scan_directory(&mut self, path: &Path) -> std::result::Result<bool, Error> {
        Ok(path.is_directory())
    }
    /// `error` is called when [`Err(iocore::Error)`] arises anywhere
    /// within a [`iocore::walk_dir`] call so that implementors
    /// can choose how to handle errors.
    ///
    /// Default implementation always returns [`Some(error)`].
    fn error(&mut self, _path_: &Path, error: Error) -> Option<Error> {
        Some(error)
    }
}

/// `NoopProgressHandler` is the builtin implementation of
/// [`iocore::WalkProgressHandler`] which aggregates results insofar
/// as the `path` given to `path_matching` exists at the moment the
/// calling thread calls it.
#[derive(Clone, Eq, PartialEq)]
pub struct NoopProgressHandler;
pub type WalkDirDepth = usize;
impl WalkProgressHandler for NoopProgressHandler {
    fn path_matching(&mut self, path: &Path) -> std::result::Result<bool, Error> {
        Ok(path.exists())
    }

    fn should_scan_directory(&mut self, path: &Path) -> std::result::Result<bool, Error> {
        Ok(path.is_directory())
    }
}
