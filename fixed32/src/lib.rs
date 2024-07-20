use std::mem::{size_of, size_of_val};

mod arithmetic;
mod cmp;
mod conversion;
mod math;
mod misc;

/// A 32-bit signed integer representing a fixed point value.
///
/// * 1 bit reserved for sign
/// * 19 bits for the integer
/// * 12 bits for the decimal
///
/// Has impls for operations between i32s and f64s, though prefer Fixed32s and i32s when possible as
/// f64 math has conversion overhead
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Fixed32(i32);

impl Fixed32 {
    pub const BITS: usize = 32;
    pub const INT_BITS: usize = 20;
    pub const FRAC_BITS: usize = 12;

    pub const MAX: Self = Fixed32(i32::MAX);
    pub const MIN: Self = Fixed32(i32::MIN);
    pub const EPSILON: Self = Fixed32(1);
    pub const EPSILON_F64: Self = Fixed32::from_bits(1);
    // seems silly, but these 4 numbers are used *incredibly* often as constants. I wish the num
    // crate had a one_half const for floats and a two const for all `Num` objects
    pub const ZERO: Self = Self::from_i32(0);
    pub const ONE: Self = Self::from_i32(1);
    pub const TWO: Self = Self::from_i32(2);
    pub const ONE_HALF: Self = Self::from_bits(0b1000_0000_0000);
    pub const NEG_ONE: Self = Self::from_i32(-1);

    pub const INT_MASK: i32 = 0b1111_1111_1111_1111_1111_0000_0000_0000_u32 as i32;
    pub const FRAC_MASK: i32 = 0b0000_0000_0000_0000_0000_1111_1111_1111_u32 as i32;
    pub const SIGN_MASK: i32 = 0b1000_0000_0000_0000_0000_0000_0000_0000_u32 as i32;

    /// Returns 0 if the sign bit is 0 (i.e. the value is positive) or 1 if the sign bit is 1 (i.e.
    /// the value is negative)
    pub const fn sign_bit(self) -> Self {
        Self(((self.0 & Self::SIGN_MASK) as u32 >> 19) as i32)
    }

    /// Returns -1 if the value is negative, +1 if the value is positive
    pub const fn signum(self) -> Self {
        // from the Bit Twiddling Hacks. Compiles down to 3 instructions. the alternative, commented
        // out below, can also return 0 which I don't really want.
        Self::from_i32(1 | (self.0 >> (i32::BITS - 1)))

        // Self::from_i32(self.0.signum())
    }

    /// returns the integer portion of the fixed point number
    pub const fn integer(self) -> Self {
        Self(self.0 & Self::INT_MASK)
    }

    /// returns the fractional portion of the fixed point number
    pub const fn fract(self) -> Self {
        Self(self.0 & Self::FRAC_MASK)
    }

    pub const fn incr(self) -> Self {
        self.const_add(Self::ONE)
    }

    pub const fn decr(self) -> Self {
        self.const_sub(Self::ONE)
    }

    pub const fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    pub const fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    /// rounds value towards negative infinity
    pub const fn floor(self) -> Self {
        self.integer()
    }

    /// rounds value towards 0
    pub const fn trunc(self) -> Self {
        // compiles to cmove, eagerly calculating both sides
        if self.fract().0 != 0 {
            self.integer().const_add(self.sign_bit())
        } else {
            self
        }
    }

    /// rounds towards positive infinity
    pub const fn ceil(self) -> Self {
        // compiles branchless and cmove-less
        if self.0 & 0b1000_0000_0000 == 0 {
            self.integer()
        } else {
            self.integer().const_add(Self::from_i32(1))
        }
    }

    pub fn round(self) -> Self {
        // has cmov's and setne's from signum and trunc, but still a short function
        self.const_add(Self::ONE_HALF.const_mul(self.signum()))
            .trunc()
        // if self.abs().0 & Self::FRAC_MASK < 0b1000_0000_0000 {
        //     self.trunc()
        // } else {
        //     self.const_add(self.signum()).trunc()
        // }
    }

    pub const fn abs(self) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::Fixed32;

    #[test]
    fn signum() {
        assert_eq!(Fixed32::ONE.signum(), Fixed32::ONE);
        assert_eq!(Fixed32::from(-1).signum(), Fixed32::from(-1))
    }

    #[test]
    fn abs() {
        assert_eq!(Fixed32::from(1.0).abs(), Fixed32::from(1.0));
        assert_eq!(Fixed32::from(-1.0).abs(), Fixed32::from(1.0));
        assert_eq!(Fixed32::from(1.5).abs(), Fixed32::from(1.5));
        assert_eq!(Fixed32::from(-1.5).abs(), Fixed32::from(1.5));
    }

    #[test]
    fn floor() {
        assert_eq!(Fixed32::from(1.5).floor(), Fixed32::from(1.0));
        assert_eq!(Fixed32::from(-1.5).floor(), Fixed32::from(-2.0));
        assert_eq!(Fixed32::from(1.0).floor(), Fixed32::from(1.0));
    }

    #[test]
    fn trunc() {
        assert_eq!(Fixed32::from(1.5).trunc(), Fixed32::from(1.0));
        assert_eq!(Fixed32::from(-1.5).trunc(), Fixed32::from(-1.0));
        assert_eq!(Fixed32::from(1.0).trunc(), Fixed32::from(1.0));
        assert_eq!(Fixed32::from(-2).trunc(), Fixed32::from(-2));
    }

    #[test]
    fn ceil() {
        assert_eq!(Fixed32::from(1.5).ceil(), Fixed32::from(2.0));
        assert_eq!(Fixed32::from(-1.5).ceil(), Fixed32::from(-1.0));
        assert_eq!(Fixed32::from(1.0).ceil(), Fixed32::from(1.0));
    }

    #[test]
    fn round() {
        assert_eq!(Fixed32::from(1.5).round(), Fixed32::from(2.0));
        assert_eq!(Fixed32::from(1.4).round(), Fixed32::from(1.0));
        assert_eq!(Fixed32::from(-1.5).round(), Fixed32::from(-2.0));
        assert_eq!(Fixed32::from(-1.4).round(), Fixed32::from(-1.0));
        assert_eq!(Fixed32::from(1.0).round(), Fixed32::from(1.0));
    }
}
