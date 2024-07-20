#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]

use crate::Fixed32;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ---------------------------------------------------------------------------------------------- //
//                                              Self                                              //
// ---------------------------------------------------------------------------------------------- //

impl Fixed32 {
    pub const fn const_neg(self) -> Self {
        Self(-self.0)
    }
    pub const fn const_add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    pub const fn const_sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }

    pub const fn const_mul(self, rhs: Self) -> Self {
        Fixed32((((self.0 as i64).wrapping_mul(rhs.0 as i64)) >> 12) as i32)
    }

    pub const fn const_div(self, rhs: Self) -> Self {
        let a = (self.0 as i64) << 12;
        let (bits, false) = a.overflowing_div(rhs.0 as i64) else {
            return Self::ZERO;
        };

        Self::from_bits(bits as i32)
    }
}

impl Neg for Fixed32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Neg for &Fixed32 {
    type Output = Fixed32;

    fn neg(self) -> Self::Output {
        Fixed32(-self.0)
    }
}

impl Add for Fixed32 {
    type Output = Fixed32;

    fn add(self, rhs: Self) -> Self::Output {
        Fixed32(self.0 + rhs.0)
    }
}

impl Add for &Fixed32 {
    type Output = Fixed32;

    fn add(self, rhs: Self) -> Self::Output {
        Fixed32(self.0 + rhs.0)
    }
}

impl Add<&Fixed32> for Fixed32 {
    type Output = Fixed32;

    fn add(self, rhs: &Fixed32) -> Self::Output {
        Fixed32(self.0 + rhs.0)
    }
}

impl Add<Fixed32> for &Fixed32 {
    type Output = Fixed32;

    fn add(self, rhs: Fixed32) -> Self::Output {
        Fixed32(self.0 + rhs.0)
    }
}

impl AddAssign<Fixed32> for Fixed32 {
    fn add_assign(&mut self, rhs: Fixed32) {
        self.0 += rhs.0;
    }
}
impl AddAssign<&Fixed32> for Fixed32 {
    fn add_assign(&mut self, rhs: &Fixed32) {
        self.0 += rhs.0;
    }
}

impl Sub<Fixed32> for Fixed32 {
    type Output = Fixed32;

    fn sub(self, rhs: Fixed32) -> Self::Output {
        Fixed32(self.0 - rhs.0)
    }
}

impl Sub<&Fixed32> for Fixed32 {
    type Output = Fixed32;

    fn sub(self, rhs: &Fixed32) -> Self::Output {
        Fixed32(self.0 - rhs.0)
    }
}

impl Sub<Fixed32> for &Fixed32 {
    type Output = Fixed32;

    fn sub(self, rhs: Fixed32) -> Self::Output {
        Fixed32(self.0 - rhs.0)
    }
}

impl Sub<&Fixed32> for &Fixed32 {
    type Output = Fixed32;

    fn sub(self, rhs: &Fixed32) -> Self::Output {
        Fixed32(self.0 - rhs.0)
    }
}

impl SubAssign<Fixed32> for Fixed32 {
    fn sub_assign(&mut self, rhs: Fixed32) {
        self.0 -= rhs.0;
    }
}
impl SubAssign<&Fixed32> for Fixed32 {
    fn sub_assign(&mut self, rhs: &Fixed32) {
        self.0 -= rhs.0;
    }
}

impl Mul<Fixed32> for Fixed32 {
    type Output = Fixed32;

    fn mul(self, rhs: Fixed32) -> Self::Output {

        Fixed32((((self.0 as i64).wrapping_mul(rhs.0 as i64)) >> 12) as i32)
    }
}

impl Mul<Fixed32> for &Fixed32 {
    type Output = Fixed32;

    fn mul(self, rhs: Fixed32) -> Self::Output {
        Fixed32((((self.0 as i64).wrapping_mul(rhs.0 as i64)) >> 12) as i32)
    }
}

impl Mul<&Fixed32> for Fixed32 {
    type Output = Fixed32;

    fn mul(self, rhs: &Fixed32) -> Self::Output {
        Fixed32(((self.0 as i64).wrapping_mul(rhs.0 as i64) >> 12) as i32)
    }
}

impl Mul<&Fixed32> for &Fixed32 {
    type Output = Fixed32;

    fn mul(self, rhs: &Fixed32) -> Self::Output {
        Fixed32(((self.0 as i64).wrapping_mul(rhs.0 as i64) >> 12) as i32)
    }
}

impl MulAssign<Fixed32> for Fixed32 {
    fn mul_assign(&mut self, rhs: Fixed32) {
        *self = Fixed32(((self.0 as i64).wrapping_mul(rhs.0 as i64) >> 12) as i32)
    }
}
impl MulAssign<&Fixed32> for Fixed32 {
    fn mul_assign(&mut self, rhs: &Fixed32) {
        *self = Fixed32(((self.0 as i64).wrapping_mul(rhs.0 as i64) >> 12) as i32)
    }
}

