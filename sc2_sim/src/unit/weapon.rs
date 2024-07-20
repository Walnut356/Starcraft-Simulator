// use std::ops::{Range, RangeInclusive};

use effect::Effect;
use rand::prelude::{Rng, StdRng};
use unit::{Collision, Flag, Unit};

use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum SearchType {
    Single,
    Bounce {
        count: Real,
        dmg_mod: Real,
    },
    Zone {
        arc: Real,
        effect: &'static [SplashZone],
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Multihit {
    Single,
    Instant(Real),
    TimeOffset(&'static [Real]),
    PosOffset(&'static [Real]),
}

impl Multihit {
    pub fn multiplier(&self) -> Real {
        match *self {
            Multihit::Single => const_real!(1),
            Multihit::Instant(x) => x,
            Multihit::TimeOffset(x) => real!(x.len()),
            Multihit::PosOffset(x) => real!(x.len()),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SplashZone {
    pub radius: Real,
    pub dmg_mod: Real,
}

#[derive(Debug, Clone, Copy)]
pub enum WeaponKind {
    Melee,
    /// Ranged damage without a missile (e.g. marine)
    Hitscan,
    /// Ranged weapon with a missile (e.g. stalker)
    Projectile,
}

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    /// Prioritizes based on distance to target
    Normal,
    /// Prioritizes targets that take the least amount of turning to hit
    LeastAngle,
    /// Prioritizes units closer to the previous target
    LeastDist,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub kind: WeaponKind,
    pub damage: Real,
    pub multihit: Multihit,
    pub search: SearchType,
    pub attack_speed: Real,
    pub bonus_damage: Real,
    pub bonus_vs: Option<Flag>,
    pub can_target: Collision,
    /// The amount that units damage increases with each upgrade, in the form [damage_up, bonus_up].
    /// If the unit does not deal bonus damage, bonus_up should always be 0.0
    pub upgrade_inc: [Real; 2],
    pub range: RangeInclusive<Real>,
    /// Once a unit is already attacking, this field determines how far the target unit must move
    /// before they are considered "out of range".
    pub range_slop: Real,
    /// If the unit's orientation relative to their target is less than or equal to this amount
    /// the unit is allowed to shoot. Typically set to 0 so the unit must look directly at their
    /// target
    pub arc: Real,
    /// Similar to range slop. Once a unit is already attacking, this field determines how far the
    /// target unit must move before the unit is no longer considered "looking at" the target
    pub arc_slop: Real,
    /// The amount of unactionable time after an attack is fired
    pub backswing: Real,
    /// The duration of the "windup" time of the attack. For hitscan attacks, damage point applies
    /// the damage instantly. For projectile attacks, the projectile is spanwed at this point
    pub damage_point: Real,
    pub random_delay: RangeInclusive<Real>,
    pub priority: Priority,
    pub effect: Option<Effect>,
}

impl Weapon {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        damage: u32,
        kind: WeaponKind,
        multihit: Multihit,
        attack_speed: Real,
        bonus_damage: u32,
        bonus_vs: Option<Flag>,
        can_target: Collision,
        upgrade_inc: [u32; 2],
        range: RangeInclusive<Real>,
        backswing: Real,
        damage_point: Real,
    ) -> Self {
        Self {
            damage: Real::from_i32(damage as i32),
            kind,
            multihit,
            attack_speed,
            bonus_damage: Real::from_i32(bonus_damage as i32),
            bonus_vs,
            can_target,
            upgrade_inc: [
                Real::from_i32(upgrade_inc[0] as i32),
                Real::from_i32(upgrade_inc[1] as i32),
            ],
            range,
            range_slop: DEFAULT_RANGE_SLOP,
            arc: const_real!(0),
            arc_slop: DEFAULT_ARC_SLOP,
            backswing,
            damage_point,
            random_delay: RANDOM_DELAY_RANGE,
            priority: Priority::Normal,
            search: SearchType::Single,
            effect: None,
        }
    }

    pub const fn scan_range(&self) -> Real {
        self.range.end().const_add(const_real!(0.5))
    }

    pub fn can_hit(&self, target: Collision) -> bool {
        match self.can_target {
            Collision::None => false,
            Collision::Both => true,
            _ => self.can_target == target,
        }
    }

    pub fn set_upgrade(&mut self, level: Real) {
        self.damage += self.upgrade_inc[0] * level;
        self.bonus_damage += self.upgrade_inc[1] * level;
    }

    pub fn is_melee(&self) -> bool {
        self.range.end() < 1.0
    }

    pub fn get_damage(&self, target: &Unit) -> Real {
        let mut dmg = self.damage;
        if self.bonus_vs.is_some_and(|b| target.has_flag(b)) {
            dmg += self.bonus_damage;
        }

        dmg -= target.hull.armor;

        dmg
    }

    /// Takes the current RNG state, returns a value in the range of `self.random_delay`
    ///
    /// see also: `get_cooldown()`
    pub fn get_delay(&self, rng: &mut StdRng) -> Real {
        Real::from_bits(
            rng.gen_range(self.random_delay.start().as_bits()..self.random_delay.end().as_bits()),
        )
    }

    /// Takes the current RNG state, returns `self.attack_speed + self.get_delay(rng)`
    pub fn get_cooldown(&self, rng: &mut StdRng) -> Real {
        self.attack_speed + self.get_delay(rng)
    }

    pub fn get_shield_damage(&self, target: &Unit) -> Real {
        let mut dmg = self.damage;
        if self.bonus_vs.is_some_and(|b| target.has_flag(b)) {
            dmg += self.bonus_damage;
        }

        dmg -= target.shields.armor;

        dmg
    }

    pub fn dps(&self, with_bonus: bool) -> std::ops::Range<Real> {
        let dmg = if with_bonus {
            self.damage + self.bonus_damage
        } else {
            self.damage
        };

        ((dmg * self.multihit.multiplier()) / (self.attack_speed + RANDOM_DELAY_MAX))
            ..((dmg * self.multihit.multiplier()) / (self.attack_speed + RANDOM_DELAY_MIN))
    }
}

#[derive(Debug, Clone)]
pub struct Projectile {
    pub timer: Real,
    pub source: u32,
    pub target: u32,
}

impl Projectile {
    pub fn new(source: usize, target: usize, range: Real, time: Real) -> Self {
        Self {
            timer: (range / DEFAULT_PROJECTILE_SPEED) + time,
            source: source as u32,
            target: target as u32,
        }
    }
}
