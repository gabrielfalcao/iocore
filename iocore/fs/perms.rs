use trilobyte::{high_water_mark_u8_to_trilobyte, TriloByte};

/// `PathPermissions`
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct PathPermissions {
    pub user: TriloByte,
    pub group: TriloByte,
    pub others: TriloByte,
}

impl PathPermissions {
    pub fn to_array(self) -> [u8; 3] {
        [self.user.into_u8(), self.group.into_u8(), self.others.into_u8()]
    }

    pub fn to_string_octal(self) -> String {
        [self.user, self.group, self.others]
            .iter()
            .map(|t| t.to_string_octal())
            .collect::<String>()
    }

    pub fn from_u32(val: u32) -> Result<PathPermissions, crate::Error> {
        let user = TriloByte::from(high_water_mark_u8_to_trilobyte((val >> 6) as u8));
        let group = TriloByte::from(high_water_mark_u8_to_trilobyte((val >> 3) as u8));
        let others = TriloByte::from(high_water_mark_u8_to_trilobyte(val as u8));
        Ok(PathPermissions {
            user,
            group,
            others,
        })
    }

    pub fn from_string_octal(repr: &str) -> Result<PathPermissions, crate::Error> {
        let val = u32::from_str_radix(repr, 8).map_err(|error| {
            crate::Error::FileSystemError(format!("cannot parse u32 from {} base 8: {}", repr, error))
        })?;
        Ok(PathPermissions::from_u32(val)?)
    }
}
impl std::fmt::Display for PathPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string_octal())
    }
}

impl std::fmt::Debug for PathPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PathPermissions[{}]", self.to_string_octal())
    }
}

#[cfg(test)]
mod tests {
    use trilobyte::TriloByte;

    use super::PathPermissions;

    #[test]
    fn test_permissions_to_array() {
        let permissions = PathPermissions {
            user: TriloByte::from(7),
            group: TriloByte::from(5),
            others: TriloByte::from(4),
        };
        assert_eq!(permissions.to_array(), [7, 5, 4]);
    }
    #[test]
    fn test_permissions_to_string_octal() {
        let permissions = PathPermissions {
            user: TriloByte::from(7),
            group: TriloByte::from(5),
            others: TriloByte::from(4),
        };
        assert_eq!(permissions.to_string_octal(), "754");
    }
    #[test]
    fn test_parse_permissions_from_u32() {
        let result = PathPermissions::from_u32(0o754);
        assert_eq!(result.is_ok(), true);
        let permissions = result.unwrap();
        assert_eq!(permissions.to_string_octal(), "754");
    }
    #[test]
    fn test_parse_permissions_from_string_octal() {
        let result = PathPermissions::from_string_octal("754");
        assert_eq!(result.is_ok(), true);
        let permissions = result.unwrap();
        assert_eq!(permissions.to_string_octal(), "754");
    }
    #[test]
    fn test_parse_permissions_from_string_octal_error() {
        let result = PathPermissions::from_string_octal("909");
        assert_eq!(result.is_err(), true);
        assert_eq!(result, Err(crate::Error::FileSystemError(format!("cannot parse u32 from 909 base 8: invalid digit found in string"))));
    }
}
