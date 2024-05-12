// mod fixed;
// pub use fixed::Fixed32;
pub mod unit;
pub use unit::{
    Armor, Base, Cost, Faction, Flag, Flags, Multihit, ProjType, Projectile, State, Target, Unit,
    Weapon,
};
pub mod army;
pub use army::Army;
pub mod coordinator;
pub mod ttk;
pub use coordinator::*;

/// Starcraft 2's internal float point numbers are represented via a 20:12 fixed point format. The
/// first bit is dedicated to the sign, 19 bits for the integer portion, and 12 for the fractional.
pub use fixed::types::I20F12 as Real;



/// Converts any number to a `Real` in a non-const context
#[macro_export]
macro_rules! real {
    ($x:expr) => {
        Real::from_num($x)
    };
} // from_num isn't const so i just pre-calculated the bits of each of these.

/// Converts any number to a `Real`, mostly useful when declaring `const` unit data. Cannot be used
/// in `const fn` due to floating point arithmetic
#[macro_export]
macro_rules! const_real {
    // horrible jank ahead, read at your own risk
    ($x:literal + $y:literal) => {
        const_real!($x).saturating_add(const_real!($y))
    };
    ($x:literal - $y:literal) => {
        const_real!($x).saturating_sub(const_real!($y))
    };
    ($x:literal * $y:literal) => {
        const_real!($x).saturating_mul(const_real!($y))
    };
    ($x:literal / $y:literal) => {
        const_real!($x).unwrapped_div(const_real!($y))
    };

    ($x:ident + $y:literal) => {
        const_real!($x).saturating_add(const_real!($y))
    };
    ($x:ident - $y:literal) => {
        const_real!($x).saturating_sub(const_real!($y))
    };
    ($x:ident * $y:literal) => {
        const_real!($x).saturating_mul(const_real!($y))
    };
    ($x:ident / $y:literal) => {
        const_real!($x).unwrapped_div(const_real!($y))
    };

    ($x:literal + $y:ident) => {
        const_real!($x).saturating_add(const_real!($y))
    };
    ($x:literal - $y:ident) => {
        const_real!($x).saturating_sub(const_real!($y))
    };
    ($x:literal * $y:ident) => {
        const_real!($x).saturating_mul(const_real!($y))
    };
    ($x:literal / $y:ident) => {
        const_real!($x).unwrapped_div(const_real!($y))
    };

    ($x:ident + $y:ident) => {
        const_real!($x).saturating_add(const_real!($y))
    };
    ($x:ident - $y:ident) => {
        const_real!($x).saturating_sub(const_real!($y))
    };
    ($x:ident * $y:ident) => {
        const_real!($x).saturating_mul(const_real!($y))
    };
    ($x:ident / $y:ident) => {
        const_real!($x).unwrapped_div(const_real!($y))
    };

    // (($x:expr) + ($y:expr)) => {
    //     const_real!($x).saturating_add(const_real!($y))
    // };



    // ($x:literal) => {
    //     {
    //         let val =  stringify!($x).as_bytes();
    //         let mut i = 0;
    //         let mut sign = 1;
    //         if val[0] == b'-' {
    //             i += 1;
    //             sign = -1;
    //         }
    //         let mut integer: i32 = 0;

    //         while i < val.len() && val[i] != b'.' {
    //             integer *= 10;
    //             integer += (val[i] - b'0') as i32;
    //             i += 1;
    //         }

    //         integer *= sign;

    //         i += 1;

    //         let mut frac: u64 = 0;
    //         let pad = i + 12;

    //         while i < pad {
    //             frac *= 10;
    //             if i < val.len() {
    //                 frac *= 10
    //             }
    //             i += 1;
    //         }

    //         frac /= 244140625;

    //         let bits: i32 = frac as i32 | (integer << 12) as i32;


    //         Real::from_bits(bits)
    //     }
    // };

    // expr must resolve into something `as` castable, which restricts it to i32s and f64s for most
    // practical purposes
    ($x:expr) => {
        {
            use const_soft_float::soft_f64::SoftF64;
            Real::from_bits(SoftF64($x as f64).mul(const_soft_float::soft_f64::SoftF64(4096.0)).to_f64() as i32)
        }
    };
}

/// Shorthand for `const_real!(x / GAME_SPEED)`, useful for build times, effect durations, etc.
#[macro_export]
macro_rules! duration {
    ($x:tt) => {
        const_real!($x / GAME_SPEED)
    };
}

/// Shorthand for `const_real!(x * GAME_SPEED)`, useful for things measured in "per second"
#[macro_export]
macro_rules! rate {
    ($x:tt) => {
        const_real!($x * GAME_SPEED)
    };
}

/// approximately 1.4
pub const GAME_SPEED_REAL: Real = const_real!(1.4);

/// approximately 1.4
pub const GAME_SPEED: f64 = ((1.4 * 4096.0) as i32) as f64 / 4096.0; // lol

/// Represents 1 in-game physics step, which occur 16 times per second in blizzard time, or
/// 22.4 times per second in real time.
pub const TICK: Real = const_real!(1.0 / 22.4);

/// the default random delay maximum applied to every attack (except for the first). See
/// BC_RANDOM_DELAY_MAX for the one exception: the battle cruiser
pub const RANDOM_DELAY_MAX: Real = const_real!(0.125 / GAME_SPEED);
/// the universal minimum random delay applied to every attack (except for the first)
pub const RANDOM_DELAY_MIN: Real = const_real!(-0.0625 / GAME_SPEED);

pub const RANDOM_DELAY_RANGE: std::ops::Range<Real> = RANDOM_DELAY_MIN..RANDOM_DELAY_MAX;
/// the battle cruiser is the one exception to the otherwise universal RANDOM_DELAY_MAX
pub const BC_RANDOM_DELAY_MAX: Real = const_real!(0.1875 / GAME_SPEED);

/// ~7.1433 seconds, also used for the reaper's regen delay
pub const SHIELD_RECHARGE_DELAY: Real = const_real!(10.0 / GAME_SPEED);
/// ~2.8/s
pub const SHIELD_RECHARGE_RATE: Real = const_real!(2.0 * GAME_SPEED);
// TODO editor says this value is rounded to the nearest 1/256th? (i.e. 0.273)
pub const ZERG_REGEN: Real = const_real!(0.2734 * GAME_SPEED);
/// Attacks must do at least this much damage. If an attack's damage is reduced below this amount via
/// armor or other damage reduction, it is clamped to 0.5
pub const MIN_DAMAGE: Real = const_real!(0.5);
pub const CHRONOBOOST_MOD: Real = const_real!(1.5);
