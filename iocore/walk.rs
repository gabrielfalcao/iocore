pub mod entry;
pub mod info;
pub mod t;

use entry::Entry;
use info::Info;
use t::{NoopProgressHandler, WalkProgressHandler};
use thread_groups::ThreadGroup;

use crate::errors::Error;
use crate::fs::{cmp_paths_by_length, cmp_paths_by_parts};
use crate::{Path, PathType, Size};

pub fn read_dir(
    path: &Path,
    handle: impl WalkProgressHandler + Clone,
) -> Result<Vec<Entry>, Error> {
    let mut result = Vec::<Entry>::new();
    let path = path.clone();

    let mut entries = std::fs::read_dir(&path.to_path_buf())
        .map_err(|e| match handle.clone().error(&path, e.into()) {
            Some(y) => Error::WalkDirError(y.to_string(), path.node()),
            None => Error::WalkDirError("unable to read directory".to_string(), path.node()),
        })
        .map(|entries| {
            entries.map(|entry| {
                entry
                    .map_err(|e| match handle.clone().error(&path, e.into()) {
                        Some(y) => Error::WalkDirError(y.to_string(), path.node()),
                        None => Error::WalkDirError(
                            "unable to read directory".to_string(),
                            path.node(),
                        ),
                    })
                    .map(|entry| Entry::from(Path::from(entry)))
            })
        })?
        .collect::<Vec<Result<Entry, Error>>>();

    entries.sort_by(|a, b| {
        if a.is_ok() && b.is_ok() {
            let a = a.clone().unwrap();
            let b = b.clone().unwrap();
            cmp_paths_by_parts(&a.path(), &b.path())
        } else {
            a.is_ok().cmp(&b.is_ok())
        }
    });
    for entry in entries {
        let entry = entry?;
        if handle.clone().path_matching(&entry.path(), &entry.node()) {
            result.push(entry);
        }
    }
    sort_entries(&mut result);
    Ok(result)
}

pub fn read_dir_size(path: &Path, progress: &mut impl FnMut(&Path, usize)) -> Result<Size, Error> {
    let info = Info::of(path);
    let mut size = info.size();
    let paths = read_dir(path, NoopProgressHandler)?;
    let count = paths.len();
    progress(path, count);
    size += paths
        .iter()
        .map(|entry| {
            progress(&entry.path(), count);
            if let Entry::Directory(d) = entry {
                read_dir_size(&d.path(), progress).unwrap_or(entry.size())
            } else {
                entry.size()
            }
        })
        .sum();

    Ok(size)
}

pub fn walk_dir(
    path: impl Into<Path>,
    mut handle: impl WalkProgressHandler + Clone,
    max_depth: Option<usize>,
    depth: Option<usize>,
) -> Result<Vec<Entry>, Error> {
    let path = Into::<Path>::into(path);
    let max_depth = max_depth.unwrap_or(u8::MAX as usize);
    let depth = depth.unwrap_or(0) + 1;
    let mut result = Vec::<Entry>::new();
    let mut threads: ThreadGroup<Result<Vec<Entry>, Error>> =
        ThreadGroup::with_id(format!("walk_dir:{}", path));

    if depth - 1 > max_depth {
        return Ok(result);
    }
    if !path.exists() {
        if let Some(error) = handle.error(&path, Error::PathDoesNotExist(path.clone())) {
            return Err(error);
        } else {
            return Ok(result);
        }
    }
    let path = path.absolute()?;
    for entry in read_dir(&path, handle.clone())? {
        let path = entry.path();
        if path.is_dir() {
            let handle = handle.clone();
            result.push(Entry::from(Info::of(&path)));
            threads.spawn(move || {
                walk_dir(&path, handle, Some(max_depth.clone()), Some(depth.clone()))
            })?;
        } else {
            result.push(Entry::from(Info::of(&path)));
        }
    }
    for entries in threads.results().iter().map(|o| o.clone().unwrap()).flatten() {
        for mut entry in entries {
            if let Entry::Directory(info) = entry.clone() {
                let subentries =
                    walk_dir(&info.path(), handle.clone(), Some(max_depth), Some(depth + 1))?;
                entry.increment_size(subentries.iter().map(|s| s.size()).sum());
                result.push(entry);
                result.extend_from_slice(&subentries);
            } else {
                result.push(entry);
            }
        }
    }
    sort_entries(&mut result);
    Ok(result)
}

pub fn walk_nodes(
    filenames: Vec<String>,
    handle: impl WalkProgressHandler + Clone,
    max_depth: Option<usize>,
) -> Result<Vec<Entry>, Error> {
    let mut result = Vec::<Entry>::new();

    if filenames.len() == 0 {
        result.extend_from_slice(&walk_dir(&Path::cwd(), handle.clone(), max_depth, None)?)
    } else {
        for pattern in filenames {
            for path in glob(pattern)? {
                match path.kind() {
                    PathType::Directory => {
                        result.extend_from_slice(&walk_dir(
                            &path,
                            handle.clone(),
                            max_depth,
                            None,
                        )?);
                    },
                    _ => {
                        result.push(Entry::from(path));
                    },
                }
            }
        }
    }
    sort_entries(&mut result);
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
        result.push(path.absolute()?);
    }
    Ok(result)
}
pub(crate) fn sort_entries(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| cmp_paths_by_length(&a.path(), &b.path()));
    entries.sort_by(|a, b| cmp_paths_by_parts(&a.path(), &b.path()));
}
