use std::ops::Range;

use rand::prelude::{Rng, StdRng};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjType {
    Projectile(Real),
    Hitscan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Multihit {
    Single,
    Instant2,
    // lib, banshee, viking, zealot, queen, probably more
    Offset2(Real),
    // thor
    Offset4([Real; 3]),
}

impl Multihit {
    pub fn multiplier(&self) -> Real {
        match *self {
            Multihit::Single => const_real!(1),
            Multihit::Instant2 | Multihit::Offset2(_) => const_real!(2),
            Multihit::Offset4(_) => const_real!(4),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub damage: Real,
    pub multihit: Multihit,
    pub attack_speed: Real, // can be negative due to armor shred
    pub bonus_damage: Real,
    pub bonus_vs: Option<Flag>,
    pub can_target: Target,
    /// The amount that units damage increases with each upgrade, in the form [damage_up, bonus_up].
    /// If the unit does not deal bonus damage, bonus_up should always be 0.0
    pub upgrade_inc: [Real; 2],
    pub range: Real,
    pub projectile: ProjType,
    pub backswing: Real,
    pub damage_point: Real,
    pub random_delay: Range<Real>,
}

impl Weapon {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        damage: u32,
        multihit: Multihit,
        attack_speed: Real,
        bonus_damage: u32,
        bonus_vs: Option<Flag>,
        can_target: Target,
        upgrade_inc: [u32; 2],
        range: Real,
        projectile: ProjType,
        backswing: Real,
        damage_point: Real,
    ) -> Self {
        Self {
            damage: Real::const_from_int(damage as i32),
            multihit,
            attack_speed,
            bonus_damage: Real::const_from_int(bonus_damage as i32),
            bonus_vs,
            can_target,
            upgrade_inc: [
                Real::const_from_int(upgrade_inc[0] as i32),
                Real::const_from_int(upgrade_inc[1] as i32),
            ],
            range,
            projectile,
            backswing,
            damage_point,
            random_delay: RANDOM_DELAY_RANGE,
        }
    }

    pub fn can_hit(&self, target: Target) -> bool {
        match self.can_target {
            Target::None => false,
            Target::Both => true,
            _ => self.can_target == target,
        }
    }

    pub fn set_upgrade(&mut self, level: Real) {
        self.damage += self.upgrade_inc[0] * level;
        self.bonus_damage += self.upgrade_inc[1] * level;
    }

    pub fn is_melee(&self) -> bool {
        self.range < 1.0
    }

    pub fn get_damage(&self, target: &Unit) -> Real {
        let mut dmg = self.damage;
        if self.bonus_vs.is_some_and(|b| target.flags.contains(b)) {
            dmg += self.bonus_damage;
        }

        dmg -= target.armor.hull;

        dmg
    }

    pub fn get_delay(&self, rng: &mut StdRng) -> Real {
        Real::from_bits(
            rng.gen_range(self.random_delay.start.to_bits()..self.random_delay.end.to_bits()),
        )
    }

    pub fn get_shield_damage(&self, target: &Unit) -> Real {
        let mut dmg = self.damage;
        if self.bonus_vs.is_some_and(|b| target.flags.contains(b)) {
            dmg += self.bonus_damage;
        }

        dmg -= target.armor.shield;

        dmg
    }

    pub fn dps(&self, with_bonus: bool) -> Range<Real> {
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
    pub source: usize,
    pub target: usize,
}

impl Projectile {
    pub fn new(source: usize, target: usize, range: Real, proj_speed: Real, time: Real) -> Self {
        Self {
            timer: (range / proj_speed) + time,
            source,
            target,
        }
    }
}
