use const_soft_float::soft_f64::SoftF64;

use crate::Fixed32;

impl Fixed32 {
    pub const PI: Self = Fixed32::from_f64(std::f64::consts::PI);
    pub const FRAC_1_PI: Self = Fixed32::from_f64(std::f64::consts::FRAC_1_PI);
    pub const FRAC_1_SQRT_PI: Self = Fixed32::from_f64(1.0 / std::f64::consts::PI);
    pub const TAU: Self = Fixed32::from_f64(std::f64::consts::TAU);
    pub const T: Self = Fixed32::PI.const_mul(Fixed32::TWO);
    pub const FRAC_PI_2: Self = Fixed32::from_f64(std::f64::consts::FRAC_PI_2);
    // pub const FRAC_180_PI: Self = Fixed32::from_i32(180).const_div(Fixed32::PI);
    // pub const FRAC_PI_180: Self = Fixed32::PI.const_div(Fixed32::from_i32(180));

    // these two functions are two of the few that i'll resort to float casting. Repeated back and
    // forth conversion via fixed point numbers results in compounding inaccuracy that's surprisingly
    // high. E.g. 180.to_radians() = ~3.12, not ~3.14 which is rough
    pub const fn to_radians(self) -> Self {
        self.const_mul(Self::PI.const_div(Self::from_i32(180)))
        // Self::from_f64(
        //     SoftF64(self.as_f64())
        //         .mul(SoftF64(std::f64::consts::PI).div(SoftF64(180.0)))
        //         .0,
        // )
    }

    pub fn to_degrees(self) -> Self {
        self.const_mul(Self::from_i32(180).const_div(Self::PI))
        // Self::from_f64(
        //     SoftF64(self.as_f64())
        //         .mul(SoftF64(180.0).div(SoftF64(std::f64::consts::PI)))
        //         .0,
        // )
    }

    // the fixed crate doesn't have atan2, and I'd rather not resort to float casting for what tends to
    // be a pretty hot function. I've taken the approximation from the link below and just converted the
    // numeric constants directly into Fixed32's
    // see: https://mazzo.li/posts/vectorized-atan2.html
    pub const fn atan2(y: Self, x: Self) -> Self {
        let swap = y.abs().const_gt(x.abs());
        let result = if swap {
            let val = x.const_div(y);
            let adjust = if val.0 >= 0 {
                Self::FRAC_PI_2
            } else {
                Self::FRAC_PI_2.const_neg()
            };

            adjust.const_sub(Self::atan_scalar_approx(val))
        } else {
            Self::atan_scalar_approx(y.const_div(x))
        };

        match (x.0 >= 0, y.0 >= 0) {
            (true, _) => result,
            (false, true) => Self::PI.const_add(result),
            (false, false) => Self::PI.const_neg().const_add(result),
        }
    }

    const fn atan_scalar_approx(x: Fixed32) -> Fixed32 {
        const A1: Fixed32 = Fixed32::from_f64(0.99997726f64);
        const A3: Fixed32 = Fixed32::from_f64(-0.33262347f64);
        const A5: Fixed32 = Fixed32::from_f64(0.19354346f64);
        const A7: Fixed32 = Fixed32::from_f64(-0.11643287f64);
        const A9: Fixed32 = Fixed32::from_f64(0.05265332f64);
        const A11: Fixed32 = Fixed32::from_f64(-0.01172120f64);

        let x_sq: Self = x.squared();

        // even rustfmt doesnt know what do to with it lmao
        x.const_mul(
            A1.const_add(x_sq.const_mul(A3.const_add(x_sq.const_mul(A5.const_add(
                x_sq.const_mul(A7.const_add(x_sq.const_mul(A9.const_add(x_sq.const_mul(A11))))),
            ))))),
        )
    }

    pub const fn squared(self) -> Self {
        self.const_mul(self)
    }

    pub const fn powi(self, pow: i32) -> Self {
        let mut i = 0;
        let mut result = self;
        while i < pow {
            result = result.const_mul(self);
            i += 1;
        }

        result
    }

