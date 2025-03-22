use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::fs::timed::PathDateTime;
use crate::{path_datetime_from_metadata_field, Path};
/// `PathTimestamps`
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PathTimestamps {
    pub path: Path,
    pub accessed: PathDateTime,
    pub modified: PathDateTime,
    pub created: PathDateTime,
}

impl PartialOrd for PathTimestamps {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.modified.partial_cmp(&other.modified)
    }
}
impl Ord for PathTimestamps {
    fn cmp(&self, other: &Self) -> Ordering {
        self.modified.cmp(&other.modified)
    }
}

impl PathTimestamps {
    pub fn from_path(path: &Path, metadata: &std::fs::Metadata) -> Result<PathTimestamps, Error> {
        let path = path.clone();
        let accessed: PathDateTime = path_datetime_from_metadata_field!(accessed, metadata, path);
        let modified: PathDateTime = path_datetime_from_metadata_field!(modified, metadata, path);
        let created: PathDateTime = path_datetime_from_metadata_field!(created, metadata, path);

        Ok(PathTimestamps {
            path,
            accessed,
            modified,
            created,
        })
    }

    pub fn fields(&self) -> [(&'static str, PathDateTime); 1] {
        [
            // ("accessed", self.accessed.clone()),
            ("modified", self.modified.clone()),
            // ("created", self.created.clone()),
        ]
    }

    pub fn set_access_time(&mut self, new_access_time: &PathDateTime) -> Result<(), Error> {
        filetime::set_file_atime(self.path.path(), new_access_time.filetime()).map_err(
            |error| {
                Error::FileSystemError(format!(
                    "error setting access time of path {:#?} to {:#?}: {}",
                    self.path.to_string(),
                    new_access_time.to_string(),
                    error
                ))
            },
        )?;
        self.accessed = new_access_time.clone();
        Ok(())
    }

    pub fn set_modified_time(&mut self, new_modified_time: &PathDateTime) -> Result<(), Error> {
        filetime::set_file_mtime(self.path.path(), new_modified_time.filetime()).map_err(
            |error| {
                Error::FileSystemError(format!(
                    "error setting modified time of path {:#?} to {:#?}: {}",
                    self.path.to_string(),
                    new_modified_time.to_string(),
                    error
                ))
            },
        )?;
        self.modified = new_modified_time.clone();
        Ok(())
    }
}

impl std::fmt::Display for PathTimestamps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fields = self
            .fields()
            .iter()
            .map(|(field, timestamp)| {
                format!("{}@{:#?}", field.to_string(), timestamp.to_rfc3339())
            })
            .collect::<Vec<String>>();
        write!(f, "[{}]{:#?}", fields.join("|"), self.path.to_string())
    }
}

impl std::fmt::Debug for PathTimestamps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fields = self
            .fields()
            .iter()
            .map(|(field, timestamp)| {
                format!("{}@{:#?}", field.to_string(), timestamp.to_rfc3339())
            })
            .collect::<Vec<String>>();
        write!(f, "PathTimestamps[{}][{}]", self.path.to_string(), fields.join("|"))
    }
}

#[macro_export]
macro_rules! path_datetime_from_metadata_field {
    ($field:ident, $metadata:ident, $path:ident $(,)?) => {
        Into::<$crate::fs::timed::PathDateTime>::into($metadata.$field().map_err(|error| {
            let io_error = Error::IOError(error.kind()).to_string();
            Error::FileSystemError(format!(
                "error obtaining $field time from metadata {:#?} of path {:#?}: {}",
                $metadata,
                $path.to_string(),
                io_error
            ))
        })?)
    };
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{Path, PathDateTime, PathTimestamps, Result};

    #[test]
    fn test_ordering_same_path() -> Result<()> {
        let old_ts = PathDateTime::parse_from_str("2025-03-18T00:10:20", "%Y-%m-%dT%H:%M:%S")?;

        let old = PathTimestamps {
            path: Path::raw("dummy"),
            modified: old_ts,
            accessed: PathDateTime::parse_from_str("2025-03-20T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
            created: PathDateTime::parse_from_str("2025-03-18T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
        };
        let new_ts = PathDateTime::parse_from_str("2025-03-22T00:10:20", "%Y-%m-%dT%H:%M:%S")?;

        let new = PathTimestamps {
            path: Path::raw("dummy"),
            modified: new_ts,
            accessed: PathDateTime::parse_from_str("2025-03-20T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
            created: PathDateTime::parse_from_str("2025-03-18T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
        };

        assert_eq!(old.cmp(&new), Ordering::Less);
        assert_eq!(new.cmp(&old), Ordering::Greater);
        let mut ts = vec![new.clone(), old.clone()];
        ts.sort();

        assert_eq!(ts, vec![old, new]);
        Ok(())
    }
    #[test]
    fn test_not_equal_same_path_and_different_modified_time() -> Result<()> {
        let old_ts = PathDateTime::parse_from_str("2025-03-18T00:10:20", "%Y-%m-%dT%H:%M:%S")?;

        let old = PathTimestamps {
            path: Path::raw("dummy"),
            modified: old_ts,
            accessed: PathDateTime::parse_from_str("2025-03-20T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
            created: PathDateTime::parse_from_str("2025-03-18T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
        };
        let new_ts = PathDateTime::parse_from_str("2025-03-22T00:10:20", "%Y-%m-%dT%H:%M:%S")?;

        let new = PathTimestamps {
            path: Path::raw("dummy"),
            modified: new_ts,
            accessed: PathDateTime::parse_from_str("2025-03-20T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
            created: PathDateTime::parse_from_str("2025-03-18T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
        };

        assert_ne!(old, new);
        Ok(())
    }
    #[test]
    fn test_equal_same_path_and_same_modified_time() -> Result<()> {
        let ts = PathDateTime::parse_from_str("2025-03-18T00:10:20", "%Y-%m-%dT%H:%M:%S")?;

        let a = PathTimestamps {
            path: Path::raw("dummy"),
            modified: ts.clone(),
            accessed: PathDateTime::parse_from_str("2025-03-20T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
            created: PathDateTime::parse_from_str("2025-03-18T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
        };

        let b = PathTimestamps {
            path: Path::raw("dummy"),
            modified: ts.clone(),
            accessed: PathDateTime::parse_from_str("2025-03-20T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
            created: PathDateTime::parse_from_str("2025-03-18T00:00:00", "%Y-%m-%dT%H:%M:%S")?,
        };

        assert_eq!(a, b);
        Ok(())
    }
}
