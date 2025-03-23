use iocore::errors::Error;
use iocore::{walk_dir, walk_globs, NoopProgressHandler, Path};
use iocore_test::{folder_path, path_to_test_file, path_to_test_folder};

#[test]
fn test_walk_globs_glob() -> Result<(), Error> {
    assert_eq!(
        walk_globs(vec![format!("iocore/*.rs")], NoopProgressHandler, None)?
            .iter()
            .filter(|entry| !entry.path().name().starts_with("."))
            .map(|entry| entry.path().name())
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
fn test_walk_globs() -> Result<(), Error> {
    let noop_path = path_to_test_folder!("noop");
    ["file5", "file8", "file9"].iter().for_each(|filename| {
        noop_path.join(filename).write_unchecked(&[]);
    });
    let pattern = noop_path.join("*").to_string();
    assert_eq!(noop_path.join("file5").exists(), true);
    assert_eq!(noop_path.join("file8").is_file(), true);
    assert_eq!(noop_path.join("file9").is_file(), true);
    let mut matches = walk_globs(vec![dbg!(pattern)], NoopProgressHandler, None)?
        .iter()
        .map(|entry| entry.path().name())
        .collect::<Vec<String>>();
    matches.sort();
    assert_eq!(matches, vec!["file5", "file8", "file9"]);
    Ok(())
}

#[test]
fn test_walk_dir() -> Result<(), Error> {
    let path = Path::raw("iocore").canonicalize()?;
    assert_eq!(
        walk_dir(&path, NoopProgressHandler, None, None)?
            .iter()
            .filter(|entry| !entry.path().is_directory()
                && !entry.path().name().starts_with(".")
                && !entry.path().name().starts_with("#"))
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
            "walk/t.rs",
            "io/buffer.rs",
            "io/error.rs",
            "io/mod.rs",
            "fs/errors.rs",
            "fs/filename.rs",
            "fs/ls_path_type.rs",
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

#[test]
fn test_walk_dir_fixtures() -> Result<(), Error> {
    let path = folder_path!("fixtures").mkdir_unchecked();
    let entries = walk_dir(&path, NoopProgressHandler, None, None)?;
    let paths = entries
        .iter()
        .map(|entry| entry.path().relative_to(&path).to_string())
        .collect::<Vec<String>>();
    assert_eq!(paths.len(), 146);
    Ok(())
}
