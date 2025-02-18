//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\
pub mod entry;
pub mod info;
pub mod t;
pub use entry::Entry;
pub use info::Info;
pub use t::*;
use thread_groups::ThreadGroup;

use crate::exceptions::Exception;
use crate::fs::{Path, PathType, Size};

pub fn read_dir(
    path: &Path,
    handle: impl WalkProgressHandler + Clone,
) -> Result<Vec<Entry>, Exception> {
    let mut result = Vec::<Entry>::new();
    let path = path.clone();

    for entry in std::fs::read_dir(&path.to_path_buf())
        .map_err(|e| match handle.clone().error(&path, e.into()) {
            Some(y) => Exception::WalkDirError(y.to_string(), path.node()),
            None => Exception::WalkDirError("unable to read directory".to_string(), path.node()),
        })
        .map(|entries| {
            entries.map(|entry| {
                entry
                    .map_err(|e| match handle.clone().error(&path, e.into()) {
                        Some(y) => Exception::WalkDirError(y.to_string(), path.node()),
                        None => Exception::WalkDirError(
                            "unable to read directory".to_string(),
                            path.node(),
                        ),
                    })
                    .map(|entry| Entry::from(Path::from(entry)))
            })
        })?
    {
        let entry = entry?;
        if handle.clone().path_matching(&entry.path(), &entry.node()) {
            result.push(entry);
        }
    }
    Ok(result)
}

pub fn read_dir_size(
    path: &Path,
    progress: &mut impl FnMut(&Path, usize),
) -> Result<Size, Exception> {
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
    path: &Path,
    mut handle: impl WalkProgressHandler + Clone,
    max_depth: Option<usize>,
    depth: Option<usize>,
) -> Result<Vec<Entry>, Exception> {
    let max_depth = max_depth.unwrap_or(u8::MAX as usize);
    let depth = depth.unwrap_or(0) + 1;
    let mut result = Vec::<Entry>::new();
    let mut threads: ThreadGroup<Result<Vec<Entry>, Exception>> =
        ThreadGroup::with_id(format!("walk_dir:{}", path));

    if depth - 1 > max_depth {
        return Ok(result);
    }
    if !path.exists() {
        if let Some(error) = handle.error(path, Exception::PathDoesNotExist(path.clone())) {
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

    Ok(result)
}

pub fn walk_nodes(
    filenames: Vec<String>,
    handle: impl WalkProgressHandler + Clone,
    max_depth: Option<usize>,
) -> Result<Vec<Entry>, Exception> {
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
    Ok(result)
}

pub fn glob(pattern: impl Into<String>) -> Result<Vec<Path>, Exception> {
    let mut result = Vec::<Path>::new();
    let pattern = pattern.into();
    for filename in match ::glob::glob(&pattern) {
        Err(e) => return Err(Exception::MalformedGlobPattern(format!("{}: {}", pattern, e))),
        Ok(paths) => paths,
    } {
        let path = match filename {
            Ok(filename) => Path::from(filename),
            Err(e) => return Err(Exception::FileSystemError(format!("{}: {}", pattern, e))),
        };
        result.push(path.absolute()?);
    }
    Ok(result)
}

#[cfg(test)]
mod functests {
    use std::path::PathBuf;

    use crate::coreio::absolute_path;
    use crate::fs::*;
    use crate::walk::*;

    #[test]
    fn test_walk_nodes_glob() -> Result<(), Exception> {
        assert_eq!(
            walk_nodes(vec![format!("iocore/*.rs")], NoopProgressHandler.clone(), None)
                .unwrap()
                .iter()
                .map(|entry| entry.node().filename())
                .collect::<Vec<String>>(),
            vec!["coreio.rs", "exceptions.rs", "fs.rs", "lib.rs", "sh.rs", "sys.rs", "walk.rs"]
        );
        Ok(())
    }
    #[test]
    fn test_walk_nodes() -> Result<(), Exception> {
        let file_paths =
            ["tests/noop/1.o", "tests/noop/6.ld", "tests/noop/8.dll", "tests/abba/6.dll"]
                .iter()
                .map(|n| Path::writable_file(*n).unwrap())
                .map(|s| s.write(b"!!!!!!!").unwrap_or(s.clone()))
                .collect::<Vec<Path>>();

        let absbufs = file_paths
            .iter()
            .map(|p| p.try_absolute().to_path_buf())
            .collect::<Vec<PathBuf>>();

        assert_eq!(
            absbufs,
            ["tests/noop/1.o", "tests/noop/6.ld", "tests/noop/8.dll", "tests/abba/6.dll",]
                .iter()
                .map(|p| absolute_path(p).unwrap_or(Path::from(p).to_path_buf()))
                .collect::<Vec<_>>()
        );
        let mut matches = walk_nodes(vec![format!("tests/noop/*")], NoopProgressHandler, None)?
            .iter()
            .map(|entry| entry.node().filename())
            .collect::<Vec<String>>();
        matches.sort();
        // let matches = file_paths.iter().map(|p|p.name()).collect::<Vec<_>>();
        assert_eq!(matches, vec!["1.o", "6.ld", "8.dll"]);
        Ok(())
    }
}
