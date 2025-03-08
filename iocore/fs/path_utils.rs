use std::path::MAIN_SEPARATOR_STR;

use regex::Regex;

use crate::fs::Path;

pub fn remove_trailing_slash(haystack: &str) -> String {
    let regex = Regex::new(r"/+$").unwrap();
    regex.replace_all(haystack, "").to_string()
}
pub fn expand_home_regex(haystack: &str, expansion: &str) -> String {
    let regex = Regex::new(r"^~").unwrap();
    regex.replace_all(haystack, expansion).to_string()
}
pub fn add_trailing_separator(path: impl std::fmt::Display) -> String {
    let path = path.to_string();
    let path = remove_trailing_slash(&path);
    format!("{}{}", path, MAIN_SEPARATOR_STR)
}
pub fn repl_beg(pattern: &str, haystack: &str, repl: &str) -> String {
    let regex = Regex::new(&format!("^{}", pattern)).unwrap();
    regex.replace_all(haystack, repl).to_string()
}
pub fn repl_end(pattern: &str, haystack: &str, repl: &str) -> String {
    let regex = Regex::new(&format!("{}$", pattern)).unwrap();
    regex.replace_all(haystack, repl).to_string()
}
pub fn remove_end(pattern: &str, haystack: &str) -> String {
    repl_end(pattern, haystack, "")
}
pub fn remove_start(pattern: &str, haystack: &str) -> String {
    repl_beg(&add_trailing_separator(pattern), haystack, "")
}
pub fn remove_duplicate_separators(p: impl std::fmt::Display) -> String {
    let e = Regex::new(&format!("[{}]+", MAIN_SEPARATOR_STR)).unwrap();
    let p = p.to_string();
    e.replace_all(&p, MAIN_SEPARATOR_STR).to_string()
}
pub fn split_str_into_relative_subpath_parts(haystack: &str) -> Vec<String> {
    remove_trailing_slash(haystack)
        .split(MAIN_SEPARATOR_STR)
        .map(|_| "..".to_string())
        .collect::<Vec<String>>()
}
pub fn path_str_to_relative_subpath(haystack: &str) -> String {
    add_trailing_separator(split_str_into_relative_subpath_parts(haystack).join(MAIN_SEPARATOR_STR))
}
pub fn remove_equal_prefix_from_path_strings(path: &str, path2: &str) -> (String, String) {
    let path = remove_trailing_slash(path);
    let path2 = remove_trailing_slash(path2);
    let tmp = remove_trailing_slash(&if path.starts_with(&path2) {
        remove_start(&path, &path2)
    } else {
        remove_start(&path2, &path)
    });
    let path_end = remove_trailing_slash(&if path.starts_with(&tmp) {
        remove_start(&tmp, &path)
    } else {
        String::new()
    });

    let path2_end = remove_trailing_slash(&if path2.starts_with(&tmp) {
        remove_start(&tmp, &path2)
    } else {
        String::new()
    });

    let path_result = if path_end == path { String::new() } else { path_end.to_string() };
    let path2_result = if path2_end == path2 { String::new() } else { path2_end.to_string() };

    // dbg!(&path, &path2);
    // dbg!(&tmp);
    // dbg!(&path_end, &path2_end);
    (path_result, path2_result)
}
// `iocore::fs::path_utils::remove_absolute_path` uses `Path::cwd` to form the absolute path of the given path
pub fn remove_absolute_path(path: &Path) -> Path {
    if path.is_absolute() {
        return path.clone();
    }
    let cwd = Path::cwd();
    let absolute_path = cwd.join(path);
    let absolute_path_string = absolute_path.to_string();
    let absolute_part = absolute_path_string.replace(&path.to_string(), "");
    // dbg!(&absolute_path, &absolute_path_string, &absolute_part);
    return Path::raw(absolute_path_string.replace(&add_trailing_separator(&absolute_part), ""));
}
