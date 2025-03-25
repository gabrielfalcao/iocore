use crate::Error;

/// returns [`std::env::args`] as [`Vec<String>`]
pub fn args() -> Vec<String> {
    std::env::args().map(|c| c.to_string()).collect()
}

/// returns [`std::env::var`] as [`Result<String, Error>`] with either [`String`] or `Error::EnvironmentVarError`
pub fn var(key: impl std::fmt::Display) -> Result<String, Error> {
    let key = key.to_string();
    Ok(std::env::var(&key).map_err(|e| {
        Error::EnvironmentVarError(format!("obtaining environment variable {:#?}: {}", &key, e))
    })?)
}

/// `args_from_string` returns [`Vec<String>`] from split [`String`].
///
/// Example
///
/// ```
/// use iocore::args_from_string;
/// assert_eq!(args_from_string("a b/c  --flag  n/o/p "), vec!["a", "b/c", "--flag", "n/o/p"]);
/// ```
pub fn args_from_string(args: impl std::fmt::Display) -> Vec<String> {
    regex::Regex::new(r"\s+")
        .unwrap()
        .split(&args.to_string())
        .filter(|args| args.len() > 0)
        .map(|args| args.to_string())
        .collect::<Vec<String>>()
}
