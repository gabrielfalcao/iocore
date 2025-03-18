use trilobyte::TriloByte;

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
}
