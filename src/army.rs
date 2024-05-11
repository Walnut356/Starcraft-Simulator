use std::collections::hash_map::Entry;

use crate::{unit::Cost, *};
use fxhash::FxHashMap;
use rand::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Army {
    pub id: usize,
    pub units: Vec<Unit>,
    pub projectiles: Vec<Projectile>,
}

impl Army {
    pub fn add_unit(&mut self, unit: Unit, count: usize) {
        for _ in 0..count {
            self.units.push(unit.clone());
        }
    }

    pub(crate) fn tick_weapons(&mut self) {
        for unit in &mut self.units {
            unit.attack_cd -= TICK;
        }
    }

    pub(crate) fn heal(&mut self, time: Real) {
        self.units.iter_mut().for_each(|u| {
            u.health = u.max_health.min(u.health + (u.health_regen * TICK));
            if u.last_damaged
                .is_some_and(|t| time - t > u.shield_regen_delay)
            {
                u.shields = u.max_shields.min(u.shields + (u.shield_regen * TICK));
            }
        });
    }

    /// Any units with no target or a dead target swap
    pub(crate) fn acquire_targets(&mut self, opnt: &mut Army, rng: &mut StdRng) {
        for unit in &mut self.units {
            if unit.curr_target.is_some_and(|x| opnt.units[x].health <= 0) {
                unit.curr_target = None;
            }

            if unit.curr_target.is_none() {
                let mut handle = rng.gen_range(0..opnt.units.len());
                while opnt.units[handle].health <= 0
                    || unit.try_get_weapon(&opnt.units[handle]).is_none()
                {
                    handle = rng.gen_range(0..opnt.units.len());
                }

                unit.curr_target = Some(handle);
                unit.state = State::Attack;
            }
        }
    }

    pub fn total_cost(&self) -> Cost {
        self.units.iter().map(|x| x.cost).sum()
    }

    pub fn total_health(&self) -> Real {
        self.units
            .iter()
            .map(|x| x.max_health + x.max_shields)
            .sum()
    }

    pub fn total_health_curr(&self) -> Real {
        self.units.iter().map(|x| x.health + x.shields).sum()
    }

    pub fn damage_dealt(&self) -> Real {
        self.units.iter().map(|x| x.damage_dealt).sum()
    }
}
