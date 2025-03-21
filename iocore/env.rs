use crate::Error;

/// Returns [`std::env::args`] as [`Vec<String>`]
pub fn args() -> Vec<String> {
    std::env::args().map(|c| c.to_string()).collect()
}

/// Returns [`std::env::var`] as [`Result<String, Error>`] with either [`String`] or `Error::EnvironmentVarError`
pub fn var(key: impl Into<String>) -> Result<String, Error> {
    let key = key.into();
    Ok(std::env::var(&key).map_err(|e| {
        Error::EnvironmentVarError(format!("obtaining environment variable {:#?}: {}", &key, e))
    })?)
}
