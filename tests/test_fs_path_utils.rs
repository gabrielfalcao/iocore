use iocore::{
    expand_home_regex, path_str_to_relative_subpath, remove_duplicate_separators, remove_end,
    remove_equal_prefix_from_path_strings, remove_redundant_current_path, remove_start,
    remove_trailing_slash, repl_beg, repl_end, split_str_into_relative_subpath_parts,
};

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
fn test_repl_end() {
    assert_eq!(repl_end("x/y/z.bin", "a/b/c/x/y/z.bin", ""), "a/b/c/");
}
#[test]
fn test_remove_redundant_current_path() {
    assert_eq!(remove_redundant_current_path("a/./b/./c/"), "a/b/c/");
}
