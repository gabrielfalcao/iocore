use trilobyte::{high_water_mark_u8_to_trilobyte, TriloByte};

use crate::traceback;
/// `PathPermissions`
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Default)]
pub struct PathPermissions {
    pub user: TriloByte,
    pub group: TriloByte,
    pub other: TriloByte,
}
impl PathPermissions {
    pub fn to_array(self) -> [u8; 3] {
        [self.user.into_u8(), self.group.into_u8(), self.other.into_u8()]
    }

    pub fn to_string_octal(self) -> String {
        [self.user, self.group, self.other]
            .iter()
            .map(|t| t.to_string_octal())
            .collect::<String>()
    }

    pub fn into_u32(self) -> u32 {
        u32::from_str_radix(&self.to_string_octal(), 8).unwrap()
    }

    pub fn from_u32(val: u32) -> Result<PathPermissions, crate::Error> {
        let user = TriloByte::from(high_water_mark_u8_to_trilobyte((val >> 6) as u8));
        let group = TriloByte::from(high_water_mark_u8_to_trilobyte((val >> 3) as u8));
        let other = TriloByte::from(high_water_mark_u8_to_trilobyte(val as u8));
        Ok(PathPermissions { user, group, other })
    }

    pub fn from_string_octal(repr: &str) -> Result<PathPermissions, crate::Error> {
        let val = u32::from_str_radix(repr, 8).map_err(|error| {
            traceback!(FileSystemError, "cannot parse u32 from {} base 8: {}", repr, error)
        })?;
        Ok(PathPermissions::from_u32(val)?)
    }

    pub fn user(&self) -> Permission {
        Permission(self.user)
    }

    pub fn group(&self) -> Permission {
        Permission(self.group)
    }

    pub fn other(&self) -> Permission {
        Permission(self.other)
    }

    pub fn set_user(&mut self, readable: bool, writable: bool, executable: bool) {
        self.user = TriloByte(readable, writable, executable);
    }

    pub fn set_group(&mut self, readable: bool, writable: bool, executable: bool) {
        self.group = TriloByte(readable, writable, executable);
    }

    pub fn set_other(&mut self, readable: bool, writable: bool, executable: bool) {
        self.other = TriloByte(readable, writable, executable);
    }

    pub fn set_user_readable(&mut self, readable: bool) {
        self.user.0 = readable;
    }

    pub fn set_user_writable(&mut self, writable: bool) {
        self.user.1 = writable;
    }

    pub fn set_user_executable(&mut self, executable: bool) {
        self.user.2 = executable;
    }

    pub fn set_group_readable(&mut self, readable: bool) {
        self.group.0 = readable;
    }

    pub fn set_group_writable(&mut self, writable: bool) {
        self.group.1 = writable;
    }

    pub fn set_group_executable(&mut self, executable: bool) {
        self.group.2 = executable;
    }

    pub fn set_other_readable(&mut self, readable: bool) {
        self.other.0 = readable;
    }

    pub fn set_other_writable(&mut self, writable: bool) {
        self.other.1 = writable;
    }

    pub fn set_other_executable(&mut self, executable: bool) {
        self.other.2 = executable;
    }

    pub fn executable(&self) -> bool {
        self.user().executable() || self.group().executable()
    }

    pub fn readable(&self) -> bool {
        self.user().readable() || self.group().readable()
    }

    pub fn writable(&self) -> bool {
        self.user().writable() || self.group().writable()
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
impl Into<u32> for PathPermissions {
    fn into(self) -> u32 {
        self.into_u32()
    }
}
#[cfg(test)]
mod tests_path_permissions {
    use trilobyte::TriloByte;

    use crate::{Error, PathPermissions};

    #[test]
    fn test_permissions_to_array() {
        let permissions = PathPermissions {
            user: TriloByte::from(7),
            group: TriloByte::from(5),
            other: TriloByte::from(4),
        };
        assert_eq!(permissions.to_array(), [7, 5, 4]);
    }
    #[test]
    fn test_permissions_to_string_octal() {
        let permissions = PathPermissions {
            user: TriloByte::from(7),
            group: TriloByte::from(5),
            other: TriloByte::from(4),
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
        assert_eq!(
            result,
            Err(Error::FileSystemError("cannot parse u32 from 909 base 8: invalid digit found in string [iocore::fs::perms::PathPermissions::from_string_octal::{{closure}}:[crates/iocore/iocore/fs/perms.rs:36]]\n".to_string()))
        );
    }
    #[test]
    fn test_permissions_into_u32() {
        let permissions = PathPermissions {
            user: TriloByte::from(7),
            group: TriloByte::from(5),
            other: TriloByte::from(4),
        };
        assert_eq!(permissions.into_u32(), 0o754);
    }

    #[test]
    fn test_permissions_set() {
        let mut permissions = PathPermissions::default();
        permissions.set_user(true, true, true);
        permissions.set_group(true, false, true);
        permissions.set_other(true, false, false);
        assert_eq!(permissions.to_string(), "754");
        permissions.set_user_executable(false);
        assert_eq!(permissions.to_string(), "654");
        permissions.set_user_readable(false);
        assert_eq!(permissions.to_string(), "254");
        permissions.set_group_executable(false);
        assert_eq!(permissions.to_string(), "244");
        permissions.set_group_readable(false);
        assert_eq!(permissions.to_string(), "204");
        permissions.set_other_readable(false);
        assert_eq!(permissions.to_string(), "200");
    }
}

pub struct Permission(TriloByte);
impl Permission {
    pub fn readable(&self) -> bool {
        self.0 .0 == true
    }

    pub fn writable(&self) -> bool {
        self.0 .1 == true
    }

    pub fn executable(&self) -> bool {
        self.0 .2 == true
    }
}

#[cfg(test)]
mod tests_permission {
    use trilobyte::TriloByte;

    use super::Permission;

    #[test]
    fn test_permission_readable() {
        assert_eq!(Permission(TriloByte(false, false, false)).readable(), false);
        assert_eq!(Permission(TriloByte(true, false, false)).readable(), true);
    }
    #[test]
    fn test_permission_writable() {
        assert_eq!(Permission(TriloByte(false, false, false)).writable(), false);
        assert_eq!(Permission(TriloByte(false, true, false)).writable(), true);
    }
    #[test]
    fn test_permission_executable() {
        assert_eq!(Permission(TriloByte(false, false, false)).executable(), false);
        assert_eq!(Permission(TriloByte(false, false, true)).executable(), true);
    }
}
