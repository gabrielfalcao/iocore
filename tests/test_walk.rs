use iocore::errors::Error;
use iocore::{glob, walk_dir, walk_globs, NoopProgressHandler, Path};
use iocore_test::{folder_path, path_to_test_file, path_to_test_folder};

#[test]
fn test_walk_globs_lib_folder() -> Result<(), Error> {
    assert_eq!(
        walk_globs(vec![format!("iocore/*.rs")], NoopProgressHandler, None)?
            .iter()
            .filter(|path| !path.name().starts_with("."))
            .map(|path| path.name())
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
    let target_glob_path = path_to_test_folder!();
    ["file5", "file8", "file9"].iter().for_each(|filename| {
        target_glob_path.join(filename).write_unchecked(&[]);
    });
    let pattern = target_glob_path.join("*").to_string();
    assert_eq!(target_glob_path.join("file5").exists(), true);
    assert_eq!(target_glob_path.join("file8").is_file(), true);
    assert_eq!(target_glob_path.join("file9").is_file(), true);
    let glob_matches = glob(&pattern)?.iter().map(|path| path.to_string()).collect::<Vec<String>>();
    assert_eq!(
        glob_matches,
        vec![
            "tests/test_walk/test_walk_globs/file5",
            "tests/test_walk/test_walk_globs/file8",
            "tests/test_walk/test_walk_globs/file9"
        ]
    );
    let mut matches = walk_globs(vec![dbg!(pattern)], NoopProgressHandler, None)?
        .iter()
        .map(|path| path.name())
        .collect::<Vec<String>>();
    matches.sort();
    assert_eq!(matches, vec!["file5", "file8", "file9"]);
    Ok(())
}
#[test]
fn test_glob() -> Result<(), Error> {
    let target_glob_path = path_to_test_folder!();
    ["file5", "file8", "file9"].iter().for_each(|filename| {
        target_glob_path.join(filename).write_unchecked(&[]);
    });
    let pattern = target_glob_path.join("*").to_string();
    assert_eq!(target_glob_path.join("file5").exists(), true);
    assert_eq!(target_glob_path.join("file8").is_file(), true);
    assert_eq!(target_glob_path.join("file9").is_file(), true);
    let glob_matches = glob(&pattern)?.iter().map(|path| path.to_string()).collect::<Vec<String>>();
    assert_eq!(
        glob_matches,
        vec![
            "tests/test_walk/test_glob/file5",
            "tests/test_walk/test_glob/file8",
            "tests/test_walk/test_glob/file9"
        ]
    );
    Ok(())
}

#[test]
fn test_walk_dir() -> Result<(), Error> {
    let path = Path::raw("iocore").canonicalize()?;
    assert_eq!(
        walk_dir(&path, NoopProgressHandler, None, None)?
            .iter()
            .filter(|path| !path.is_directory()
                && !path.name().starts_with(".")
                && !path.name().starts_with("#"))
            .map(|entry_path| entry_path.relative_to(&path).to_string())
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
            "fs/errors.rs",
            "fs/filename.rs",
            "fs/ls_path_type.rs",
            "fs/opts.rs",
            "fs/path_cmp.rs",
            "fs/path_status.rs",
            "fs/path_timestamps.rs",
            "fs/path_type.rs",
            "fs/path_utils.rs",
            "fs/perms.rs",
            "fs/size.rs",
            "fs/timed.rs",
            "io/buffer.rs",
            "io/error.rs",
            "io/mod.rs",
            "walk/t.rs",
        ]
    );
    Ok(())
}

#[test]
fn test_walk_dir_fixtures() -> Result<(), Error> {
    let path = folder_path!("fixtures").mkdir_unchecked();
    let entries = walk_dir(&path, NoopProgressHandler, None, None)?;
    assert_eq!(entries.len(), 146);
    Ok(())
}
