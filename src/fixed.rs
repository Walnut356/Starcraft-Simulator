use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// A 32-bit signed integer representing a fixed point value.
///
/// * 1 bit reserved for sign
/// * 19 bits for the integer
/// * 12 bits for the decimal
///
/// Has impls for operations between i32s and f64s, though prefer Fixed32s and i32s when possible as
/// f64 math has conversion overhead
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Fixed32(i32);

impl Fixed32 {
    pub const MAX: Self = Fixed32(i32::MAX);
    pub const MIN: Self = Fixed32(i32::MIN);
    pub const EPSILON: Self = Fixed32(1);
    pub const EPSILON_F64: f64 = 1.0 / 4096.0;

    /// Masks out all but the integer bits
    const INT_MASK: i32 = 0b1111_1111_1111_1111_1111_0000_0000_0000u32 as i32;
    /// Masks out all but the decimal bits
    const FRAC_MASK: i32 = 0b0000_0000_0000_0000_0000_1111_1111_1111u32 as i32;

    // returns the integer portion of the fixed point number
    pub fn int(&self) -> i32 {
        (self.0 & Self::INT_MASK) >> 12
    }

    // returns the fractional portion of the fixed point number
    pub fn fract(&self) -> f64 {
        (self.0 & Self::FRAC_MASK) as f64 / 4096.0
    }

    pub fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    pub fn raw_value(&self) -> i32 {
        self.0
    }

    pub fn floor(&self) -> Self {
        Self(self.0 & Self::INT_MASK)
    }

    pub fn trunc(&self) -> Self {
        if self.is_negative() {
            Self((self.0 & Self::INT_MASK) + (1 << 12))
        } else {
            Self(self.0 & Self::INT_MASK)
        }
    }

    pub fn ceil(&self) -> Self {
        // 0b1000_0000_0000 = 0.5
        if self.0 & 0b1000_0000_0000 == 0 {
            Self(self.0 & Self::INT_MASK)
        } else {
            Self((self.0 & Self::INT_MASK) + (1 << 12))
        }
    }

    pub fn round(&self) -> Self {
        // 0b1000_0000_0000 = 0.5
        if self.0 & 0b1000_0000_0000 == 0 {
            Self(self.0 & Self::INT_MASK)
        } else {
            Self((self.0 & Self::INT_MASK) + (1 << 12))
        }
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}

impl std::fmt::Display for Fixed32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f64::from(*self))
    }
}

impl std::fmt::Debug for Fixed32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fixed32({})", f64::from(*self))
    }
}

impl From<i32> for Fixed32 {
    fn from(value: i32) -> Self {
        Self(value << 12)
    }
}

impl From<f64> for Fixed32 {
    fn from(value: f64) -> Self {
        Self((value * 4096.0) as i32)
    }
}

impl From<Fixed32> for i32 {
    fn from(value: Fixed32) -> Self {
        value.0 >> 12
    }
}

impl From<Fixed32> for f64 {
    fn from(value: Fixed32) -> Self {
        value.0 as f64 / 4096.0
    }
}

impl Neg for Fixed32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Fixed32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Fixed32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Fixed32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) / 4096)
    }
}

impl Div for Fixed32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from(self.0 as f64 / rhs.0 as f64)
    }
}

impl Add<i32> for Fixed32 {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + (rhs << 12))
    }
}

impl Sub<i32> for Fixed32 {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self(self.0 - (rhs << 12))
    }
}

impl Mul<i32> for Fixed32 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * (rhs << 12) / 4096)
    }
}

impl Div<i32> for Fixed32 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self::from(self.0 as f64 / rhs as f64)
    }
}

impl PartialEq<i32> for Fixed32 {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other << 12
    }
}

impl PartialOrd<i32> for Fixed32 {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&(*other << 12)))
    }
}

// impl Add<f64> for Fixed32 {
//     type Output = Self;

//     fn add(self, rhs: f64) -> Self::Output {
//         self + Self::from_f64(rhs)
//     }
// }

impl PartialEq<f64> for Fixed32 {
    fn eq(&self, other: &f64) -> bool {
        *other == (*self).into()
    }
}

impl PartialOrd<f64> for Fixed32 {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        let f = f64::from(*self);
        let diff = f - *other;
        if diff > Self::EPSILON_F64 {
            Some(Ordering::Greater)
        } else if diff < -Self::EPSILON_F64 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}
