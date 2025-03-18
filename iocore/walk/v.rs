use crate::errors::Error;
use crate::fs::{Node, Path};
use crate::walk::t::{Depth, MaxDepth};

pub fn walk_dir<M, E>(
    path: &Path,
    matcher: &mut M,
    error_handler: &mut E,
    max_depth: Option<MaxDepth>,
    depth: Option<Depth>,
) -> Result<Vec<Path>, Error>
where
    M: FnMut(&Path, &Node) -> bool,
    E: FnMut(&Path, Error) -> Option<Error>,
{
    let mut result = Vec::<Path>::new();
    let node = path.node();

    if node.is_dir && matcher(&path, &node) {
        for entry in match std::fs::read_dir(&path.to_path_buf()) {
            Ok(entry) => entry,
            Err(y) => {
                let x = format!("{}", y);
                match error_handler(path, y.into()) {
                    Some(exc) => return Err(exc),
                    None =>
                        return Err(Error::WalkDirInterrupted(
                            x,
                            node,
                            if let Some(depth) = depth { depth } else { 0 },
                        )),
                }
            },
        } {
            let spath = Path::from(entry?.path());
            let snode = spath.node();
            if matcher(&path, &snode) {
                result.push(spath);
            }
        }
    } else if let Some(exc) =
        error_handler(path, Error::FileSystemError(format!("not a directory: {}", path)))
    {
        return Err(exc);
    }

    if depth.map(|d| d < max_depth.unwrap_or(usize::MAX)).unwrap_or(true) {
        let depth = depth.unwrap_or(1);
        for path in result.clone().iter().filter(|n| n.is_dir()).collect::<Vec<_>>() {
            result.extend(walk_dir(path, matcher, error_handler, max_depth, Some(depth))?);
        }
    }
    Ok(result)
}

pub fn walk_nodes<M, E>(
    filenames: Vec<String>,
    matcher: &mut M,
    error_handler: &mut E,
    max_depth: Option<MaxDepth>,
) -> Result<Vec<Node>, Error>
where
    M: FnMut(&Path, &Node) -> bool,
    E: FnMut(&Path, Error) -> Option<Error>,
{
    let mut result = Vec::<Node>::new();

    let cwd = Path::from(absolutely_current_path()?);

    let mut origins = Vec::<Node>::new();

    if filenames.len() == 0 {
        origins.push(cwd.node());
    } else {
        for filename in &filenames {
            origins.extend_from_slice(
                &match glob::glob(filename) {
                    Ok(paths) => paths,
                    Err(e) => {
                        if let Some(exc) = &error_handler(
                            &cwd.join(&filename),
                            Error::FileSystemError(format!("expanding {}: {}", filename, e)),
                        ) {
                            return Err(exc.clone());
                        } else {
                            continue;
                        }
                    },
                }
                .map(|s| cwd.join(s.expect(&format!("expanding {}", filename))))
                .map(|p| p.node())
                .collect::<Vec<Node>>(),
            )
        }
    };

    for node in origins {
        let path = node.path();
        if node.is_dir {
            match walk_dir(&path, matcher, error_handler, max_depth, None) {
                Ok(paths) =>
                    result.extend_from_slice(&paths.iter().map(|p| p.node()).collect::<Vec<_>>()),
                Err(e) =>
                    if let Some(exc) = &error_handler(&path, e) {
                        return Err(exc.clone());
                    } else {
                        continue;
                    },
            }
        } else if matcher(&path, &node) {
            result.push(node.clone());
        }
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
    fn test_walk_nodes_glob() -> Result<(), Error> {
        assert_eq!(
            walk_nodes(
                vec![format!("src/*.rs")],
                &mut |_path, _node| { true },
                &mut |_path, _exc| { None },
                None
            )
            .unwrap()
            .iter()
            .map(|node| node.filename())
            .collect::<Vec<String>>(),
            vec!["coreio.rs", "errors.rs", "fs.rs", "lib.rs", "sys.rs", "walk.rs"]
        );
        Ok(())
    }
    #[test]
    fn test_walk_nodes() -> Result<(), Error> {
        let file_paths =
            ["tests/noop/1.o", "tests/noop/6.ld", "tests/noop/8.dll", "tests/abba/6.dll"]
                .iter()
                .map(|n| Path::from(*n))
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
        let mut matches = walk_nodes(
            vec![format!("tests/*")],
            &mut |path, _node| path.tostring().contains("noop"),
            &mut |_path, _exc| None,
            None,
        )?
        .iter()
        .map(|node| node.filename())
        .collect::<Vec<String>>();
        matches.sort();
        // let matches = file_paths.iter().map(|p|p.name()).collect::<Vec<_>>();
        assert_eq!(matches, vec!["1.o", "6.ld", "8.dll"]);
        Ok(())
    }
}
