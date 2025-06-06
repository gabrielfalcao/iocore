use iocore::{glob, walk_dir, walk_globs, Error, NoopProgressHandler, Path, WalkProgressHandler};
use iocore_test::{folder_path, path_to_test_folder};

#[test]
fn test_walk_globs_lib_folder() -> Result<(), Error> {
    assert_eq!(
        walk_globs(vec![format!("iocore/*.rs")], NoopProgressHandler, None)?
            .iter()
            .filter(|path| !path.name().starts_with("."))
            .map(|path| path.name())
            .collect::<Vec<String>>(),
        vec!["env.rs", "errors.rs", "fs.rs", "lib.rs", "sh.rs", "sys.rs", "walk.rs",]
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
            "tests/__test_files__/test_walk/test_walk_globs/file5",
            "tests/__test_files__/test_walk/test_walk_globs/file8",
            "tests/__test_files__/test_walk/test_walk_globs/file9"
        ]
    );
    let matches = walk_globs(vec![pattern], NoopProgressHandler, None)?
        .iter()
        .map(|path| path.name())
        .collect::<Vec<String>>();
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
            "tests/__test_files__/test_walk/test_glob/file5",
            "tests/__test_files__/test_walk/test_glob/file8",
            "tests/__test_files__/test_walk/test_glob/file9"
        ]
    );
    Ok(())
}

#[test]
fn test_walk_dir() -> Result<(), Error> {
    let path = Path::raw("iocore").canonicalize()?;
    let results = walk_dir(&path, NoopProgressHandler, None)?
        .iter()
        .filter(|path| {
            !path.is_directory() && !path.name().starts_with(".") && !path.name().starts_with("#")
        })
        .map(|entry_path| entry_path.relative_to(&path).to_string())
        .collect::<Vec<String>>();

    assert_eq!(
        results,
        vec![
            "env.rs",
            "errors.rs",
            "fs.rs",
            "lib.rs",
            "sh.rs",
            "sys.rs",
            "walk.rs",
            "fs/filename.rs",
            "fs/ls_path_type.rs",
            "fs/opts.rs",
            "fs/path_cmp.rs",
            "fs/path_datetime.rs",
            "fs/path_status.rs",
            "fs/path_timestamps.rs",
            "fs/path_type.rs",
            "fs/path_utils.rs",
            "fs/perms.rs",
            "fs/size.rs",
        ]
    );
    Ok(())
}

#[test]
fn test_walk_dir_no_aggregating_specific_directory() -> Result<(), Error> {
    let path = Path::raw("iocore").canonicalize()?;
    #[derive(Clone, Eq, PartialEq, Debug)]
    struct IgnoreWalkDirectoryHandler;
    impl WalkProgressHandler for IgnoreWalkDirectoryHandler {
        fn path_matching(&mut self, path: &Path) -> Result<bool, Error> {
            Ok(!path.to_string().contains("walk/"))
        }
    }
    let results = walk_dir(&path, IgnoreWalkDirectoryHandler, None)?
        .iter()
        .filter(|path| {
            !path.is_directory() && !path.name().starts_with(".") && !path.name().starts_with("#")
        })
        .map(|entry_path| entry_path.relative_to(&path).to_string())
        .collect::<Vec<String>>();
    assert_eq!(
        results,
        vec![
            "env.rs",
            "errors.rs",
            "fs.rs",
            "lib.rs",
            "sh.rs",
            "sys.rs",
            "walk.rs",
            "fs/filename.rs",
            "fs/ls_path_type.rs",
            "fs/opts.rs",
            "fs/path_cmp.rs",
            "fs/path_datetime.rs",
            "fs/path_status.rs",
            "fs/path_timestamps.rs",
            "fs/path_type.rs",
            "fs/path_utils.rs",
            "fs/perms.rs",
            "fs/size.rs",
        ]
    );
    Ok(())
}

#[test]
fn test_walk_dir_skip_scanning_specific_directories() -> Result<(), Error> {
    let path = Path::raw("iocore").canonicalize()?;
    #[derive(Clone, Eq, PartialEq, Debug)]
    struct SkipWalkDirectoryHandler;
    impl WalkProgressHandler for SkipWalkDirectoryHandler {
        fn path_matching(&mut self, path: &Path) -> Result<bool, Error> {
            Ok(!path.name().starts_with(".") && !path.name().starts_with("#"))
        }

        fn should_scan_directory(&mut self, path: &Path) -> std::result::Result<bool, Error> {
            let skip_directory_names = vec!["io".to_string(), "walk".to_string()];
            Ok(!skip_directory_names.contains(&path.name()))
        }
    }
    let results = walk_dir(&path, SkipWalkDirectoryHandler, None)?
        .iter()
        .filter(|path| !path.is_directory())
        .map(|entry_path| entry_path.relative_to(&path).to_string())
        .collect::<Vec<String>>();
    assert_eq!(
        results,
        vec![
            "env.rs",
            "errors.rs",
            "fs.rs",
            "lib.rs",
            "sh.rs",
            "sys.rs",
            "walk.rs",
            "fs/filename.rs",
            "fs/ls_path_type.rs",
            "fs/opts.rs",
            "fs/path_cmp.rs",
            "fs/path_datetime.rs",
            "fs/path_status.rs",
            "fs/path_timestamps.rs",
            "fs/path_type.rs",
            "fs/path_utils.rs",
            "fs/perms.rs",
            "fs/size.rs",
        ]
    );
    Ok(())
}

#[test]
fn test_walk_dir_error_handling_() -> Result<(), Error> {
    let path = Path::raw("iocore").canonicalize()?;
    #[derive(Clone, Eq, PartialEq, Debug)]
    struct ErrorOnWalkHandler;
    impl WalkProgressHandler for ErrorOnWalkHandler {
        fn path_matching(&mut self, path: &Path) -> Result<bool, Error> {
            Ok(path.exists())
        }

        fn should_scan_directory(&mut self, path: &Path) -> std::result::Result<bool, Error> {
            if path.name() == "fs" {
                return Err(Error::PathScanningError(format!("path shall not be scanned",)));
            }
            Ok(path.is_directory())
        }
    }
    assert_eq!(
        walk_dir(&path, ErrorOnWalkHandler, None),
        Err(Error::WalkDirError("PathScanningError: path shall not be scanned [depth:1] [iocore::walk::iocore_walk_dir:[crates/iocore/iocore/walk.rs:69]]\n".to_string())),
    );
    Ok(())
}

#[test]
fn test_walk_dir_fixtures() -> Result<(), Error> {
    let path = folder_path!("fixtures");
    let entries = walk_dir(&path, NoopProgressHandler, None)?;
    assert_eq!(entries.len(), 146);
    Ok(())
}
