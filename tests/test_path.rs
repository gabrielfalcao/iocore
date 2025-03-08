use iocore::{
    expand_home_regex, path_str_to_relative_subpath, remove_duplicate_separators, remove_end,
    remove_equal_prefix_from_path_strings, remove_start, remove_trailing_slash, repl_beg, repl_end,
    split_str_into_relative_subpath_parts, Path, User
};
use iocore_test::{folder_path, path_to_test_file};

#[test]
fn test_remove_start() {
    assert_eq!(remove_start("a/b/c/", "a/b/c/x/y/z.bin"), "x/y/z.bin");
    assert_eq!(repl_beg("a/b/c/", "a/b/c/x/y/z.bin", ""), "x/y/z.bin");
}
#[test]
fn test_remove_end() {
    assert_eq!(remove_end("x/y/z.bin", "a/b/c/x/y/z.bin"), "a/b/c/");
    assert_eq!(repl_end("x/y/z.bin", "a/b/c/x/y/z.bin", ""), "a/b/c/");
}

#[test]
fn test_remove_equal_prefix_from_path_strings() {
    assert_eq!(
        remove_equal_prefix_from_path_strings(
            "/absolute/path/to/a/b/c/x/y/z.bin",
            "/absolute/path/to/a/b/c",
        ),
        ("x/y/z.bin".to_string(), "".to_string()),
    );
    assert_eq!(
        remove_equal_prefix_from_path_strings(
            "/absolute/path/to/a/b/c",
            "/absolute/path/to/a/b/c/x/y/z.bin",
        ),
        ("".to_string(), "x/y/z.bin".to_string())
    );
    assert_eq!(
        remove_equal_prefix_from_path_strings(
            "relative/path/to/a/b/c/x/y/z.bin",
            "relative/path/to/a/b/c",
        ),
        ("x/y/z.bin".to_string(), "".to_string())
    );
    assert_eq!(
        remove_equal_prefix_from_path_strings(
            "relative/path/to/a/b/c",
            "relative/path/to/a/b/c/x/y/z.bin",
        ),
        ("".to_string(), "x/y/z.bin".to_string()),
    );
}
#[test]
fn test_path_join() {
    let folder = Path::new("folder");
    assert_eq!(folder.join("a"), Path::new("folder/a"));
    assert_eq!(folder.join("a/b"), Path::new("folder/a/b"));
    assert_eq!(folder.join("a").join("b"), Path::new("folder/a/b"));
    assert_eq!(folder.join("/a"), Path::new("/a"));
}
#[test]
fn test_remove_trailing_slash() {
    assert_eq!(remove_trailing_slash("a/b/c/"), "a/b/c");
}
#[test]
fn test_remove_duplicate_separators() {
    assert_eq!(remove_duplicate_separators("a//b//c/"), "a/b/c/");
}
#[test]
fn test_expand_home_regex() {
    assert_eq!(expand_home_regex("~baz", "/foo/"), "/foo/baz");
}
#[test]
fn test_split_str_into_relative_subpath_parts() {
    assert_eq!(split_str_into_relative_subpath_parts("a/b/c/"), vec!["..", "..", ".."]);
}
#[test]
fn test_path_str_to_relative_subpath() {
    assert_eq!(path_str_to_relative_subpath("a/b/c"), "../../../");
}

#[test]
fn test_repl_beg() {
    assert_eq!(repl_beg("a/b/c/", "a/b/c/x/y/z.bin", ""), "x/y/z.bin");
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
    let existing_file_path_string = path_to_test_file!("test_path_file/file").write(&[]).unwrap().to_string();

    assert!(Path::file(&existing_file_path_string).is_ok());
    Path::file(&existing_file_path_string).unwrap().delete().unwrap();
    assert!(Path::file(&existing_file_path_string).is_err());
}


// #[test]
// fn test_path_directory() {
//     let existing_directory_path_string = folder_path!("test_path_directory");
//
//     assert!(Path::directory(&existing_directory_path_string).is_ok());
//     Path::directory(&existing_directory_path_string).unwrap().delete().unwrap();
//     assert!(Path::directory(&existing_directory_path_string).is_err());
// }