impl Div<Fixed32> for Fixed32 {
    type Output = Fixed32;

    fn div(self, rhs: Fixed32) -> Self::Output {
        let a = (self.0 as i64) << 12;
        let (bits, false) = a.overflowing_div(rhs.0 as i64) else {
            return Self::ZERO;
        };

        Self::from_bits(bits as i32)
    }
}

impl Div<&Fixed32> for Fixed32 {
    type Output = Fixed32;

    fn div(self, rhs: &Fixed32) -> Self::Output {
        let a = (self.0 as i64) << 12;
        let (bits, false) = a.overflowing_div(rhs.0 as i64) else {
            return Self::ZERO;
        };

        Self::from_bits(bits as i32)
    }
}

impl DivAssign<Fixed32> for Fixed32 {
    fn div_assign(&mut self, rhs: Fixed32) {
        *self = Fixed32((((self.0 as i64).wrapping_div((rhs.0 as i64) << 20)) >> 32) as i32);
    }
}

impl DivAssign<&Fixed32> for Fixed32 {
    fn div_assign(&mut self, rhs: &Fixed32) {
        *self = Fixed32((((self.0 as i64).wrapping_div((rhs.0 as i64) << 20)) >> 32) as i32);
    }
}

// ---------------------------------------------------------------------------------------------- //
//                                             Integer                                            //
// ---------------------------------------------------------------------------------------------- //

