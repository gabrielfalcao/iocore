//     /\\\\         /\\       /\\\\     /\\\\\\\    /\\\\\\\\
//   /\\    /\\   /\\   /\\  /\\    /\\  /\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\\    /\\  /\\
// /\\        /\\/\\       /\\        /\\/\ /\\      /\\\\\\
// /\\        /\\/\\       /\\        /\\/\\  /\\    /\\
//   /\\     /\\  /\\   /\\  /\\     /\\ /\\    /\\  /\\
//     /\\\\        /\\\\      /\\\\     /\\      /\\/\\\\\\\\
#![allow(non_snake_case)]

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TriloByteParseError(String, u8);
impl std::error::Error for TriloByteParseError {}
impl std::fmt::Display for TriloByteParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid converstion from {} to TriloByte {}", self.0, self.1)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Debug)]
pub struct TriloByte(bool, bool, bool);
pub fn u8sbin(val: u8) -> String {
    format!("{:b}", val)
}

pub fn hwmtb(val: u8) -> u8 {
    let l2 = if val > 0b11 { val.ilog2() as u8 - 0b10 } else { 0 };
    u8sbin(l2);
    u8sbin(val);
    let val = val.reverse_bits() >> l2;
    u8sbin(val);
    let val = val.reverse_bits() >> l2;
    u8sbin(val);
    val
}

impl TriloByte {
    pub const MAX: u8 = 7;

    pub fn from_u8(N: u8) -> Result<TriloByte, TriloByteParseError> {
        if N > Self::MAX {
            return Err(TriloByteParseError(format!("{} higher than {}", N, Self::MAX), N));
        }
        Ok(TriloByte::from_u8_highwatermark(N))
    }

    pub fn from_u8_highwatermark(val: u8) -> TriloByte {
        let val = hwmtb(val);
        let t = (val >> 0b010 & !0b110) == 0b1;
        let l = (val >> 0b001 & !0b010) == 0b1;
        let b = (val >> 0b000 & !0b110) == 0b1;
        TriloByte(t, l, b)
    }

    pub fn to_array(self) -> [u8; 3] {
        [self.0 as u8, self.1 as u8, self.2 as u8]
    }

    pub fn to_tuple(self) -> (u8, u8, u8) {
        self.to_array().into()
    }

    pub fn into_string(self) -> String {
        self.to_array().iter().map(|b| b.to_string()).collect()
    }
}

impl From<u8> for TriloByte {
    fn from(val: u8) -> TriloByte {
        TriloByte::from_u8_highwatermark(val)
    }
}
impl Into<String> for TriloByte {
    fn into(self) -> String {
        self.into_string()
    }
}

impl std::fmt::Display for TriloByte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.into_string())
    }
}

#[cfg(test)]
mod trilobyte_tests {
    use super::{hwmtb, u8sbin, TriloByte, TriloByteParseError};
    // use std::borrow::Borrow;

    #[test]
    fn test_from_u8() {
        assert_eq!(TriloByte::from_u8(0b111), Ok(TriloByte(true, true, true)));
        assert_eq!(TriloByte::from_u8(0b110), Ok(TriloByte(true, true, false)));
        assert_eq!(TriloByte::from_u8(0b101), Ok(TriloByte(true, false, true)));
        assert_eq!(TriloByte::from_u8(0b100), Ok(TriloByte(true, false, false)));
        assert_eq!(TriloByte::from_u8(0b011), Ok(TriloByte(false, true, true)));
        assert_eq!(TriloByte::from_u8(0b010), Ok(TriloByte(false, true, false)));
        assert_eq!(TriloByte::from_u8(0b001), Ok(TriloByte(false, false, true)));
        assert_eq!(TriloByte::from_u8(0b000), Ok(TriloByte(false, false, false)));
        assert_eq!(
            TriloByte::from_u8(0x008),
            Err(TriloByteParseError("8 higher than 7".to_string(), 0b1000))
        );
    }

    #[test]
    fn test_trait_from_u8() {
        assert_eq!(TriloByte::from_u8(0b111).unwrap(), TriloByte(true, true, true));
        assert_eq!(TriloByte::from_u8(0b110).unwrap(), TriloByte(true, true, false));
        assert_eq!(TriloByte::from_u8(0b101).unwrap(), TriloByte(true, false, true));
        assert_eq!(TriloByte::from_u8(0b100).unwrap(), TriloByte(true, false, false));
        assert_eq!(TriloByte::from_u8(0b011).unwrap(), TriloByte(false, true, true));
        assert_eq!(TriloByte::from_u8(0b010).unwrap(), TriloByte(false, true, false));
        assert_eq!(TriloByte::from_u8(0b001).unwrap(), TriloByte(false, false, true));
        assert_eq!(TriloByte::from_u8(0b000).unwrap(), TriloByte(false, false, false));
    }

    #[test]
    fn test_to_array() {
        assert_eq!(TriloByte(true, true, true).to_array(), [1, 1, 1]);
        assert_eq!(TriloByte(true, true, false).to_array(), [1, 1, 0]);
        assert_eq!(TriloByte(true, false, true).to_array(), [1, 0, 1]);
        assert_eq!(TriloByte(true, false, false).to_array(), [1, 0, 0]);
        assert_eq!(TriloByte(false, true, true).to_array(), [0, 1, 1]);
        assert_eq!(TriloByte(false, true, false).to_array(), [0, 1, 0]);
        assert_eq!(TriloByte(false, false, true).to_array(), [0, 0, 1]);
        assert_eq!(TriloByte(false, false, false).to_array(), [0, 0, 0]);
    }

