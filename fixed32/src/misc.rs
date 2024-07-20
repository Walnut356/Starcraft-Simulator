use std::iter::Sum;

use crate::Fixed32;

impl Sum<Fixed32> for Fixed32 {
    fn sum<I: Iterator<Item = Fixed32>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}