use serde::{Deserialize, Serialize};

use crate::errors::Error;
use crate::fs::timed::PathDateTime;
use crate::Path;
/// `PathTimestamps`
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PathTimestamps {
    pub path: Path,
    pub accessed: PathDateTime,
    pub modified: PathDateTime,
    pub created: PathDateTime,
}

impl PathTimestamps {
    pub fn from_path(path: &Path, metadata: &std::fs::Metadata) -> Result<PathTimestamps, Error> {
        let path = path.clone();
        let accessed: PathDateTime =
            Into::<PathDateTime>::into(metadata.accessed().map_err(|error| {
                let io_error = Error::IOError(error.kind()).to_string();
                Error::FileSystemError(format!(
                    "error obtaining access time from metadata {:#?} of path {:#?}: {}",
                    &metadata,
                    path.to_string(),
                    io_error
                ))
            })?);
        let modified: PathDateTime =
            Into::<PathDateTime>::into(metadata.modified().map_err(|error| {
                let io_error = Error::IOError(error.kind()).to_string();
                Error::FileSystemError(format!(
                    "error obtaining access time from metadata {:#?} of path {:#?}: {}",
                    &metadata,
                    path.to_string(),
                    io_error
                ))
            })?);
        let created: PathDateTime =
            Into::<PathDateTime>::into(metadata.created().map_err(|error| {
                let io_error = Error::IOError(error.kind()).to_string();
                Error::FileSystemError(format!(
                    "error obtaining access time from metadata {:#?} of path {:#?}: {}",
                    &metadata,
                    path.to_string(),
                    io_error
                ))
            })?);

        Ok(PathTimestamps {
            path,
            accessed,
            modified,
            created,
        })
    }

    pub fn fields(&self) -> [(&'static str, PathDateTime); 3] {
        [
            ("accessed", self.accessed.clone()),
            ("modified", self.modified.clone()),
            ("created", self.created.clone()),
        ]
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
        $crate::fs::timed::PathDateTime =
            Into::<$crate::fs::timed::PathDateTime>::into(metadata.$field().map_err(|error| {
                let io_error = Error::IOError(error.kind()).to_string();
                Error::FileSystemError(format!(
                    "error obtaining $field time from metadata {:#?} of path {:#?}: {}",
                    &$metadata,
                    $path.to_string(),
                    io_error
                ))
            })?);
    };
}
