mod unit_data;
pub use unit_data::*;
mod weapon_data;
pub use weapon_data::*;
mod upgrades;
pub use upgrades::*;
mod weapon;
pub use weapon::*;
mod builder;

use std::{default, ops::Range};
use strum::{Display, EnumString, IntoStaticStr};

use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    IntoStaticStr,
    Display,
    strum::VariantNames,
    PartialOrd,
    Ord,
)]
#[repr(u8)]
pub enum Base {
    /// Custom unit, contains an ID to differentiate between multiple custom units
    Custom,

    Probe,
    Zealot,
    Stalker,
    Adept,
    Sentry,
    HighTemplar,
    DarkTemplar,
    Archon,
    Observer,
    WarpPrism,
    Immortal,
    Colossus,
    Disruptor,
    Phoenix,
    VoidRay,
    Oracle,
    Tempest,
    Carrier,
    Interceptor,
    Mothership,

    SCV,
    MULE,
    Marine,
    Reaper,
    Marauder,
    Ghost,
    Hellion,
    Hellbat,
    WidowMine,
    WidowMineBurrowed,
    Cyclone,
    SiegeTank,
    SiegeTankSieged,
    Thor,
    ThorAlt,
    VikingGround,
    VikingAir,
    Medivac,
    Liberator,
    LiberatorSieged,
    Raven,
    Banshee,
    Battlecruiser,

    Drone,
    Larva,
    Cocoon,
    Overlord,
    Overseer,
    Queen,
    Zergling,
    Baneling,
    Roach,
    RoachBurrowed,
    Ravager,
    RavagerCocoon,
    Hydralisk,
    Lurker,
    LurkerBurrowed,
    LurkerEgg,
    Mutalisk,
    Corruptor,
    SwarmHost,
    Locust,
    LocustFlying,
    Infestor,
    InfestorBurrowed,
    Viper,
    Ultralisk,
    BroodLord,
    Broodling,
    Changeling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr, Display)]
#[repr(u16)]
pub enum Flag {
    None = 0,
    Light = 1 << 0,
    Armored = 1 << 1,
    Biological = 1 << 2,
    Massive = 1 << 3,
    Mechanical = 1 << 4,
    Psionic = 1 << 5,
    Structure = 1 << 6,
    Heroic = 1 << 7,
    AlwaysThreat = 1 << 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr, Display)]
pub enum Faction {
    Custom,
    Protoss,
    Terran,
    Zerg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr, Display, Default)]
pub enum Collision {
    /// Should only apply to invincible units like ones under stasis
    None,
    #[default]
    Ground,
    Flying,
    /// Should only apply to the colossus
    Both,
}

impl Collision {
    pub fn can_interact(&self, other: Self) -> bool {
        match self {
            Collision::None => false,
            Collision::Ground | Collision::Flying => *self == other || other == Collision::Both,
            Collision::Both => other != Collision::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    None = 0,
    Cocoon = 10,
    Building = 11,
    Low = 19,
    Normal = 20,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, IntoStaticStr, Display)]
// pub enum FlagArmor {
//     None,
//     Light,
//     Armored,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, IntoStaticStr, Display)]
// pub enum FlagTaxonomy {
//     None,
//     Biological,
//     Mechanical,
//     Both,
// }

#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub(crate) inner: u32,
    // pub light: bool,
    // pub armored: bool,
    // pub mechanical: bool,
    // pub biological: bool,
    // pub massive: bool,
    // pub psionic: bool,
    // pub structure: bool,
    // pub heroic: bool,
    // pub always_threat: bool,
    // pub hallucination: bool,
}

impl Flags {
    // /// Shortcut for new that sets everything except armor and taxonomy to false
    // pub const fn new_std(armor: Flag, taxonomy: Flag) -> Self {
    //     let mut inner = 0;
    //     inner = match armor {
    //         Flag::Light | Flag::Armored => inner | armor as u32,
    //         Flag::Both => inner | Flag::Light as u32 | Flag::Armored as u32,
    //         _ => inner,
    //     };

    //     inner = match taxonomy {
    //         Flag::Biological | Flag::Mechanical => inner | taxonomy as u32,
    //         Flag::Both => inner | Flag::Biological as u32 | Flag::Mechanical as u32,
    //         _ => inner
    //     };

