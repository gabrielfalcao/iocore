use std::path::PathBuf;

use iocore::coreio::absolute_path;
use iocore::errors::Error;
use iocore::{walk_dir, walk_nodes, NoopProgressHandler, Path};

#[test]
fn test_walk_nodes_glob() -> Result<(), Error> {
    assert_eq!(
        walk_nodes(vec![format!("iocore/*.rs")], NoopProgressHandler, None)
            .unwrap()
            .iter()
            .filter(|entry| !entry.node().filename().starts_with("."))
            .map(|entry| entry.node().filename())
            .collect::<Vec<String>>(),
        vec![
            "coreio.rs",
            "env.rs",
            "errors.rs",
            "fs.rs",
            "lib.rs",
            "sh.rs",
            "sys.rs",
            "walk.rs",
        ]
    );
    Ok(())
}
#[test]
fn test_walk_nodes() -> Result<(), Error> {
    let file_paths = ["tests/noop/1.o", "tests/noop/6.ld", "tests/noop/8.dll", "tests/abba/6.dll"]
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

#[test]
fn test_walk_dir() -> Result<(), Error> {
    let path = Path::raw("iocore");
    assert_eq!(
        walk_dir(&path, NoopProgressHandler, None, None)
            .unwrap()
            .iter()
            .filter(
                |entry| !entry.node().filename().starts_with(".") && !entry.path().is_directory()
            )
            .map(|entry| entry.path().relative_to(&path).to_string())
            .collect::<Vec<String>>(),
        vec![
            "coreio.rs",
            "env.rs",
            "errors.rs",
            "fs.rs",
            "lib.rs",
            "sh.rs",
            "sys.rs",
            "walk.rs",
            "walk/entry.rs",
            "walk/info.rs",
            "walk/s.rs",
            "walk/t.rs",
            "walk/v.rs",
            "io/buffer.rs",
            "io/error.rs",
            "io/mod.rs",
            "fs/errors.rs",
            "fs/filename.rs",
            "fs/ls_node_type.rs",
            "fs/node.rs",
            "fs/opts.rs",
            "fs/path_status.rs",
            "fs/path_timestamps.rs",
            "fs/path_type.rs",
            "fs/path_utils.rs",
            "fs/perms.rs",
            "fs/size.rs",
            "fs/timed.rs"
        ]
    );
    Ok(())
}