    #[test]
    fn test_to_tuple() {
        assert_eq!(TriloByte(true, true, true).to_tuple(), (1, 1, 1));
        assert_eq!(TriloByte(true, true, false).to_tuple(), (1, 1, 0));
        assert_eq!(TriloByte(true, false, true).to_tuple(), (1, 0, 1));
        assert_eq!(TriloByte(true, false, false).to_tuple(), (1, 0, 0));
        assert_eq!(TriloByte(false, true, true).to_tuple(), (0, 1, 1));
        assert_eq!(TriloByte(false, true, false).to_tuple(), (0, 1, 0));
        assert_eq!(TriloByte(false, false, true).to_tuple(), (0, 0, 1));
        assert_eq!(TriloByte(false, false, false).to_tuple(), (0, 0, 0));
    }
    #[test]
    fn test_to_string() {
        assert_eq!(TriloByte(true, true, true).to_string(), "111");
        assert_eq!(TriloByte(true, true, false).to_string(), "110");
        assert_eq!(TriloByte(true, false, true).to_string(), "101");
        assert_eq!(TriloByte(true, false, false).to_string(), "100");
        assert_eq!(TriloByte(false, true, true).to_string(), "011");
        assert_eq!(TriloByte(false, true, false).to_string(), "010");
        assert_eq!(TriloByte(false, false, true).to_string(), "001");
        assert_eq!(TriloByte(false, false, false).to_string(), "000");
    }
    #[test]
    fn test_hwmtb() {
        assert_eq!(u8sbin(hwmtb(0b11111001)), u8sbin(0b001));
        assert_eq!(u8sbin(hwmtb(0b11111010)), u8sbin(0b010));
        assert_eq!(u8sbin(hwmtb(0b11111100)), u8sbin(0b100));
        assert_eq!(u8sbin(hwmtb(0b11111101)), u8sbin(0b101));
        assert_eq!(u8sbin(hwmtb(0b11111110)), u8sbin(0b110));
        assert_eq!(u8sbin(hwmtb(0b11111111)), u8sbin(0b111));
    }
    #[test]
    fn test_from_u8_highwatermark() {
        assert_eq!(TriloByte::from_u8_highwatermark(0b001), TriloByte(false, false, true));

        assert_eq!(TriloByte::from_u8_highwatermark(0b11111001), TriloByte(false, false, true));

        assert_eq!(TriloByte::from_u8_highwatermark(0b111), TriloByte(true, true, true));

        assert_eq!(TriloByte::from_u8_highwatermark(0b10000111), TriloByte(true, true, true));
        assert_eq!(TriloByte::from_u8_highwatermark(0b10000000), TriloByte(false, false, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b10000100), TriloByte(true, false, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b10000110), TriloByte(true, true, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b10000111), TriloByte(true, true, true));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11001000), TriloByte(false, false, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11001001), TriloByte(false, false, true));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11001010), TriloByte(false, true, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11001100), TriloByte(true, false, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11011001), TriloByte(false, false, true));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11011010), TriloByte(false, true, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11011100), TriloByte(true, false, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11111001), TriloByte(false, false, true));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11111010), TriloByte(false, true, false));
        assert_eq!(TriloByte::from_u8_highwatermark(0b11111100), TriloByte(true, false, false));
    }
}

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct StatBitsParseError(String, u8);
// impl std::error::Error for StatBitsParseError {}
// impl std::fmt::Display for StatBitsParseError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "invalid converstion from {} to StatBits {}", self.0, self.1)
//     }
// }

// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
// pub struct StatBits {
//     pub user: TriloByte,
//     pub group: TriloByte,
//     pub others: TriloByte,
// }

// impl StatBits {
//     pub const MAX: u32 = 0o777;
//     pub fn from_u8(n: u32) -> Result<StatBits, StatBitsParseError> {
//         if n > Self::MAX {
//             return Err(StatBitsParseError(format!("{} higher than {}", n, Self::MAX), n));
//         }
//     }
// }
// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
// pub struct SpecialStatBits {
//     pub setuid: bool,
//     pub setgid: bool,
//     pub sticky: bool,
// }
// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
// pub struct NodePermissions {
//     pub special: SpecialStatBits,
//     pub user: StatBits,
//     pub group: StatBits,
//     pub other: StatBits,
// }

// impl Serialize for StatBits {
//     fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         todo!()
//     }
// }
// impl<'de> Deserialize<'de> for StatBits {
//     fn deserialize<D>(de: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         todo!()
//     }
// }

// // #[cfg(test)]
// // mod fs_stat_bits {
// //     // use crate::fs::NodePermissions;
// //     // use crate::fs::SpecialStatBits;
// //     use crate::fs::StatBits;
// //     use crate::Error;

// //     #[test]
// //     fn test_stat_bits() -> Result<(), Error> {
// //         Ok(())
// //     }
// // }
