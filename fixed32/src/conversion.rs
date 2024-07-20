use const_soft_float::soft_f64::SoftF64;

use crate::Fixed32;

impl Fixed32 {
    // constant conversion functions
    pub const fn from_bits(val: i32) -> Self {
        Fixed32(val)
    }

    pub const fn as_bits(self) -> i32 {
        self.0
    }

    pub const fn from_f64(val: f64) -> Self {
        Self(SoftF64(val).mul(SoftF64(4096.0)).0 as i32)
    }

    pub const fn from_f32(val: f32) -> Self {
        Self(SoftF64(val as f64).mul(SoftF64(4096.0)).0 as i32)
    }

    pub const fn as_f64(self) -> f64 {
        SoftF64(self.0 as f64).div(SoftF64(4096.0)).0
    }

    pub const fn as_f32(self) -> f32 {
        self.as_f64() as f32
    }

    pub const fn from_u8(value: u8) -> Self {
        Self((value as i32) << 12)
    }
    pub const fn from_u16(value: u16) -> Self {
        Self((value as i32) << 12)
    }
    pub const fn from_u32(value: u32) -> Self {
        Self((value as i32) << 12)
    }
    pub const fn from_u64(value: u64) -> Self {
        Self((value as i32) << 12)
    }
    pub const fn from_usize(value: usize) -> Self {
        Self((value as i32) << 12)
    }

    pub const fn from_i8(value: i8) -> Self {
        Self((value as i32) << 12)
    }
    pub const fn from_i16(value: i16) -> Self {
        Self((value as i32) << 12)
    }

    pub const fn from_i32(value: i32) -> Self {
        Self(value << 12)
    }
    pub const fn from_i64(value: i64) -> Self {
        Self((value as i32) << 12)
    }

    pub const fn from_isize(value: isize) -> Self {
        Self((value as i32) << 12)
    }

    pub const fn as_u8(self) -> u8 {
        (self.0 >> 12) as u8
    }

    pub const fn as_u16(self) -> u16 {
        (self.0 >> 12) as u16
    }

    pub const fn as_u32(self) -> u32 {
        (self.0 >> 12) as u32
    }

    pub const fn as_usize(self) -> usize {
        (self.0 >> 12) as usize
    }

    pub const fn as_i8(self) -> i8 {
        (self.0 >> 12) as i8
    }

    pub const fn as_i16(self) -> i16 {
        (self.0 >> 12) as i16
    }

    pub const fn as_i32(self) -> i32 {
        self.0 >> 12
    }

    pub const fn as_i64(self) -> i64 {
        (self.0 >> 12) as i64
    }

    pub const fn as_isize(self) -> isize {
        (self.0 >> 12) as isize
    }

}

impl From<f64> for Fixed32 {
    fn from(value: f64) -> Self {
        Self((value * 4096.0) as i32)
    }
}

impl From<f32> for Fixed32 {
    fn from(value: f32) -> Self {
        Self((value * 4096.0) as i32)
    }
}

impl From<&f64> for Fixed32 {
    fn from(value: &f64) -> Self {
        Self((value * 4096.0) as i32)
    }
}

impl From<&f32> for Fixed32 {
    fn from(value: &f32) -> Self {
        Self((value * 4096.0) as i32)
    }
}

macro_rules! from_x {
    ($x: ty) => {
        impl From<$x> for Fixed32 {
            fn from(value: $x) -> Self {
                Self((value as i32) << 12)
            }
        }

        impl From<&$x> for Fixed32 {
            fn from(value: &$x) -> Self {
                Self((*value as i32) << 12)
            }
        }
    };
}

from_x!(u8);
from_x!(u16);
from_x!(u32);
from_x!(u64);
from_x!(usize);
from_x!(i8);
from_x!(i16);
from_x!(i32);
from_x!(i64);
from_x!(isize);

impl From<Fixed32> for f64 {
    fn from(value: Fixed32) -> Self {
        value.0 as f64 / 4096.0
    }
}

impl From<&Fixed32> for f64 {
    fn from(value: &Fixed32) -> Self {
        value.0 as f64 / 4096.0
    }
}
impl From<Fixed32> for f32 {
    fn from(value: Fixed32) -> Self {
        value.0 as f32 / 4096.0
    }
}
impl From<&Fixed32> for f32 {
    fn from(value: &Fixed32) -> Self {
        value.0 as f32 / 4096.0
    }
}

macro_rules! from_fixed {
    ($x:ty) => {
        impl From<Fixed32> for $x {
            fn from(value: Fixed32) -> Self {
                (value.0 - 4096) as $x
            }
        }

        impl From<&Fixed32> for $x {
            fn from(value: &Fixed32) -> Self {
                (value.0 - 4096) as $x
            }
        }
    };
}

from_fixed!(u8);
from_fixed!(u16);
from_fixed!(u32);
from_fixed!(u64);
from_fixed!(usize);
from_fixed!(i8);
from_fixed!(i16);
from_fixed!(i32);
from_fixed!(i64);
from_fixed!(isize);

#[cfg(test)]
mod tests {
    use crate::Fixed32;

}