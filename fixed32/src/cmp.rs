use crate::Fixed32;

use std::cmp::Ordering;

impl Fixed32 {
    pub const fn min(self, rhs: Self) -> Self {
        if self.0 > rhs.0 {
            rhs
        } else {
            self
        }
    }

    pub const fn approx_eq(self, rhs: Self) -> bool {
        (self.0 - rhs.0).abs() <= const { Fixed32::from_f64(0.001).0 }
    }

    pub const fn const_eq(self, rhs: Self) -> bool {
        self.0 == rhs.0
    }

    pub const fn const_lt(self, rhs: Self) -> bool {
        self.0 < rhs.0
    }

    pub const fn const_gt(self, rhs: Self) -> bool {
        self.0 > rhs.0
    }

    pub const fn const_lte(self, rhs: Self) -> bool {
        self.0 <= rhs.0
    }

    pub const fn const_gte(self, rhs: Self) -> bool {
        self.0 >= rhs.0
    }
}

impl PartialEq<Fixed32> for &Fixed32 {
            fn eq(&self, other: &Fixed32) -> bool {
                **self == *other
            }
        }

impl PartialEq<f64> for Fixed32 {
    fn eq(&self, other: &f64) -> bool {
        *self == Self::from_f64(*other)
    }
}

impl PartialEq<f32> for Fixed32 {
    fn eq(&self, other: &f32) -> bool {
        *self == Self::from_f32(*other)
    }
}

impl PartialOrd<f64> for Fixed32 {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        let f = Self::from_f64(*other);
        let diff = *self - f;
        if diff >= Self::EPSILON_F64 {
            Some(Ordering::Greater)
        } else if diff < -Self::EPSILON_F64 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl PartialOrd<f32> for Fixed32 {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        let f = Self::from_f32(*other);
        let diff = *self - f;
        if diff >= Self::EPSILON_F64 {
            Some(Ordering::Greater)
        } else if diff < -Self::EPSILON_F64 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

macro_rules! cmp {
    ($x:ty) => {
        impl PartialEq<$x> for Fixed32 {
            fn eq(&self, other: &$x) -> bool {
                self == Self::from(*other)
            }
        }

        impl PartialOrd<$x> for Fixed32 {
            fn partial_cmp(&self, other: &$x) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&Self::from(other).0)
            }
        }
    }
}

cmp!(u8);
cmp!(u16);
cmp!(u32);
cmp!(u64);
cmp!(usize);
cmp!(i8);
cmp!(i16);
cmp!(i32);
cmp!(i64);
cmp!(isize);