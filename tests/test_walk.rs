use std::path::PathBuf;

use iocore::coreio::absolute_path;
use iocore::errors::Error;
use iocore::{walk_nodes, NoopProgressHandler, Path};

#[test]
fn test_walk_nodes_glob() -> Result<(), Error> {
    assert_eq!(
        walk_nodes(vec![format!("iocore/*.rs")], NoopProgressHandler.clone(), None)
            .unwrap()
            .iter()
            .map(|entry| entry.node().filename())
            .collect::<Vec<String>>(),
        vec![
            "coreio.rs",
            "errors.rs",
            "walk.rs",
            "env.rs",
            "lib.rs",
            "sys.rs",
            "fs.rs",
            "sh.rs"
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
