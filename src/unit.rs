mod unit_data;
use std::ops::Range;

pub use unit_data::*;
mod weapon_data;
pub use weapon_data::*;
mod upgrades;
pub use upgrades::*;
mod weapon;
pub use weapon::*;

use strum::{Display, EnumString, IntoStaticStr};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, IntoStaticStr, Display)]
pub enum Base {
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
    Cyclone,
    SiegeTank,
    Thor,
    Viking,
    Medivac,
    Liberator,
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
    Ravager,
    Hydralisk,
    Lurker,
    Mutalisk,
    Corruptor,
    SwarmHost,
    Locust,
    Infestor,
    Viper,
    Ultralisk,
    BroodLord,
    Broodling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr, Display)]
pub enum Flag {
    None,
    Light,
    Armored,
    Biological,
    Massive,
    Mechanical,
    Psionic,
    Structure,
    Heroic,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr, Display)]
pub enum Faction {
    Protoss,
    Terran,
    Zerg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, IntoStaticStr, Display)]
pub enum Target {
    /// Should only apply to invincible units like ones under stasis
    None,
    Ground,
    Flying,
    /// Should only apply to the colossus
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum State {
    #[default]
    Wait,
    Attack,
    DmgPoint(Real),
    Attack2,
    Attack3,
    Attack4,
    Move,
    Dead,
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

#[derive(Debug, Clone)]
pub struct Flags {
    armor: Flag,
    taxonomy: Flag,
    massive: bool,
    psionic: bool,
    structure: bool,
    heroic: bool,
}

impl Flags {
    pub const fn new(
        armor: Flag,
        taxonomy: Flag,
        massive: bool,
        psionic: bool,
        structure: bool,
        heroic: bool,
    ) -> Self {
        Self {
            armor,
            taxonomy,
            massive,
            psionic,
            structure,
            heroic,
        }
    }

    /// Shortcut for new that sets everything except armor and taxonomy to false
    pub const fn new_std(armor: Flag, taxonomy: Flag) -> Self {
        Self {
            armor,
            taxonomy,
            massive: false,
            psionic: false,
            structure: false,
            heroic: false,
        }
    }

    pub fn contains(&self, flag: Flag) -> bool {
        match flag {
            Flag::Light | Flag::Armored => self.armor == flag,
            Flag::Biological | Flag::Mechanical => self.taxonomy == flag,
            Flag::Massive => self.massive,
            Flag::Psionic => self.psionic,
            Flag::Structure => self.structure,
            Flag::Heroic => self.heroic,
            // this should never be checked, but it'll always return false just in case
            Flag::None | Flag::Both => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Armor {
    pub hull: Real, // can be negative due to armor shred
    pub shield: Real,
}

impl Armor {
    pub const fn new(hull: u8, shield: u8) -> Self {
        Self {
            hull: Real::const_from_int(hull as i32),
            shield: Real::const_from_int(shield as i32),
        }
    }

    pub fn armor_upgrade(&mut self, level: Real) {
        self.hull += level;
    }

    pub fn shield_upgrade(&mut self, level: Real) {
        self.shield += level;
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
            minerals: Real::const_from_int(minerals as i32),
            gas: Real::const_from_int(gas as i32),
            supply: Real::const_from_int(supply as i32),
            build_time,
        }
    }

    pub fn total_resources(&self) -> Real {
        self.minerals + self.gas
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

macro_rules! builder_fn {
    ($x:ident, $t:ty) => {
        paste::paste! {
            pub const fn [<with_ $x>](mut self, $x: $t) -> Self {
                self.$x = $x;
                self
            }
        }
    };
    ($x:ident, $y:ident, $t:ty) => {
        paste::paste! {
            pub const fn [<with_ $y>](mut self, $y: $t) -> Self {
                self.$x.$y = $y;
                self
            }
        }
    };
    ($x:ident[], $t:ty) => {
        paste::paste! {
            pub const fn [<with_ $x>](mut self, $x: $t, idx: usize) -> Self {
                self.$x[idx] = $x;
                self
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub base: Base,
    pub faction: Faction,
    /// Ground, Air, Both, or None
    pub kind: Target,
    pub flags: Flags,
    pub max_health: Real,
    pub health: Real,
    pub health_regen: Real,
    pub max_shields: Real,
    pub shields: Real,
    pub shield_regen: Real,
    pub shield_regen_delay: Real,
    pub weapons: [Option<Weapon>; 3],
    pub armor: Armor,
    pub cost: Cost,
    pub size: Real,
    pub move_speed: Real,
    /// Handle to the current target
    pub curr_target: Option<usize>,
    pub death_timestamp: Option<Real>,
    pub last_damaged: Option<Real>,
    pub damage_dealt: Real,
    pub overkill: Real,
    pub state: State,
}

impl Unit {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        base: Base,
        faction: Faction,
        kind: Target,
        flags: Flags,
        max_health: u32,
        max_shields: u32,
        weapons: [Option<Weapon>; 3],
        armor: Armor,
        cost: Cost,
        size: Real,
        move_speed: Real,
    ) -> Self {
        let hp = Real::const_from_int(max_health as i32);
        let sh = Real::const_from_int(max_shields as i32);
        let shield_regen = if matches!(faction, Faction::Protoss) {
            SHIELD_RECHARGE_RATE
            // Real::from_bits(0)
        } else {
            Real::from_bits(0)
        };

        let health_regen = if matches!(faction, Faction::Zerg) {
            ZERG_REGEN
        } else {
            Real::from_bits(0)
        };

        Self {
            base,
            faction,
            kind,
            flags,
            max_health: hp,
            health: hp,
            health_regen,
            max_shields: sh,
            shields: sh,
            shield_regen,
            shield_regen_delay: SHIELD_RECHARGE_DELAY,
            weapons,
            armor,
            cost,
            size,
            move_speed,
            curr_target: None,
            state: State::Wait,

            death_timestamp: None,
            last_damaged: None,
            damage_dealt: Real::const_from_int(0),
            overkill: Real::const_from_int(0),
        }
    }

    // I can't decide if this is disgusting or sick as fuck
    builder_fn! {base, Base}
    builder_fn! {faction, Faction}
    builder_fn! {kind, Target}
    builder_fn! {flags, Flags}
    builder_fn! {max_health, Real}
    builder_fn! {max_shields, Real}
    pub const fn with_weapon(mut self, weapon: Option<Weapon>, idx: usize) -> Self {
        self.weapons[idx] = weapon;
        self
    }
    builder_fn! {cost, build_time, Real}
    builder_fn! {health_regen, Real}

    // rust please...
    const ZEALOT_GW: Real = duration!(38.0);
    const SENTRY_BOTH: Real = duration!(32.0);
    const STALKER_GW: Real = duration!(42.0);
    const TEMPLAR_GW: Real = duration!(55.0);

    // WG at the start because WG and GW is a recipe for disaster
    const WG_ZEALOT: Real = duration!(28);
    const WG_STALKER: Real = Self::SENTRY_BOTH;
    const WG_TEMPLAR: Real = duration!(45.0);

    pub const fn via_gateway(self) -> Self {
        match self.base {
            Base::Zealot => self.with_build_time(Self::ZEALOT_GW),
            Base::Adept | Base::Stalker => self.with_build_time(Self::STALKER_GW),
            Base::Sentry => self.with_build_time(Self::SENTRY_BOTH),
            Base::HighTemplar | Base::DarkTemplar => self.with_build_time(Self::TEMPLAR_GW),
            _ => self,
        }
    }

    pub const fn via_warpgate(self) -> Self {
        match self.base {
            Base::Adept | Base::Zealot => self.with_build_time(Self::WG_ZEALOT),
            Base::Stalker | Base::Sentry => self.with_build_time(Self::WG_STALKER),
            Base::HighTemplar | Base::DarkTemplar => self.with_build_time(Self::WG_TEMPLAR),
            _ => self,
        }
    }

    // TODO
    const FAST_WARPIN: Real = const_real!(5.0);
    // TODO
    const SLOW_WARPIN: Real = const_real!(16.0);

    /// Modifies build time to include warpgate (if applicable) + fast warpin duration
    pub const fn with_fast_warpin(mut self) -> Self {
        self = self.via_warpgate();
        let t = self.cost.build_time;
        self.with_build_time(t.saturating_add(Self::FAST_WARPIN))
    }

    /// Modifies build time to include warpgate (if applicable) + slow warpin duration
    pub const fn with_slow_warpin(mut self) -> Self {
        self = self.via_warpgate();
        let t = self.cost.build_time;
        self.with_build_time(t.saturating_add(Self::SLOW_WARPIN))
    }

    pub const fn with_chronoboost(self) -> Self {
        let val = self.cost.build_time.saturating_mul(CHRONOBOOST_MOD);
        self.with_build_time(val)
    }

    pub fn try_get_weapon(&self, target: &Unit) -> Option<&Weapon> {
        self.weapons.iter().find_map(|w| w.as_ref().filter(|x| x.can_hit(target.kind)))
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn has_armor_type(&self, flag: Flag) -> bool {
        self.flags.armor == flag
    }

    pub fn has_taxonomy(&self, flag: Flag) -> bool {
        self.flags.taxonomy == flag || self.flags.taxonomy == Flag::Both
    }

    pub const fn is_massive(&self) -> bool {
        self.flags.massive
    }

    pub const fn is_psionic(&self) -> bool {
        self.flags.psionic
    }

    pub const fn is_structure(&self) -> bool {
        self.flags.structure
    }

    pub const fn is_heroic(&self) -> bool {
        self.flags.heroic
    }

    /// Calculates the effective DPS, given a duration. Relies on the unit's internal `damage_dealt`
    /// see also: `ideal_dps()`
    pub fn effective_dps(&self, time: Real) -> Real {
        self.damage_dealt / time
    }

    pub fn ideal_dps(&self, weapon_idx: usize, with_bonus: bool) -> Range<Real> {
        self.weapons[weapon_idx]
            .as_ref()
            .map(|x| x.dps(with_bonus))
            .unwrap_or(real!(0)..real!(0))
    }
}