    //     Self {
    //         inner,
    //         light: matches!(armor, Flag::Light),
    //         armored: matches!(armor, Flag::Armored),
    //         biological: matches!(taxonomy, Flag::Biological | Flag::Both),
    //         mechanical: matches!(taxonomy, Flag::Mechanical | Flag::Both),
    //         massive: false,
    //         psionic: false,
    //         structure: false,
    //         heroic: false,
    //         always_threat: false,
    //         hallucination: false,
    //     }
    // }
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        light: bool,
        armored: bool,
        mechanical: bool,
        biological: bool,
        massive: bool,
        psionic: bool,
        structure: bool,
        heroic: bool,
        always_threat: bool,
    ) -> Self {
        let mut inner = 0;
        inner |= light as u32;
        inner |= (armored as u32) << 1;
        inner |= (biological as u32) << 2;
        inner |= (massive as u32) << 3;
        inner |= (mechanical as u32) << 4;
        inner |= (psionic as u32) << 5;
        inner |= (structure as u32) << 6;
        inner |= (heroic as u32) << 7;
        inner |= (always_threat as u32) << 8;

        Self { inner }
    }

    pub const fn is(&self, flag: Flag) -> bool {
        self.inner & flag as u32 != 0
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Cost {
    pub minerals: Real,
    pub gas: Real,
    pub supply: Real, // has to be a float to allow for .5 supply zerglings
    pub build_time: Real,
}

impl Cost {
    pub const fn new(minerals: u32, gas: u32, supply: u8, build_time: Real) -> Self {
        Self {
            minerals: Real::from_i32(minerals as i32),
            gas: Real::from_i32(gas as i32),
            supply: Real::from_i32(supply as i32),
            build_time,
        }
    }

    pub fn total_resources(&self) -> Real {
        self.minerals + self.gas
    }

    pub fn is_free(&self) -> bool {
        self.total_resources() == 0
    }
}

impl std::iter::Sum for Cost {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap_or_default()
    }
}

impl std::ops::Add for Cost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            minerals: self.minerals + rhs.minerals,
            gas: self.gas + rhs.gas,
            supply: self.supply + rhs.supply,
            build_time: self.build_time + rhs.build_time,
        }
    }
}

impl std::ops::Sub for Cost {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            minerals: self.minerals - rhs.minerals,
            gas: self.gas - rhs.gas,
            supply: self.supply - rhs.supply,
            build_time: self.build_time - rhs.build_time,
        }
    }
}

impl std::ops::Mul<i32> for Cost {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            minerals: self.minerals * rhs,
            gas: self.gas * rhs,
            supply: self.supply * rhs,
            build_time: self.build_time * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Health {
    pub max: Real,
    pub regen: Real,
    pub delay: Real,
    pub armor: Real,
}



#[derive(Debug, Clone, Default)]
pub struct Movement {
    pub speed: Real,
    pub accel: Real,
    pub decel: Real,
    pub turn_rate: Real,
    pub lateral_accel: Real,
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub base: Base,
    pub faction: Faction,
    /// Ground, Air, Both, or None
    pub collision: Collision,
    pub flags: Flags,
    pub hull: Health,
    pub shields: Health,
    pub movement: Movement,
    pub cost: Cost,
    pub size: Real,
    pub cargo_size: Real,
    pub sight: Real,
    pub weapons: [Option<Weapon>; 2],
    pub push_priority: Real,
    pub energy_start: Real,
    pub energy_max: Real,
}

impl Unit {
    // #[allow(clippy::too_many_arguments)]
    // pub const fn new(
    //     base: Base,
    //     faction: Faction,
    //     collision: Collision,
    //     flags: Flags,
    //     max_hull: u32,
    //     hull_armor: u8,
    //     max_shields: u32,
    //     weapons: [Option<Weapon>; 2],
    //     cost: Cost,
    //     size: Real,
    //     cargo_size: u8,
    //     sight: Real,
    //     movement: Movement,
    // ) -> Self {
    //     let hp = Real::const_from_int(max_hull as i32);
    //     let sh = Real::const_from_int(max_shields as i32);
    //     let (shield_regen, shield_delay) = if matches!(faction, Faction::Protoss) {
    //         (SHIELD_RECHARGE_RATE, SHIELD_RECHARGE_DELAY)
    //     } else {
    //         (const_real!(0), const_real!(0))
    //     };

    //     let (hull_regen, hull_delay) =
    //         if matches!(faction, Faction::Zerg) | matches!(base, Base::Reaper) {
    //             match base {
    //                 Base::Mutalisk => (MUTA_REGEN, const_real!(0)),
    //                 Base::Reaper => (SHIELD_RECHARGE_RATE, SHIELD_RECHARGE_DELAY),
    //                 _ => (ZERG_REGEN, const_real!(0)),
    //             }
    //         } else {
    //             (const_real!(0), const_real!(0))
    //         };

    //     Self {
    //         base,
    //         faction,
    //         collision,
    //         flags,
    //         hull: Health {
    //             max: hp,
    //             regen: hull_regen,
    //             delay: hull_delay,
    //             armor: const_real!(hull_armor),
    //         },
    //         shields: Health {
    //             max: sh,
    //             regen: shield_regen,
    //             delay: shield_delay,
    //             armor: const_real!(0),
    //         },
    //         weapons,
    //         cost,
    //         size,
    //         cargo_size: const_real!(cargo_size),
    //         sight,
    //         movement,
    //         push_priority: const_real!(10),
    //     }
    // }



    pub fn try_get_weapon(&self, target: &Unit) -> Option<&Weapon> {
        self.weapons
            .iter()
            .find_map(|w| w.as_ref().filter(|x| x.can_hit(target.collision)))
    }

    pub fn has_flag(&self, flag: Flag) -> bool {
        self.flags.is(flag)
    }

    pub fn ideal_dps(&self, weapon_idx: usize, with_bonus: bool) -> Range<Real> {
        self.weapons[weapon_idx]
            .as_ref()
            .map(|x| x.dps(with_bonus))
            .unwrap_or(real!(0)..real!(0))
    }
}
