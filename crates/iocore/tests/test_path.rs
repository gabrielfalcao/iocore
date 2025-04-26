use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::MAIN_SEPARATOR_STR;

use iocore::{Error, Path, PathDateTime, PathPermissions, PathStatus, PathType, Result};
use iocore_test::{
    folder_path, path_to_test_directory, path_to_test_file, path_to_test_folder, seq_bytes, current_source_file
};
use trilobyte::TriloByte;

#[test]
fn test_path_join() -> Result<()> {
    let folder = Path::new("folder");
    assert_eq!(folder.join("a"), Path::new("folder/a"));
    assert_eq!(folder.join("a/b"), Path::new("folder/a/b"));
    assert_eq!(folder.join("a").join("b"), Path::new("folder/a/b"));
    assert_eq!(folder.join("/a"), Path::new("/a"));
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_non_existing_paths() -> Result<()> {
    assert_eq!(
        path_to_test_file!("a/b/c/x/y/z.bin")
            .relative_to(&path_to_test_folder!("a/b/c"))
            .to_string(),
        "x/y/z.bin"
    );
    Ok(())
}

#[test]
fn test_relative_to_child_to_parent() -> Result<()> {
    assert_eq!(
        Path::raw("a/b/c/x/y/z.bin").relative_to(&Path::raw("a/b/c")).to_string(),
        "x/y/z.bin"
    );
    assert_eq!(
        Path::raw("a/b/c/x/y/z.bin").relative_to(&Path::raw("a/b/c/")).to_string(),
        "x/y/z.bin"
    );
    Ok(())
}

#[test]
fn test_relative_to_cwd() -> Result<()> {
    assert_eq!(
        Path::cwd().join("iocore/fs/exceptions.rs").relative_to_cwd(),
        Path::raw("iocore/fs/exceptions.rs")
    );
    let curr_file = file!().to_string();
    let curr_cwd = Path::cwd().to_string();
    let current_source_file_ = current_source_file!();
    let module_path_ = module_path!().to_string();
    dbg!(&module_path_, &curr_file, &curr_cwd, &current_source_file_);
    assert_eq!(Path::raw(current_source_file!()).relative_to_cwd(), Path::raw("tests/test_path.rs"));
    Ok(())
}

#[test]
fn test_split_extension() -> Result<()> {
    let path = Path::new("/foo/baz.txt");
    assert_eq!(path.split_extension(), ("baz".to_string(), Some("txt".to_string())));
    Ok(())
}

#[test]
fn test_join_extension() -> Result<()> {
    let path = Path::join_extension("baz".to_string(), Some("txt".to_string()));
    assert_eq!(path, "baz.txt");
    Ok(())
}

#[test]
fn test_tildify() -> Result<()> {
    let cargo_path = Path::raw(iocore::USER.home()?).join(".cargo");

    assert_eq!(cargo_path.to_string().starts_with("~"), false);
    assert_eq!(cargo_path.tildify().to_string(), "~/.cargo");

    assert_eq!(Path::new(cargo_path.tildify().to_string()).to_string(), cargo_path.to_string());
    Ok(())
}

#[test]
fn test_path_path() -> Result<()> {
    let test_path = Path::raw(current_source_file!()).relative_to_cwd();
    let mut pathbuf = std::path::PathBuf::new();

    pathbuf.push("tests");
    pathbuf.push("test_path.rs");
    assert_eq!(test_path.path(), pathbuf.as_path());
    Ok(())
}

#[test]
fn test_path_contains() -> Result<()> {
    let test_path = Path::raw(current_source_file!());
    assert!(test_path.contains("test_path.rs"));
    assert!(test_path.contains("tests/test_path.rs"));
    assert!(test_path.contains("sts/test_path.rs"));
    assert!(test_path.contains("_path.rs"));
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn test_path_safe() -> Result<()> {
    let path_string = (0..63)
        .map(|_| format!("path"))
        .collect::<Vec<String>>()
        .join(MAIN_SEPARATOR_STR);
    assert_eq!(Path::safe(path_string), Err(Error::FileSystemError(String::from("iocore::fs::Path path too long in \"macos\": \"path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path\""))));
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn test_path_safe() -> Result<()> {
    let path_string = (0..255)
        .map(|_| format!("path"))
        .collect::<Vec<String>>()
        .join(MAIN_SEPARATOR_STR);
    assert_eq!(Path::safe(path_string),
               Err(Error::FileSystemError(String::from("iocore::fs::Path path too long in \"linux\": \"path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path/path\""))));
    Ok(())
}

#[test]
fn test_path_from_path_buf() -> Result<()> {
    let mut pathbuf = std::path::PathBuf::new();
    pathbuf.push("/resolved");
    pathbuf.push("path");

    assert_eq!(Path::from_path_buf(&pathbuf), Path::raw("/resolved/path"));
    Ok(())
}

#[test]
fn test_path_from_std_path() -> Result<()> {
    let mut pathbuf = std::path::PathBuf::new();
    pathbuf.push("/resolved");
    pathbuf.push("path");
    let std_path = pathbuf.as_path();
    assert_eq!(Path::from_std_path(std_path), Path::raw("/resolved/path"));
    Ok(())
}

#[test]
fn test_path_inner_string() -> Result<()> {
    assert_eq!(Path::raw("string").inner_string(), String::from("string"));
    Ok(())
}

#[test]
fn test_path_with_filename() -> Result<()> {
    let path = Path::raw("path/with-filename.rs");
    assert_eq!(path.with_filename("with-filename.go"), Path::raw("path/with-filename.go"));
    Ok(())
}

#[test]
fn test_path_status() -> Result<()> {
    let file = path_to_test_file!("test_path_status_file").write(&[])?;
    let folder = folder_path!("test_path_status_folder").mkdir()?;
    assert_eq!(file.status(), PathStatus::WritableFile);
    assert_eq!(folder.status(), PathStatus::WritableDirectory);
    Ok(())
}

#[test]
fn test_path_create() -> Result<()> {
    let path = path_to_test_file!("test_path_create").write(&[])?;
    let mut created = path.create()?;
    created.write(b"resolved")?;
    assert_eq!(path.read()?, "resolved");
    Ok(())
}

#[test]
fn test_path_append() -> Result<()> {
    let path = path_to_test_file!("test_path_append").write(&[])?;
    let mut append = path.create()?;
    append.write(b"resolved")?;
    path.append(b"\nend")?;
    assert_eq!(path.read()?, "resolved\nend");
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_parent_exists_child_doesnt() -> Result<()> {
    let existing_folder_path = path_to_test_folder!("a/b/c").mkdir()?;
    let nonexisting_file_path = existing_folder_path.join("x/y/z.bin").delete()?;
    assert_eq!(
        existing_folder_path.relative_to(&nonexisting_file_path).to_string(),
        "../../../"
    );
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_parent_doesnt_exist_child_exists(
) -> Result<()> {
    let nonexisting_folder_path = folder_path!(
        "test_relative_to_parent_to_child_no_trailing_slash_parent_doesnt_exist_child_exists/a/b/c"
    )
    .delete()?;
    let existing_file_path = nonexisting_folder_path.join("x/y/z.bin").write(&[])?;
    assert_eq!(
        nonexisting_folder_path.relative_to(&existing_file_path).to_string(),
        "../../../"
    );
    Ok(())
}

#[test]
fn test_path_timestamps() -> Result<()> {
    let modified_path_datetime =
        PathDateTime::parse_from_str("2025-03-18T23:49:43.445802000Z", "%Y-%m-%dT%H:%M:%S.%fZ")?;
    let file_mode_640 = path_to_test_file!("test_path_timestamps.640")
        .write(&[])?
        .set_mode(0o640)?
        .set_modified_time(&modified_path_datetime)?;
    let timestamps = file_mode_640.timestamps()?;

    assert_eq!(&timestamps.path, &file_mode_640);
    if std::env::var("TZ").unwrap_or_default() == "UTC" {
        assert_eq!(format!("{}", timestamps.modified), "2025-03-18T23:49:43.445802000Z");
        assert_eq!(
            format!("{:#?}", timestamps.modified),
            "PathDateTime[2025-03-18T23:49:43.445802000Z]"
        );
    } else {
        assert_eq!(format!("{}", timestamps.modified), "2025-03-18T20:49:43.445802000-03:00");
        assert_eq!(
            format!("{:#?}", timestamps.modified),
            "PathDateTime[2025-03-18T20:49:43.445802000-03:00]"
        );
    }
    Ok(())
}

#[test]
fn test_path_timestamps_accessed() -> Result<()> {
    let file = path_to_test_file!("test_path_timestamps_accessed").write(&[])?;
    let timestamps = file.timestamps()?;

    assert_eq!(file.accessed(), Some(timestamps.accessed));
    Ok(())
}
#[test]
fn test_path_timestamps_created() -> Result<()> {
    let file = path_to_test_file!("test_path_timestamps_created").write(&[])?;
    let timestamps = file.timestamps()?;

    assert_eq!(file.created(), Some(timestamps.created));
    Ok(())
}
#[test]
fn test_path_timestamps_modified() -> Result<()> {
    let file = path_to_test_file!("test_path_timestamps_modified").write(&[])?;
    let timestamps = file.timestamps()?;

    assert_eq!(file.modified(), Some(timestamps.modified));
    Ok(())
}

#[test]
fn test_path_ordering() -> Result<()> {
    let mut paths = vec![
        folder_path!("test_path_ordering/a").mkdir()?,
        path_to_test_file!("test_path_ordering/a/a").write(&[])?,
        path_to_test_file!("test_path_ordering/a/b").write(&[])?,
        path_to_test_file!("test_path_ordering/a/c").write(&[])?,
        path_to_test_file!("test_path_ordering/a/d").write(&[])?,
        folder_path!("test_path_ordering/b").mkdir()?,
        path_to_test_file!("test_path_ordering/b/a").write(&[])?,
        path_to_test_file!("test_path_ordering/b/b").write(&[])?,
        path_to_test_file!("test_path_ordering/b/c").write(&[])?,
        path_to_test_file!("test_path_ordering/b/d").write(&[])?,
        folder_path!("test_path_ordering/c").mkdir()?,
        path_to_test_file!("test_path_ordering/c/a").write(&[])?,
        path_to_test_file!("test_path_ordering/c/b").write(&[])?,
        path_to_test_file!("test_path_ordering/c/c").write(&[])?,
        path_to_test_file!("test_path_ordering/c/d").write(&[])?,
        folder_path!("test_path_ordering/d").mkdir()?,
        path_to_test_file!("test_path_ordering/d/a").write(&[])?,
        path_to_test_file!("test_path_ordering/d/b").write(&[])?,
        path_to_test_file!("test_path_ordering/d/c").write(&[])?,
        path_to_test_file!("test_path_ordering/d/d").write(&[])?,
    ];
    paths.sort();

    assert_eq!(
        paths,
        vec![
            folder_path!("test_path_ordering/a"),
            folder_path!("test_path_ordering/b"),
            folder_path!("test_path_ordering/c"),
            folder_path!("test_path_ordering/d"),
            path_to_test_file!("test_path_ordering/a/a"),
            path_to_test_file!("test_path_ordering/a/b"),
            path_to_test_file!("test_path_ordering/a/c"),
            path_to_test_file!("test_path_ordering/a/d"),
            path_to_test_file!("test_path_ordering/b/a"),
            path_to_test_file!("test_path_ordering/b/b"),
            path_to_test_file!("test_path_ordering/b/c"),
            path_to_test_file!("test_path_ordering/b/d"),
            path_to_test_file!("test_path_ordering/c/a"),
            path_to_test_file!("test_path_ordering/c/b"),
            path_to_test_file!("test_path_ordering/c/c"),
            path_to_test_file!("test_path_ordering/c/d"),
            path_to_test_file!("test_path_ordering/d/a"),
            path_to_test_file!("test_path_ordering/d/b"),
            path_to_test_file!("test_path_ordering/d/c"),
            path_to_test_file!("test_path_ordering/d/d"),
        ]
    );

    Ok(())
}

#[test]
fn test_path_size() -> Result<()> {
    let path_a = path_to_test_file!("test_path_size/a").write(&seq_bytes(104))?;
    assert_eq!(path_a.size()?.as_u64(), 104);
    assert_eq!(path_a.size()?.to_string(), "104B");

    let path_b = path_to_test_file!("test_path_size/b").write(&seq_bytes(4096))?;
    assert_eq!(path_b.size()?.as_u64(), 4096);
    assert_eq!(path_b.size()?.to_string(), "4Kb");

    let path_c = path_to_test_file!("test_path_size/c").write(&seq_bytes(4194304))?;
    assert_eq!(path_c.size()?.to_string(), "4Mb");
    assert_eq!(path_c.size()?.as_u64(), 4194304);

    let mut sizes = vec![path_b.size()?, path_c.size()?, path_a.size()?];
    sizes.sort();
    assert_eq!(sizes, vec![path_a.size()?, path_b.size()?, path_c.size()?]);
    Ok(())
}
#[test]
fn test_relative_to_parent_to_child_with_trailing_slash_both_inexisting_paths() -> Result<()> {
    let test_folder = path_to_test_folder!("a/b/c").delete_unchecked();
    let test_file = path_to_test_file!("a/b/c/x/y/z.bin").delete_unchecked();
    assert_eq!(test_folder.relative_to(&test_file).to_string(), "../../../");
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_existing_paths() -> Result<()> {
    let folder_path = path_to_test_folder!("a/b/c");
    assert_eq!(
        folder_path
            .relative_to(&path_to_test_file!("a/b/c/x/y/z.bin").write_unchecked(&[]))
            .to_string(),
        "../../../"
    );
    assert_eq!(
        path_to_test_folder!("a/b/c")
            .mkdir()?
            .relative_to(&path_to_test_file!("a/b/c/x/y/z.bin").write_unchecked(&[]))
            .to_string(),
        "../../../"
    );
    Ok(())
}

#[test]
fn test_path_file() -> Result<()> {
    let existing_file_path_string = path_to_test_file!("file").write_unchecked(&[]).to_string();

    assert!(Path::file(&existing_file_path_string).is_ok());
    Path::file(&existing_file_path_string)?.delete()?;
    assert!(Path::file(&existing_file_path_string).is_err());
    Ok(())
}

#[test]
fn test_path_directory() -> Result<()> {
    let existing_directory_path_string = path_to_test_folder!("folder").to_string();
    let absolute_path_to_existing_directory_path =
        Path::raw(&existing_directory_path_string).canonicalize()?;

    assert_eq!(
        Path::directory(&existing_directory_path_string),
        Ok(Path::new(&existing_directory_path_string))
    );
    Path::directory(&existing_directory_path_string)?.delete()?;
    assert_eq!(
        Path::directory(&Path::raw(existing_directory_path_string)),
        Err(Error::UnexpectedPathType(
            absolute_path_to_existing_directory_path,
            PathType::None
        ))
    );

    Ok(())
}

#[test]
fn test_path_kind() -> Result<()> {
    let file = path_to_test_file!("test_path_kind_file").write_unchecked(&[]);
    let folder = folder_path!("test_path_kind_folder").mkdir_unchecked();
    assert_eq!(file.kind(), PathType::File);
    assert_eq!(folder.kind(), PathType::Directory);
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_inexisting_paths() -> Result<()> {
    let test_folder = path_to_test_folder!("a/b/c");
    test_folder.delete_unchecked();
    let test_file = test_folder.join("x/y/z.bin");
    test_file.delete_unchecked();

    assert_eq!(test_folder.relative_to(&test_file).to_string(), "../../../");
    Ok(())
}

#[test]
fn test_file() -> Result<()> {
    let test_file = path_to_test_file!("a/b/c").write_unchecked(&[]);
    assert_eq!(Path::file(test_file.to_string()), Ok(test_file));
    Ok(())
}

#[test]
fn test_writable_file() -> Result<()> {
    let test_file = path_to_test_file!("a/b/c").write_unchecked(&[]);
    assert_eq!(Path::writable_file(test_file.to_string()), Ok(test_file));
    Ok(())
}

#[test]
fn test_directory() -> Result<()> {
    let test_directory = path_to_test_directory!("a/b/c").mkdir()?;
    assert_eq!(Path::directory(test_directory.to_string()), Ok(test_directory));
    Ok(())
}

#[test]
fn test_writable_directory() -> Result<()> {
    let test_directory = path_to_test_directory!("a/b/c").mkdir()?;
    assert_eq!(Path::writable_directory(test_directory.to_string()), Ok(test_directory));
    Ok(())
}

#[test]
fn test_path_tmp_file() -> Result<()> {
    let tmp = Path::tmp_file();
    assert_eq!(tmp.exists(), true);
    assert_eq!(tmp.is_file(), true);
    Ok(())
}

#[test]
fn test_path_tmp() -> Result<()> {
    let tmp = Path::tmp();
    assert_eq!(tmp.exists(), true);
    assert_eq!(tmp.is_directory(), true);
    Ok(())
}

#[test]
fn test_path_canonicalize() -> Result<()> {
    assert_eq!(
        Path::raw("~").canonicalize()?,
        Path::raw(iocore::USERS_PATH).join(&iocore::User::id()?.name)
    );
    Ok(())
}


#[test]
fn test_path_permissions() -> Result<()> {
    let file_mode_640 =
        path_to_test_file!("test_path_permissions.640").write(&[])?.set_mode(0o640)?;
    let metadata = std::fs::metadata(file_mode_640.path())?;

    assert_eq!(format!("{:o}", metadata.mode()), "100640");
    assert_eq!(
        PathPermissions::from_u32(metadata.mode())?,
        PathPermissions {
            user: TriloByte::from(0b0110),
            group: TriloByte::from(0b100),
            other: TriloByte::from(0b00),
        }
    );

    assert_eq!(file_mode_640.mode(), 0o640);
    assert_eq!(file_mode_640.permissions(), PathPermissions::from_u32(metadata.mode())?);

    assert_eq!(file_mode_640.readable(), true);
    assert_eq!(file_mode_640.writable(), true);
    assert_eq!(file_mode_640.executable(), false);
    assert_eq!(file_mode_640.permissions().readable(), true);
    assert_eq!(file_mode_640.permissions().writable(), true);
    assert_eq!(file_mode_640.permissions().executable(), false);

    assert_eq!(file_mode_640.permissions().user().writable(), true);
    assert_eq!(file_mode_640.permissions().user().readable(), true);
    assert_eq!(file_mode_640.permissions().user().executable(), false);

    assert_eq!(file_mode_640.permissions().group().writable(), false);
    assert_eq!(file_mode_640.permissions().group().readable(), true);
    assert_eq!(file_mode_640.permissions().group().executable(), false);

    assert_eq!(file_mode_640.permissions().other().writable(), false);
    assert_eq!(file_mode_640.permissions().other().readable(), false);
    assert_eq!(file_mode_640.permissions().other().executable(), false);
    Ok(())
}

#[test]
fn test_path_set_mode() -> Result<()> {
    let mut file = Path::tmp_file();
    file.set_mode(0o755)?;
    assert_eq!(format!("{:o}", file.mode()), "755");
    Ok(())
}
#[test]
fn test_path_set_permissions() -> Result<()> {
    let mut file = Path::tmp_file();

    file.set_permissions(&PathPermissions::from_u32(0o777)?)?;
    assert_eq!(format!("{:o}", file.mode()), "777");
    Ok(())
}
