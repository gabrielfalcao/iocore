use std::path::MAIN_SEPARATOR_STR;

use crate::errors::{Error, Result};
use crate::Path;

pub struct FileName {
    value: String,
}

impl FileName {
    pub fn new(name: impl Into<String>) -> Result<FileName> {
        let value: String = value.into();
        if value.contains(MAIN_SEPARATOR_STR) {
            Err(Error::MalformedFileName(format!(
                "FileName contains path separator {:#?}: {:#?}",
                MAIN_SEPARATOR_STR, &value
            )))
        } else {
            FileName { value }
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.value.contains(MAIN_SEPARATOR_STR) {
            Err(Error::MalformedFileName(format!(
                "FileName contains path separator {:#?}: {:#?}",
                MAIN_SEPARATOR_STR, &self.value
            )))
        } else {
            Ok(())
        }
    }

    pub fn at_path(&self, path: &Path) -> Result<Path> {
        self.validate()?;
        if !path.exists() || path.is_dir() {
            Ok(path.join(&self.value))
        } else {
            Err(Error::PathConversionError(format!(
                "in joining {:#?}: {:#?} exists and is not a diretory",
                &self, path
            )))
        }
    }
}

impl std::fmt::Display for FileName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.validate()?;
        write!(f, "{}", &self.value)
    }
}

impl std::fmt::Debug for FileName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}::FileName({:#?})", module_path!(), &self.value)
    }
}
