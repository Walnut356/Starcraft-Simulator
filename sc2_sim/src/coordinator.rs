use std::collections::HashMap;

use crate::*;
use army::{ActionState, Army};
use effect::{Affects, AoE};
use itertools::Itertools;
use rand::prelude::*;
use strum::Display;

macro_rules! unit_from_handle {
    ($army:expr, $handle:expr) => {
        &$army.base_units[&$army.units[$handle as usize].base]
    };
}

macro_rules! unit_from_base {
    ($army:expr, $base:expr) => {
        &$army.base_units[&$base]
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Team {
    Team1,
    Team2,
}

use unit::{Base, Cost, Multihit, Projectile, Weapon, WeaponKind};
use Team::*;

use self::army::{State, Tracker};

#[derive(Debug, Clone)]
pub struct Coordinator {
    pub a1: Army,
    pub a2: Army,
    pub time: Real,
    pub rng: StdRng,
    pub seed: u64,
}

/*
    the struture of this feels a bit silly, but it basically resolves a bunch of ownership issues
    that I just don't want to think about, nor do I want to delegate them to interior mutability.
    there's a bit of unsafe, but we're mostly working with runtime immutable data here (and there's
    not a great way to communicate that to rust without a bunch of other annoying nonsense), so we
    should be fine.
*/
impl Coordinator {
    pub fn reset(&mut self) {
        self.a1.reset();
        self.a2.reset();
        self.time = const_real!(0);
    }

    pub fn seed_rng(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
        self.seed = seed;
    }

    pub fn randomize_seed(&mut self) -> u64 {
        let seed = rand::thread_rng().gen();
        self.rng = StdRng::seed_from_u64(seed);
        self.seed = seed;
        seed
    }

    pub fn units_left(&self, team: Team) -> usize {
        match team {
            Team1 => self
                .a1
                .units
                .iter()
                .map(|u| if u.is_alive() { 1 } else { 0 })
                .sum(),
            Team2 => self
                .a2
                .units
                .iter()
                .map(|u| if u.is_alive() { 1 } else { 0 })
                .sum(),
        }
    }

    pub fn simulate(&mut self) -> Outcome {
        let mut a1_units = self.a1.units.len();
        let mut a2_units = self.a2.units.len();

        while a1_units != 0 && a2_units != 0 {
            self.acquire_targets();
            self.heal();
            // self.tick_weapons();
            self.attack();
            self.tick_projectiles();

            self.time += TICK;
            a1_units = self.units_left(Team1);
            a2_units = self.units_left(Team2);
        }

        while !self.a1.projectiles.is_empty() && !self.a2.projectiles.is_empty() {
            self.heal();
            self.tick_projectiles();
            self.time += TICK;
            a1_units = self.units_left(Team1);
            a2_units = self.units_left(Team2);
        }

        Outcome {
            winner: match (a1_units == 0, a2_units == 0) {
                (true, true) => None,
                (true, false) => Some(Team::Team2),
                (false, true) => Some(Team::Team1),
                (_, _) => panic!("Unreachable"),
            },
            inner: self,
        }
    }

    // for the following few functions, it's easier to delegat to the `Army` impls since they each
    // require mutable references which would break if we did this iteratively, and I don't want
    // 2 full copies of the internal laying around.
    fn acquire_targets(&mut self) {
        self.a1.acquire_targets(&mut self.a2, &mut self.rng);
        self.a2.acquire_targets(&mut self.a1, &mut self.rng);
    }

    fn heal(&mut self) {
        self.a1.heal(self.time);
        self.a2.heal(self.time);
    }

    // fn tick_weapons(&mut self) {
    //     self.t1.tick_weapons();
    //     self.t2.tick_weapons();
    // }

    fn tick_projectiles(&mut self) {
        let mut _inner = |a1: &mut Army, a2: &mut Army| {
            let mut i = 0;

            while i < a1.projectiles.len() {
                if a1.projectiles[i].timer < self.time {
                    let p = a1.projectiles.swap_remove(i);

                    let base = &a1.base_units[&a1.units[p.source as usize].base];
                    let weapon = base
                        .try_get_weapon(a2.unit_from_handle(p.target))
                        .expect("Cannot fire projectile without weapon");
                    Coordinator::apply_damage(
                        &mut a1.trackers[p.source as usize],
                        p.target,
                        a2,
                        weapon,
                        self.time,
                    );

                    continue;
                }

                i += 1;
            }
        };

        _inner(&mut self.a1, &mut self.a2);
        _inner(&mut self.a2, &mut self.a1);
    }

    fn attack(&mut self) {
        // eliminates code duplication. I use a closure so it captures (and partial borrows) self
        let mut _inner = |attk: &mut Army, dfnd: &mut Army| {
            for (u_handle, unit) in attk.units.iter_mut().enumerate() {
                if unit.is_dead() || unit.target.is_none() || !unit.can_attack {
                    continue;
                }

                // launch attack if we've finished backswing
                if let ActionState::DmgPoint(timestamp, next_dmgpoint_idx) = unit.state {
                    // backswing is a timestamp
                    if timestamp > self.time {
                        continue;
                    }

                    let t_handle = unit
                        .target
                        .expect("Cannot be in backswing without a target");
                    let target = unit_from_handle!(dfnd, t_handle);

                    let weapon = unit_from_base!(attk, unit.base)
                        .try_get_weapon(target)
                        .expect("Can only enter State::Backswing with a weapon");

                    match weapon.kind {
                        WeaponKind::Projectile => attk.projectiles.push(Projectile::new(
                            u_handle,
                            t_handle as usize,
                            weapon.range.end(),
                            self.time,
                        )),
                        _ => Coordinator::apply_damage(
                            &mut attk.trackers[u_handle],
                            t_handle,
                            dfnd,
                            weapon,
                            self.time,
                        ),
                    }

                    unit.state = match weapon.multihit {
                        Multihit::TimeOffset(offsets)
                            if (next_dmgpoint_idx as usize) < offsets.len() =>
                        {
                            ActionState::DmgPoint(
                                offsets[next_dmgpoint_idx as usize] + self.time,
                                next_dmgpoint_idx + 1,
                            )
                        }
                        _ => ActionState::Attack,
                    };
                }

                if unit.attack_cd > self.time {
                    continue;
                }

                // not dead, can attack, has target, not in the middle of a multihit:

                let t_handle = unit.target.expect("Cannot attack without a target");
                let target = unit_from_handle!(dfnd, t_handle);
                let Some(weapon) = unit_from_base!(attk, unit.base).try_get_weapon(target) else {
                    continue;
                };

                unit.attack_cd = self.time + weapon.get_cooldown(&mut self.rng);
                unit.state = ActionState::DmgPoint(weapon.damage_point + self.time, 0);
            }
        };

        _inner(&mut self.a1, &mut self.a2);
        _inner(&mut self.a2, &mut self.a1);
    }

    // Nonsense necessary because the callsite requires a mutable borrow on an `Army` to iterate
    // over the units, so we can't call any methods that require references to that same `Army`.
    // That means we're limited to freestanding and associated functions.
    fn apply_damage(
        u_tracker: &mut Tracker,
        target: u32,
        dfnd: &mut Army,
        weapon: &Weapon,
        time: Real,
    ) {
        let mut hull_damage = Real::default();
        let mut overkill = Real::default();

        // let upgrade_bonus = match

        let t_base = unit_from_handle!(dfnd, target);
        let t = &mut dfnd.units[target as usize];

        if t.shields != 0 {
            let mut shield_damage = MIN_DAMAGE.max(weapon.get_shield_damage(t_base));
            if let Multihit::Instant(x) = weapon.multihit {
                shield_damage *= x;
            }
            t.shields -= shield_damage;
            u_tracker.damage_dealt += shield_damage;

            if t.shields < 0 {
                // yes carryover damage has the hull armor applied, I checked this in-game
                let spillover = t.shields.abs() - t_base.hull.armor;
                t.hull -= spillover;
                hull_damage += spillover;

                // we don't ever want negative shields past the spillover damage, as shields can
                // be regenerated and we want to begin the healing from 0
                t.shields = real!(0);
            }
        } else {
            hull_damage += MIN_DAMAGE.max(weapon.get_damage(t_base));
            if let Multihit::Instant(x) = weapon.multihit {
                hull_damage *= x;
            }

            t.hull -= hull_damage;
            if t.hull < 0 {
                overkill = t.hull.abs();
                dfnd.trackers[target as usize].death_timestamp = Some(time);
            }
        }

        u_tracker.damage_dealt += hull_damage - overkill;
        u_tracker.overkill += overkill;
        dfnd.units[target as usize].last_damaged = Some(time);
    }

    fn apply_aoe(&mut self, aoe: AoE) {
        let _inner = |army: &mut Army| {
            for (handle, &position) in army.positions.iter().enumerate() {
                if aoe.collides_with(position) {
                    (aoe.effect)(&mut army.units[handle])
                }
            }
        };

        match (aoe.affects, aoe.team) {
            (Affects::Friendly, Team1) | (Affects::Enemy, Team2) => _inner(&mut self.a1),
            (Affects::Friendly, Team2) | (Affects::Enemy, Team1) => _inner(&mut self.a2),
            (Affects::Both, _) => {
                _inner(&mut self.a1);
                _inner(&mut self.a2);
            }
        }
    }
}

impl Default for Coordinator {
    fn default() -> Self {
        Self {
            a1: Default::default(),
            a2: Default::default(),
            time: Default::default(),
            // It's as good a default seed as any
            rng: StdRng::seed_from_u64(17313471783455232199),
            seed: 17313471783455232199,
        }
    }
}

pub struct Outcome<'a> {
    winner: Option<Team>,
    inner: &'a Coordinator,
}

impl<'a> Outcome<'a> {
    pub fn winner(&self) -> Option<Team> {
        self.winner
    }

    pub fn team_1(&self) -> &Army {
        &self.inner.a1
    }

    pub fn team_2(&self) -> &Army {
        &self.inner.a2
    }

    pub fn duration(&self) -> Real {
        self.inner.time
    }

    pub fn total_cost(&self, team: Team) -> Cost {
        match team {
            Team1 => self.inner.a1.total_cost(),
            Team2 => self.inner.a2.total_cost(),
        }
    }

    pub fn cost_difference(&self, team: Team) -> Cost {
        let (a, b) = match team {
            Team::Team1 => (&self.inner.a1, &self.inner.a2),
            Team::Team2 => (&self.inner.a2, &self.inner.a1),
        };

        a.total_cost() - b.total_cost()
    }

    /// Returns the winner's resources lost. Since the loser must have lost all their units, their
    /// resources lost will always be the same as their `.total_cost()`. If the fight was a draw,
    /// this function returns None
    pub fn resources_lost(&self) -> Option<Cost> {
        if let Some(w) = self.winner {
            let a = match w {
                Team1 => &self.inner.a1,
                Team2 => &self.inner.a2,
            };

            return Some(
                a.units
                    .iter()
                    .filter_map(|u| u.is_dead().then_some(unit_from_base!(a, u.base).cost))
                    .sum(),
            );
        }

        None
    }

    /// Returns the winner's units remaining. If the fight is a draw, returns Cost::default()
    pub fn cost_units_remaining(&self) -> Cost {
        if let Some(w) = self.winner {
            let a = match w {
                Team1 => &self.inner.a1,
                Team2 => &self.inner.a2,
            };

            return a
                .units
                .iter()
                .filter_map(|u: &State| u.is_alive().then_some(unit_from_base!(a, u.base).cost))
                .sum();
        }

        Cost::default()
    }

    pub fn units_remaining(&self) -> HashMap<Base, usize> {
        if let Some(w) = self.winner {
            let a = match w {
                Team1 => &self.inner.a1,
                Team2 => &self.inner.a2,
            };

            return a
                .units
                .iter()
                .filter_map(|u| u.is_alive().then_some(unit_from_base!(a, u.base).base))
                .counts();
        }

        HashMap::new()
    }

    pub fn units_lost(&self) -> HashMap<Base, usize> {
        if let Some(w) = self.winner {
            let a = match w {
                Team1 => &self.inner.a1,
                Team2 => &self.inner.a2,
            };

            return a
                .units
                .iter()
                .enumerate()
                .filter_map(|(i, u)| u.is_dead().then_some(a.unit_from_handle(i as u32).base))
                .counts();
        }

        HashMap::new()
    }
}
