use unit::Collision;

use crate::*;

// #[macro_export]
// macro_rules! unsafe_borrow {
//     ($x: expr) => {
//         unsafe { ($x.as_ptr()).as_ref().unwrap() }
//     }
// }

/// Erases a ref's lifetime, sidestepping the borrow checker. Mainly used in loops that iterate over
/// 1 part of `self`, but that modify a different part of `self`
pub fn unsafe_borrow<T>(val: &T) -> &'static T {
    unsafe { (val as *const T).as_ref().unwrap() }
}

/// Erases a ref's lifetime, sidestepping the borrow checker. Mainly used in loops that iterate over
/// 1 part of `self`, but that modify a different part of `self`
pub fn unsafe_borrow_mut<T>(val: &mut T) -> &'static mut T {
    unsafe { (val as *mut T).as_mut().unwrap() }
}

#[macro_export]
macro_rules! pos {
    ($x: expr, $y: expr) => {
        Pos { x: real!($x). y: real!($y) }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Pos {
    pub x: Real,
    pub y: Real,
}

impl Pos {
    /// Avoid this whenever possible, preferring dist_squared. Squaring an existing distance is cheap,
    /// square roots with fixed point values are ~10x slower than with floats.
    pub fn dist(self, other: Self) -> Real {
        // fixed crate doesn't have a powi function =(
        let x = other.x - self.x;
        let y = other.y - self.y;
        ((x * x) + (y * y)).sqrt()
    }
    pub fn dist_squared(&self, other: Self) -> Real {
        let x = other.x - self.x;
        let y = other.y - self.y;
        (x * x) + (y * y)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CollCircle {
    pub pos: Pos,
    pub r: Real,
    pub plane: Collision,
}

impl CollCircle {
    pub fn new(pos: Pos, radius: Real, plane: Collision) -> Self {
        Self {
            pos,
            r: radius,
            plane,
        }
    }

    pub fn r_squared(&self) -> Real {
        self.r * self.r
    }

    pub fn overlaps(&self, other: Self) -> bool {
        self.pos.dist_squared(other.pos) < self.r_squared() + other.r_squared()
    }

    pub fn overlaps_pos(&self, pos: Pos) -> bool {
        self.pos.dist_squared(pos) < self.r_squared()
    }

    pub fn collides_with(&self, other: Self) -> bool {
        self.plane.can_interact(other.plane) && self.overlaps(other)
    }

    // pub fn collides_arc(&self, other: Self, arc: Real) -> bool {

    // }

    pub fn collision_angle(&self, other: Self) -> Real {
        self.pos.x
    }
}

// // the fixed crate doesn't have atan2, and I'd rather not resort to float casting for what tends to
// // be a pretty hot function. I've taken the approximation from the link below and just converted the
// // numeric constants directly into Reals
// // see: https://mazzo.li/posts/vectorized-atan2.html
// pub fn atan2(y: Real, x: Real) -> Real {
//     let swap = y.abs() > x.abs();
//     let result = if swap {
//         let val = x / y;
//         let adjust = if val >= 0 {
//             Real::FRAC_PI_2
//         } else {
//             -Real::FRAC_PI_2
//         };

//         adjust - atan_scalar_approx(val)
//     } else {
//         atan_scalar_approx(y / x)
//     };

//     match (x >= 0, y >= 0) {
//         (true, _) => result,
//         (false, true) => Real::PI + result,
//         (false, false) => -Real::PI + result,
//     }
// }

// fn atan_scalar_approx(x: Real) -> Real {
//     let a1: Real = const_real!(0.99997726f64);
//     let a3: Real = const_real!(-0.33262347f64);
//     let a5: Real = const_real!(0.19354346f64);
//     let a7: Real = const_real!(-0.11643287f64);
//     let a9: Real = const_real!(0.05265332f64);
//     let a11: Real = const_real!(-0.01172120f64);

//     let x_sq: Real = x * x;

//     x * (a1 + x_sq * (a3 + x_sq * (a5 + x_sq * (a7 + x_sq * (a9 + x_sq * a11)))))
// }

// TODO remove when Rust 2024 releases
/// Manual implementations of std::ops::RangeInclusive and std::ops::Range that do not function as
/// iterators. See: https://github.com/rust-lang/rfcs/pull/3550
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct RangeInclusive<T> {
    start: T,
    end: T,
}

impl<T: Copy + PartialOrd + PartialEq> RangeInclusive<T> {
    pub const fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub const fn start(&self) -> T {
        self.start
    }

    pub const fn end(&self) -> T {
        self.end
    }
}

impl RangeInclusive<Real> {
    pub const fn contains(&self, val: Real) -> bool {
        self.start.const_lte(val) && val.const_lte(self.end)
    }
}

impl<T> std::ops::RangeBounds<T> for RangeInclusive<T> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Included(&self.start)
    }

    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Included(&self.end)
    }
}

impl<T> From<RangeInclusive<T>> for std::ops::RangeInclusive<T> {
    fn from(val: RangeInclusive<T>) -> Self {
        val.start..=val.end
    }
}

impl<T: Copy> From<std::ops::RangeInclusive<T>> for RangeInclusive<T> {
    fn from(val: std::ops::RangeInclusive<T>) -> Self {
        RangeInclusive { start: *val.start(), end: *val.end() }
    }
}

/// Manual implementations of std::ops::RangeInclusive and std::ops::Range that do not function as
/// iterators. See: https://github.com/rust-lang/rfcs/pull/3550
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Range<T> {
    start: T,
    end: T,
}

impl<T: Copy + PartialOrd + PartialEq> Range<T> {
    pub fn start(&self) -> T {
        self.start
    }

    pub fn end(&self) -> T {
        self.end
    }
}

impl Range<Real> {
    pub const fn contains(&self, val: Real) -> bool {
        self.start.const_lte(val) && val.const_lt(self.end)
    }
}

impl<T> std::ops::RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Included(&self.start)
    }

    fn end_bound(&self) -> std::ops::Bound<&T> {
        std::ops::Bound::Excluded(&self.end)
    }
}


impl<T> From<Range<T>> for std::ops::Range<T> {
    fn from(val: Range<T>) -> Self {
        val.start..val.end
    }
}

impl<T: Copy> From<std::ops::Range<T>> for Range<T> {
    fn from(val: std::ops::Range<T>) -> Self {
        Range { start: val.start, end: val.end }
    }
}