macro_rules! arith {
    ($rht:ty) => {
        impl Add<$rht> for Fixed32 {
            type Output = Fixed32;

            fn add(self, rhs: $rht) -> Fixed32 {
                Fixed32(self.0 + ((rhs as i32) << 12))
            }
        }

        impl Add<$rht> for &Fixed32 {
            type Output = Fixed32;

            fn add(self, rhs: $rht) -> Fixed32 {
                Fixed32(self.0 + ((rhs as i32) << 12))
            }
        }

        impl Add<&$rht> for Fixed32 {
            type Output = Fixed32;

            fn add(self, rhs: &$rht) -> Fixed32 {
                Fixed32(self.0 + ((*rhs as i32) << 12))
            }
        }

        impl Add<&$rht> for &Fixed32 {
            type Output = Fixed32;

            fn add(self, rhs: &$rht) -> Fixed32 {
                Fixed32(self.0 + ((*rhs as i32) << 12))
            }
        }

        impl AddAssign<$rht> for Fixed32 {
            fn add_assign(&mut self, rhs: $rht) {
                self.0 += (rhs as i32) << 12;
            }
        }

        impl AddAssign<&$rht> for Fixed32 {
            fn add_assign(&mut self, rhs: &$rht) {
                self.0 += (*rhs as i32) << 12;
            }
        }

        // ----------------------------------------- sub ---------------------------------------- //

        impl Sub<$rht> for Fixed32 {
            type Output = Fixed32;

            fn sub(self, rhs: $rht) -> Fixed32 {
                Fixed32(self.0 - ((rhs as i32) << 12))
            }
        }

        impl Sub<$rht> for &Fixed32 {
            type Output = Fixed32;

            fn sub(self, rhs: $rht) -> Fixed32 {
                Fixed32(self.0 - ((rhs as i32) << 12))
            }
        }

        impl Sub<&$rht> for Fixed32 {
            type Output = Fixed32;

            fn sub(self, rhs: &$rht) -> Fixed32 {
                Fixed32(self.0 - ((*rhs as i32) << 12))
            }
        }

        impl Sub<&$rht> for &Fixed32 {
            type Output = Fixed32;

            fn sub(self, rhs: &$rht) -> Fixed32 {
                Fixed32(self.0 - ((*rhs as i32) << 12))
            }
        }

        impl SubAssign<$rht> for Fixed32 {
            fn sub_assign(&mut self, rhs: $rht) {
                self.0 -= (rhs as i32) << 12;
            }
        }

        impl SubAssign<&$rht> for Fixed32 {
            fn sub_assign(&mut self, rhs: &$rht) {
                self.0 -= (*rhs as i32) << 12;
            }
        }

        // ----------------------------------------- Mul ---------------------------------------- //

        impl Mul<$rht> for Fixed32 {
            type Output = Fixed32;

            fn mul(self, rhs: $rht) -> Fixed32 {
                Fixed32((self.0).wrapping_mul(rhs as i32))
            }
        }

        impl Mul<$rht> for &Fixed32 {
            type Output = Fixed32;

            fn mul(self, rhs: $rht) -> Fixed32 {
                Fixed32((self.0).wrapping_mul(rhs as i32))
            }
        }

        impl Mul<&$rht> for Fixed32 {
            type Output = Fixed32;

            fn mul(self, rhs: &$rht) -> Fixed32 {
                Fixed32((self.0).wrapping_mul(*rhs as i32))
            }
        }

        impl Mul<&$rht> for &Fixed32 {
            type Output = Fixed32;

            fn mul(self, rhs: &$rht) -> Fixed32 {
                Fixed32((self.0).wrapping_mul(*rhs as i32))
            }
        }

        impl MulAssign<$rht> for Fixed32 {
            fn mul_assign(&mut self, rhs: $rht) {
                self.0 = self.0.wrapping_mul(rhs as i32);
            }
        }

        impl MulAssign<&$rht> for Fixed32 {
            fn mul_assign(&mut self, rhs: &$rht) {
                self.0 = self.0.wrapping_mul(*rhs as i32);
            }
        }

        // ----------------------------------------- Div ---------------------------------------- //

        impl Div<$rht> for Fixed32 {
            type Output = Fixed32;

            fn div(self, rhs: $rht) -> Fixed32 {
                let a = (self.0 as i64) << 12;
                let (bits, false) = a.overflowing_div((rhs as i64) << 12) else {
                    return Fixed32::ZERO;
                };

                Fixed32::from_bits(bits as i32)
            }
        }

        impl Div<$rht> for &Fixed32 {
            type Output = Fixed32;

            fn div(self, rhs: $rht) -> Fixed32 {
                let a = (self.0 as i64) << 12;
                let (bits, false) = a.overflowing_div((rhs as i64) << 12) else {
                    return Fixed32::ZERO;
                };

                Fixed32::from_bits(bits as i32)
            }
        }

        impl Div<&$rht> for Fixed32 {
            type Output = Fixed32;

            fn div(self, rhs: &$rht) -> Fixed32 {
                let a = (self.0 as i64) << 12;
                let (bits, false) = a.overflowing_div((*rhs as i64) << 12) else {
                    return Fixed32::ZERO;
                };

                Fixed32::from_bits(bits as i32)
            }
        }

        impl Div<&$rht> for &Fixed32 {
            type Output = Fixed32;

            fn div(self, rhs: &$rht) -> Fixed32 {
                let a = (self.0 as i64) << 12;
                let (bits, false) = a.overflowing_div((*rhs as i64) << 12) else {
                    return Fixed32::ZERO;
                };

                Fixed32::from_bits(bits as i32)
            }
        }

        impl DivAssign<$rht> for Fixed32 {
            fn div_assign(&mut self, rhs: $rht) {
                let a = (self.0 as i64) << 12;
                let (bits, false) = a.overflowing_div((rhs as i64) << 12) else {
                    self.0 = 0;
                    return;
                };

                *self = Fixed32::from_bits(bits as i32)
            }
        }

        impl DivAssign<&$rht> for Fixed32 {
            fn div_assign(&mut self, rhs: &$rht) {
                let a = (self.0 as i64) << 12;
                let (bits, false) = a.overflowing_div((*rhs as i64) << 12) else {
                    self.0 = 0;
                    return;
                };

                *self = Fixed32::from_bits(bits as i32)
            }
        }
    };
}

arith!(u8);
arith!(u16);
arith!(u32);
arith!(u64);
arith!(usize);
arith!(i8);
arith!(i16);
arith!(i32);
arith!(i64);
arith!(isize);

// ---------------------------------------------------------------------------------------------- //
//                                              Float                                             //
// ---------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use crate::Fixed32;

    // yeah i'm lazy
    macro_rules! val {
        ($x:literal) => {
            Fixed32::from($x)
        };
    }

    #[test]
    fn arith() {
        assert_eq!(val!(1) + val!(1), val!(2));
        assert_eq!(val!(-3.5) + val!(1), val!(-2.5));
        assert_eq!(val!(5) - val!(7), val!(-2));
        assert_eq!(val!(0.25) - val!(1), val!(-0.75));
        assert_eq!(val!(5) * val!(4), val!(20));
        assert_eq!(val!(10.5) * val!(2), val!(21));
        assert_eq!(val!(0.5) * val!(-1.0), val!(-0.5));
        assert_eq!(val!(1000) / val!(25), val!(40));
    }

    #[test]
    fn int_arith() {
        assert_eq!(val!(1) + 1, 2);
        assert_eq!(val!(-3.5) + 1, val!(-2.5));
        assert_eq!(val!(5) - 7, -2);
        assert_eq!(val!(0.25) - 1, val!(-0.75));
        assert_eq!(val!(5) * 4, 20);
        assert_eq!(val!(10.5) * 2, val!(21));
        assert_eq!(val!(1000) / 25, 40);
    }
}