    // taken directly from the fixed crate here: https://gitlab.com/tspiteri/fixed/-/blob/master/src/sqrt.rs?ref_type=heads
    pub const fn sqrt(self) -> Self {
        if self.0 == 0 {
            return self;
        }
        if self.is_negative() {
            panic!("Square root of negative")
        }
        const FRAC_NBITS: u32 = 12;
        const INT_NBITS: u32 = i32::BITS - FRAC_NBITS;
        let val = self.0 as u32;
        let odd_frac_nbits = FRAC_NBITS % 2 != 0;
        let leading = val.leading_zeros();
        let sig_int_pairs = if odd_frac_nbits {
            ((INT_NBITS + 1) / 2) as i32 - ((leading + 1) / 2) as i32
        } else {
            (INT_NBITS / 2) as i32 - (leading / 2) as i32
        };

        let mut i = 1;
        let mut q_i = 1 << (i32::BITS - 2);
        let mut next_bit = q_i >> 1;
        let mut y_i = val;
        let input_shl = INT_NBITS as i32 - sig_int_pairs * 2;
        if input_shl < 0 {
            // This can only happen when we have odd frac_nbits and the most
            // significant bit is set. We would need to shift right by 1.
            debug_assert!(input_shl == -1);

            // Do one iteration here as this is a special case.

            // In this special case, y is in the range [1, 2) instead of [1, 4),
            // and q is in the range [1, √2) instead of [1, 2).
            // Therefore, q_1 is always 0b1.0, and never 0b1.1.
            // Since q_0 = q_1 = 1, y_1 = 2 × (y - q_1^2) = 2 × y - 2 × q_i.
            // Since input_shl is -1, its effect is cancelled out by 2 × y,
            // and we only need to subtract 2 × q_i from y_i.
            y_i -= 2 * q_i;
            next_bit >>= 1;
            i += 1;
        } else {
            y_i <<= input_shl;
            y_i -= q_i;
        };

        let iters = (FRAC_NBITS as i32 - 1 + sig_int_pairs) as u32;
        while i <= iters {
            let d = next_bit >> 1;
            if d == 0 {
                if i == iters {
                    // Here result_shr must be 0, otherwise we wouldn't have
                    // room to potentially insert one extra bit.
                    debug_assert!(INT_NBITS as i32 - 1 - sig_int_pairs == 0);

                    // d == 0.5, so we Selfly need q_i + 0.5 <= y_i,
                    // which can be obtained with q_i < y_i
                    if q_i < y_i {
                        q_i += 1;
                    }

                    return Fixed32::from_bits(q_i as i32);
                }

                debug_assert!(i == iters - 1);
                // Here result_shr must be -1, otherwise we wouldn't have
                // room to potentially insert two extra bits.
                debug_assert!(INT_NBITS as i32 - 1 - sig_int_pairs == -1);

                // d == 0.5, so we Selfly need q_i + 0.5 <= y_i,
                // which can be obtained with q_i < y_i
                if q_i < y_i {
                    // We cannot subtract d == 0.5 from y_i immediately, so
                    // we subtract 1 from y_i before the multiplication by 2
                    // and then add 1 back. (There may be a potential overflow
                    // if we multiply y_i by 2 and then subtract 1.)
                    y_i -= q_i + 1;
                    y_i *= 2;
                    y_i += 1;
                    q_i += 1;
                } else {
                    y_i *= 2;
                }

                // d == 0.25, so we Selfly need q_i + 0.25 <= y_i,
                // which can be obtained with q_i < y_i
                if q_i < y_i {
                    // We cannot add next_bit == 0.5 to q_i immediately, so
                    // we add 1 to q_i after the left shift.
                    q_i = (q_i << 1) + 1;
                } else {
                    q_i <<= 1;
                }

                return Fixed32::from_bits(q_i as i32);
            }

            if q_i + d <= y_i {
                y_i -= q_i + d;
                q_i += next_bit;
            }
            y_i *= 2;

            next_bit = d;
            i += 1;
        }
        let result_shr = INT_NBITS as i32 - 1 - sig_int_pairs;
        Fixed32::from_bits((q_i >> result_shr) as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::Fixed32;

    #[test]
    fn sqrt() {
        assert_eq!(Fixed32::from(4).sqrt(), Fixed32::from(2));
        assert_eq!(Fixed32::from(100).sqrt(), Fixed32::from(10));
        assert_eq!(Fixed32::from(150).sqrt(), Fixed32::from(12.24744871391589));
    }

    #[test]
    fn rad_degree() {
        // assert_eq!(Fixed32::PI.const_div(Fixed32::from_i32(180)), Fixed32::FRAC_PI_180);
        // assert_eq!(Fixed32::from_i32(180).const_div(Fixed32::PI), Fixed32::FRAC_180_PI);
        assert_eq!(Fixed32::PI.to_degrees(), Fixed32::from_i32(180));
        assert_eq!(
            Fixed32::from(360)
                .to_radians(),
            Fixed32::TAU
        );
    }
}
