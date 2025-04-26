use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ByteUnit {
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
}

impl ByteUnit {
    pub fn byte(self) -> Size {
        1.into()
    }

    fn kilo(self) -> Size {
        1024.into()
    }

    pub fn scale(self) -> u32 {
        use ByteUnit::*;
        match self {
            Byte => 1,
            Kilo => 2,
            Mega => 3,
            Giga => 4,
            Tera => 5,
            Peta => 6,
        }
    }

    pub fn as_str(self) -> &'static str {
        use ByteUnit::*;
        match self {
            Byte => "B",
            Kilo => "Kb",
            Mega => "Mb",
            Giga => "Gb",
            Tera => "Tb",
            Peta => "Pb",
        }
    }

    pub fn size(self) -> Size {
        self.kilo().exp(self.scale() - 1)
    }

    pub fn as_u64(self) -> u64 {
        self.size().as_u64()
    }

    pub fn variants() -> Vec<ByteUnit> {
        use ByteUnit::*;
        vec![Byte, Kilo, Mega, Giga, Tera, Peta]
    }

    pub fn fit(size: Size) -> (ByteUnit, Option<u64>) {
        let log = size.as_u64().ilog(1024);
        let variant = ByteUnit::variants()[log as usize];
        let remainder = size.as_u64() % variant.as_u64();
        (variant, if remainder > 0 { Some(remainder) } else { None })
    }

    pub fn fmt(size: Size) -> String {
        let (unit, remainder) = ByteUnit::fit(size);
        format!(
            "{}{}{}",
            size.as_u64() / unit.as_u64(),
            match remainder {
                Some(remainder) => format!(".{:1}", remainder),
                None => String::new(),
            },
            unit.as_str()
        )
    }
}

#[derive(Clone, Debug, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    bytes: u64,
}
impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", ByteUnit::fmt(*self))
    }
}

impl Default for Size {
    fn default() -> Size {
        let bytes = 0;
        Size { bytes }
    }
}
impl Size {
    pub fn as_u64(self) -> u64 {
        self.bytes
    }

    pub fn unit(self) -> ByteUnit {
        let (unit, _) = ByteUnit::fit(self);
        unit
    }

    pub fn exp(self, by: u32) -> Size {
        Size::from(self.bytes.checked_pow(by).unwrap_or(self.bytes))
    }
}
impl From<u64> for Size {
    fn from(bytes: u64) -> Size {
        Size { bytes }
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Self) {
        self.bytes += other.as_u64();
    }
}

impl Add for Size {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output::from(self.bytes + other.as_u64())
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, other: Self) {
        self.bytes -= other.as_u64();
    }
}

impl Sub for Size {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output::from(self.bytes - other.as_u64())
    }
}

impl MulAssign for Size {
    fn mul_assign(&mut self, other: Self) {
        self.bytes *= other.as_u64();
    }
}

impl Mul for Size {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output::from(self.bytes * other.as_u64())
    }
}

impl DivAssign for Size {
    fn div_assign(&mut self, other: Self) {
        self.bytes /= other.as_u64();
    }
}

impl Div for Size {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::Output::from(self.bytes / other.as_u64())
    }
}

impl Sum for Size {
    fn sum<S>(iter: S) -> Size
    where
        S: Iterator<Item = Size>,
    {
        let mut a = Size::default();
        for s in iter {
            a += s;
        }
        a
    }
}

#[cfg(test)]
mod test_byteunit {
    use super::*;
    #[test]
    fn test_unit_scale() {
        assert_eq!(ByteUnit::Byte.scale(), 1);
        assert_eq!(ByteUnit::Kilo.scale(), 2);
        assert_eq!(ByteUnit::Mega.scale(), 3);
        assert_eq!(ByteUnit::Giga.scale(), 4);
        assert_eq!(ByteUnit::Tera.scale(), 5);
        assert_eq!(ByteUnit::Peta.scale(), 6);
    }
    #[test]
    fn test_size_unit() {
        assert_eq!(ByteUnit::fit(Size::from(1)), (ByteUnit::Byte, None));
        assert_eq!(ByteUnit::fit(Size::from(1024)), (ByteUnit::Kilo, None));
        assert_eq!(ByteUnit::fit(Size::from(2037)), (ByteUnit::Kilo, Some(1013)));
        assert_eq!(ByteUnit::fit(Size::from(1024 * 1024)), (ByteUnit::Mega, None));
        assert_eq!(ByteUnit::fit(Size::from(1024 * 1024 + 37)), (ByteUnit::Mega, Some(37)));
        assert_eq!(ByteUnit::fit(Size::from(1024 * 1024 * 1024)), (ByteUnit::Giga, None));
        assert_eq!(ByteUnit::fit(Size::from(1024 * 1024 * 1024 + 37)), (ByteUnit::Giga, Some(37)));
        assert_eq!(ByteUnit::fit(Size::from(1024 * 1024 * 1024 * 1024)), (ByteUnit::Tera, None));
        assert_eq!(
            ByteUnit::fit(Size::from(1024 * 1024 * 1024 * 1024 * 1024)),
            (ByteUnit::Peta, None)
        );
        assert_eq!(
            ByteUnit::fit(Size::from(1024 * 1024 * 1024 * 1024 * 1024 + 37)),
            (ByteUnit::Peta, Some(37))
        );
    }
}
