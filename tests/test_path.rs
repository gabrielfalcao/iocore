use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::MAIN_SEPARATOR_STR;

use iocore::{Error, Path, PathPermissions, PathStatus, PathType, User, PathDateTime};
use iocore_test::{folder_path, path_to_test_file};
use trilobyte::TriloByte;

#[test]
fn test_path_join() {
    let folder = Path::new("folder");
    assert_eq!(folder.join("a"), Path::new("folder/a"));
    assert_eq!(folder.join("a/b"), Path::new("folder/a/b"));
    assert_eq!(folder.join("a").join("b"), Path::new("folder/a/b"));
    assert_eq!(folder.join("/a"), Path::new("/a"));
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_inexisting_paths() {
    folder_path!("test_relative_to_parent_to_child_no_trailing_slash_both_inexisting_paths/a/b/c")
        .delete()
        .map(|_| false)
        .unwrap_or_else(|_| false);

    assert_eq!(
        Path::raw("test_relative_to_parent_to_child_no_trailing_slash_both_inexisting_paths/a/b/c").relative_to(&Path::raw("test_relative_to_parent_to_child_no_trailing_slash_both_inexisting_paths/a/b/c/x/y/z.bin")).to_string(),
        "../../../"
    );
}

#[test]
fn test_relative_to_parent_to_child_with_trailing_slash_both_inexisting_paths() {
    folder_path!(
        "test_relative_to_parent_to_child_with_trailing_slash_both_inexisting_paths/a/b/c"
    )
    .delete()
    .map(|_| false)
    .unwrap_or_else(|_| false);
    assert_eq!(
        Path::raw("test_relative_to_parent_to_child_with_trailing_slash_both_inexisting_paths/a/b/c/").relative_to(&Path::raw("test_relative_to_parent_to_child_with_trailing_slash_both_inexisting_paths/a/b/c/x/y/z.bin")).to_string(),
        "../../../"
    );
}
#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_existing_paths() {
    let folder_path = folder_path!(
        "test_relative_to_parent_to_child_no_trailing_slash_both_existing_paths/a/b/c"
    )
    .mkdir()
    .unwrap();
    assert_eq!(
        folder_path
            .relative_to(&folder_path.join("x/y/z.bin").write(&[]).unwrap())
            .to_string(),
        "../../../"
    );
    assert_eq!(
        folder_path!("test_relative_to_parent_to_child_no_trailing_slash_both_existing_paths/a/b/c")
            .mkdir()
            .unwrap()
            .relative_to(&path_to_test_file!("test_relative_to_parent_to_child_no_trailing_slash_both_existing_paths/a/b/c/x/y/z.bin").write(&[]).unwrap())
            .to_string(),
        "../../../"
    );
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_non_existing_paths() {
    assert_eq!(
        folder_path!("non-existing/a/b/c")
            .delete()
            .unwrap()
            .relative_to(&path_to_test_file!("non-existing/a/b/c/x/y/z.bin").delete().unwrap())
            .to_string(),
        "../../../"
    );
}

#[test]
fn test_relative_to_child_to_parent() {
    assert_eq!(
        Path::raw("a/b/c/x/y/z.bin").relative_to(&Path::raw("a/b/c")).to_string(),
        "x/y/z.bin"
    );
    assert_eq!(
        Path::raw("a/b/c/x/y/z.bin").relative_to(&Path::raw("a/b/c/")).to_string(),
        "x/y/z.bin"
    );
}

#[test]
fn test_relative_to_cwd() {
    assert_eq!(
        Path::cwd()
            .join("iocore/fs/exceptions.rs")
            .try_canonicalize()
            .relative_to_cwd(),
        Path::new("iocore/fs/exceptions.rs")
    );
    assert_eq!(
        Path::cwd().join("tests/test_path.rs").try_canonicalize().relative_to_cwd(),
        Path::new(file!()).try_canonicalize().relative_to(&Path::cwd()),
    );
}

#[test]
fn test_split_extension() {
    let path = Path::new("/foo/baz.txt");
    assert_eq!(path.split_extension(), ("baz".to_string(), Some("txt".to_string())))
}

#[test]
fn test_join_extension() {
    let path = Path::join_extension("baz".to_string(), Some("txt".to_string()));
    assert_eq!(path, "baz.txt");
}

#[test]
fn test_tildify() {
    let cargo_path = Path::raw(User::id().unwrap().home().unwrap()).join(".cargo");

    assert_eq!(cargo_path.to_string().starts_with("~"), false);
    assert_eq!(cargo_path.tildify().to_string().starts_with("~"), true);
}

#[test]
fn test_path_as_str() {
    let test_path = Path::raw(file!()).relative_to_cwd();

    assert_eq!(test_path.as_str(), "tests/test_path.rs");
}

#[test]
fn test_path_path() {
    let test_path = Path::raw(file!()).relative_to_cwd();
    let mut pathbuf = std::path::PathBuf::new();
    pathbuf.push("tests");
    pathbuf.push("test_path.rs");
    assert_eq!(test_path.path(), pathbuf.as_path());
}

#[test]
fn test_path_contains() {
    let test_path = Path::raw(file!()).relative_to_cwd();
    assert!(test_path.contains("test_path.rs"));
    assert!(test_path.contains("tests/test_path.rs"));
    assert!(test_path.contains("sts/test_path.rs"));
    assert!(test_path.contains("_path.rs"));
}

#[test]
fn test_path_file() {
    let existing_file_path_string =
        path_to_test_file!("test_path_file/file").write(&[]).unwrap().to_string();

    assert!(Path::file(&existing_file_path_string).is_ok());
    Path::file(&existing_file_path_string).unwrap().delete().unwrap();
    assert!(Path::file(&existing_file_path_string).is_err());
}

#[test]
fn test_path_directory() {
    let existing_directory_path_string = folder_path!("test_path_directory").mkdir().unwrap();

    assert!(Path::directory(&existing_directory_path_string).is_ok());
    Path::directory(&existing_directory_path_string).unwrap().delete().unwrap();
    assert!(Path::directory(&existing_directory_path_string).is_err());
}

#[cfg(target_os = "macos")]
#[test]
fn test_path_safe() {
    let path_string = (0..63)
        .map(|_| format!("path"))
        .collect::<Vec<String>>()
        .join(MAIN_SEPARATOR_STR);
    assert_eq!(Path::safe(path_string), Err(Error::FileSystemError(String::from("iocore::fs::Path path too long in \"macos\": \"path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path\""))));
}

#[cfg(target_os = "linux")]
#[test]
fn test_path_safe() {
    let path_string = (0..255)
        .map(|_| format!("path"))
        .collect::<Vec<String>>()
        .join(MAIN_SEPARATOR_STR);
    assert_eq!(Path::safe(path_string),
               Err(Error::FileSystemError(String::from("iocore::fs::Path path too long in \"linux\": \"path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path\""))))
}

#[test]
fn test_path_from_path_buf() {
    let mut pathbuf = std::path::PathBuf::new();
    pathbuf.push("/resolved");
    pathbuf.push("path");

    assert_eq!(Path::from_path_buf(&pathbuf), Path::raw("/resolved/path"));
}

#[test]
fn test_path_from_std_path() {
    let mut pathbuf = std::path::PathBuf::new();
    pathbuf.push("/resolved");
    pathbuf.push("path");
    let std_path = pathbuf.as_path();
    assert_eq!(Path::from_std_path(std_path), Path::raw("/resolved/path"));
}

#[test]
fn test_path_kind() {
    let file = path_to_test_file!("test_path_kind_file").write(&[]).unwrap();
    let folder = folder_path!("test_path_kind_folder").mkdir().unwrap();
    assert_eq!(file.kind(), PathType::File);
    assert_eq!(folder.kind(), PathType::Directory);
}

#[test]
fn test_path_inner_string() {
    assert_eq!(Path::raw("string").inner_string(), String::from("string"));
}

#[test]
fn test_path_str() {
    assert_eq!(Path::raw("&'static str").as_str(), "&'static str");
}

#[test]
fn test_path_() {
    assert_eq!(Path::raw("&'static str").as_str(), "&'static str");
}

#[test]
fn test_path_status() {
    let file = path_to_test_file!("test_path_status_file").write(&[]).unwrap();
    let folder = folder_path!("test_path_status_folder").mkdir().unwrap();
    assert_eq!(file.status(), PathStatus::WritableFile);
    assert_eq!(folder.status(), PathStatus::WritableDirectory);
}

#[test]
fn test_path_create() {
    let path = path_to_test_file!("test_path_create").write(&[]).unwrap();
    let mut created = path.create().unwrap();
    created.write(b"resolved").unwrap();
    assert_eq!(path.read().unwrap(), "resolved");
}

#[test]
fn test_path_append() {
    let path = path_to_test_file!("test_path_append").write(&[]).unwrap();
    let mut append = path.create().unwrap();
    append.write(b"resolved").unwrap();
    path.append(b"\nend").unwrap();
    assert_eq!(path.read().unwrap(), "resolved\nend");
}

#[test]
fn test_path_with_filename() {
    let path = Path::raw("path/with-filename.rs");
    assert_eq!(path.with_filename("with-filename.go"), Path::raw("path/with-filename.go"));
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_parent_exists_child_doesnt() {
    let existing_folder_path = folder_path!(
        "test_relative_to_parent_to_child_no_trailing_slash_parent_exists_child_doesnt/a/b/c"
    )
    .mkdir()
    .unwrap();
    let nonexisting_file_path = existing_folder_path.join("x/y/z.bin").delete().unwrap();
    assert_eq!(
        existing_folder_path.relative_to(&nonexisting_file_path).to_string(),
        "../../../"
    );
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_parent_doesnt_exist_child_exists() {
    let nonexisting_folder_path = folder_path!(
        "test_relative_to_parent_to_child_no_trailing_slash_parent_doesnt_exist_child_exists/a/b/c"
    )
    .delete()
    .unwrap();
    let existing_file_path = nonexisting_folder_path.join("x/y/z.bin").write(&[]).unwrap();
    assert_eq!(
        nonexisting_folder_path.relative_to(&existing_file_path).to_string(),
        "../../../"
    );
}

#[test]
fn test_path_permissions() {
    let file_mode_640 = folder_path!().join("test_mode_640.file");
    let metadata = std::fs::metadata(file_mode_640.path()).unwrap();

    assert_eq!(metadata.mode(), 0o100640);

    assert_eq!(
        PathPermissions::from_u32(metadata.mode()).unwrap(),
        PathPermissions {
            user: TriloByte::from(0b0110),
            group: TriloByte::from(0b100),
            others: TriloByte::from(0b00),
        }
    );
    assert_eq!(
        file_mode_640.permissions(),
        PathPermissions::from_u32(metadata.mode()).unwrap()
    );
    assert_eq!(file_mode_640.mode(), 0o640,);
}

#[test]
fn test_path_timestamps() {
    let mut file_mode_640 = folder_path!().join("test_mode_640.file");
    let created_path_datetime = PathDateTime::parse_from_str("2025-03-18T06:28:30.007453605Z", "%Y-%m-%dT%H:%M:%S.%fZ").unwrap();
    let modified_path_datetime = PathDateTime::parse_from_str("2025-03-18T23:49:43.445802000Z", "%Y-%m-%dT%H:%M:%S.%fZ").unwrap();
    file_mode_640.set_created_time(&created_path_datetime);
    file_mode_640.set_modified_time(&modified_path_datetime);
    let timestamps = file_mode_640.timestamps().unwrap();


    assert_eq!(&timestamps.path, &file_mode_640);
    if std::env::var("TZ").unwrap_or_default() == "UTC" {
        assert_eq!(format!("{}", timestamps.created), "2025-03-18T06:28:30.007453605Z");
        assert_eq!(format!("{}", timestamps.modified), "2025-03-18T23:49:43.445802000Z");
        assert_eq!(
            format!("{:#?}", timestamps.created),
            "PathDateTime[2025-03-18T06:28:30.007453605Z]"
        );
        assert_eq!(
            format!("{:#?}", timestamps.modified),
            "PathDateTime[2025-03-18T23:49:43.445802000Z]"
        );
    } else {
        assert_eq!(format!("{}", timestamps.created), "2025-03-18T03:28:30.007453605-03:00");
        assert_eq!(format!("{}", timestamps.modified), "2025-03-18T20:49:43.445802000-03:00");
        assert_eq!(
            format!("{:#?}", timestamps.created),
            "PathDateTime[2025-03-18T03:28:30.007453605-03:00]"
        );
        assert_eq!(
            format!("{:#?}", timestamps.modified),
            "PathDateTime[2025-03-18T20:49:43.445802000-03:00]"
        );
    }
}
