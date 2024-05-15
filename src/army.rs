use std::collections::hash_map::Entry;

use crate::{unit::Cost, *};
use fxhash::FxHashMap;
use rand::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Army {
    pub id: usize,
    pub units: Vec<Unit>,
    pub attack_cds: Vec<Real>,
    pub projectiles: Vec<Projectile>,
}

impl Army {
    pub fn reset(&mut self) {
        self.units.iter_mut().for_each(|u| {
            u.health = u.max_health;
            u.shields = u.max_shields;
            u.curr_target = None;
            u.death_timestamp = None;
            u.last_damaged = None;
            u.damage_dealt = const_real!(0);
            u.overkill = const_real!(0);
            u.state = State::Wait;
        });
        self.attack_cds.fill(const_real!(0));
        self.projectiles.clear();
    }

    pub fn add_unit(&mut self, unit: Unit, count: usize) {
        for _ in 0..count {
            self.units.push(unit.clone());
            self.attack_cds.push(const_real!(0));
        }
    }

    pub(crate) fn heal(&mut self, time: Real) {
        self.units.iter_mut().for_each(|u| {
            if u.is_alive() {
                u.health = u.max_health.min(u.health + (u.health_regen * TICK));
                if u.last_damaged
                    .is_some_and(|t| time - t > u.shield_regen_delay)
                {
                    u.shields = u.max_shields.min(u.shields + (u.shield_regen * TICK));
                }
            }
        });
    }

    /// Any units with no target or a dead target swap
    pub(crate) fn acquire_targets(&mut self, opnt: &mut Army, rng: &mut StdRng) {
        for unit in &mut self.units {
            if unit.curr_target.is_some_and(|x| opnt.units[x].is_dead()) {
                unit.curr_target = None;
            }

            if unit.curr_target.is_none() {
                let mut handle = rng.gen_range(0..opnt.units.len());
                while opnt.units[handle].is_dead()
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
