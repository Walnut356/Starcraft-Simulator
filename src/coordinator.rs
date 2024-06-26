use std::collections::HashMap;

use crate::*;
use itertools::Itertools;
use rand::prelude::*;
use strum::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Team {
    Team1,
    Team2,
}

use Team::*;

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
                .map(|u| if u.health > 0 { 1 } else { 0 })
                .sum(),
            Team2 => self
                .a2
                .units
                .iter()
                .map(|u| if u.health > 0 { 1 } else { 0 })
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
        let mut i = 0;
        while i < self.a1.projectiles.len() {
            if self.a1.projectiles[i].timer < self.time {
                let p = self.a1.projectiles.swap_remove(i);
                Coordinator::apply_damage(
                    &mut self.a1.units[p.source],
                    &mut self.a2.units[p.target],
                    self.time,
                );

                continue;
            }

            i += 1;
        }

        let mut i = 0;
        while i < self.a2.projectiles.len() {
            if self.a2.projectiles[i].timer < self.time {
                let p = self.a2.projectiles.swap_remove(i);
                Coordinator::apply_damage(
                    &mut self.a2.units[p.source],
                    &mut self.a1.units[p.target],
                    self.time,
                );

                continue;
            }

            i += 1;
        }
    }

    // this one will have 2 separate copies because otherwise there's a bunch of borrowchecker hoops
    // to jump through.
    fn attack(&mut self) {
        let attk = &mut self.a1;
        let dfnd = &mut self.a2;
        for (u_hdl, unit) in attk.units.iter_mut().enumerate() {
            // launch attack if we've finished backswing
            if let State::DmgPoint(x) = unit.state {
                // backswing is a timestamp
                if x > self.time {
                    continue;
                }
                let t_hdl = unit.curr_target.unwrap();
                let target = &mut dfnd.units[t_hdl];
                let weapon = unit
                    .try_get_weapon(target)
                    .expect("Can only enter State::Backswing with a weapon");

                match weapon.projectile {
                    ProjType::Projectile(x) => attk.projectiles.push(Projectile::new(
                        u_hdl,
                        t_hdl,
                        weapon.range,
                        x,
                        self.time,
                    )),
                    ProjType::Hitscan => Coordinator::apply_damage(unit, target, self.time),
                }

                unit.state = State::Attack;
            }

            if attk.attack_cds[u_hdl] > self.time || unit.is_dead() {
                continue;
            }

            if unit.curr_target.is_none() {
                continue;
            }

            // not dead and can attack and has target:

            let t_hdl = unit.curr_target.unwrap();
            let target = &mut dfnd.units[t_hdl];
            let weapon = unit.try_get_weapon(target);
            if weapon.is_none() {
                continue;
            }
            let weapon = weapon.unwrap();

            /*
                multihit attack handling:

                if a unit hasn't attacked yet, their normal weapon attack cooldown is used. If they
                have, their attack cooldown is set to the static "gap" cooldown that occurs between
                the multihits. If they're on the last hit of the multihit, it's set back to the
                attack's default cooldown.

                Multiple damage instances via Multihit::Instant2 are handled in the damage
                calculation by just multiplying the damage by the attack count. It's easier and
                faster to assume that the missiles will hit at the same time rather than
                conditionally spawning 2 of them.
            */
            let (attack_cd, new_state) = match weapon.multihit {
                Multihit::Offset2(x) if unit.state == State::Attack => (x, State::Attack2),
                Multihit::Single | Multihit::Instant2 | Multihit::Offset2(_) => (
                    weapon.attack_speed + weapon.get_delay(&mut self.rng),
                    // damage_point is stored as a timestamp, i.e. "what time will it be when the
                    // damage is applied?")
                    State::DmgPoint(self.time + weapon.damage_point),
                ),
                Multihit::Offset4(x) => match unit.state {
                    State::Attack => (x[0], State::Attack2),
                    State::Attack2 => (x[1], State::Attack3),
                    State::Attack3 => (x[2], State::Attack4),
                    State::Attack4 => (
                        weapon.attack_speed + weapon.get_delay(&mut self.rng),
                        State::Attack,
                    ),
                    _ => panic!("Unreachable"),
                },
            };

            attk.attack_cds[u_hdl] = self.time + attack_cd;
            unit.state = new_state;
        }

        let attk = &mut self.a2;
        let dfnd = &mut self.a1;

        for (u_hdl, unit) in attk.units.iter_mut().enumerate() {
            // launch attack if we've finished backswing
            if let State::DmgPoint(x) = unit.state {
                // backswing is a timestamp
                if x > self.time {
                    continue;
                }
                let t_hdl = unit.curr_target.unwrap();
                let target = &mut dfnd.units[t_hdl];
                let weapon = unit
                    .try_get_weapon(target)
                    .expect("Can only enter State::Backswing with a weapon");

                match weapon.projectile {
                    ProjType::Projectile(x) => attk.projectiles.push(Projectile::new(
                        u_hdl,
                        t_hdl,
                        weapon.range,
                        x,
                        self.time,
                    )),
                    ProjType::Hitscan => Coordinator::apply_damage(unit, target, self.time),
                }

                unit.state = State::Attack;
            }

            if attk.attack_cds[u_hdl] > self.time || unit.is_dead() {
                continue;
            }

            if unit.curr_target.is_none() {
                continue;
            }

            // not dead and can attack and has target:

            let t_hdl = unit.curr_target.unwrap();
            let target = &mut dfnd.units[t_hdl];
            let weapon = unit.try_get_weapon(target);
            if weapon.is_none() {
                continue;
            }
            let weapon = weapon.unwrap();

            /*
                multihit attack handling:

                if a unit hasn't attacked yet, their normal weapon attack cooldown is used. If they
                have, their attack cooldown is set to the static "gap" cooldown that occurs between
                the multihits. If they're on the last hit of the multihit, it's set back to the
                attack's default cooldown.

                Multiple damage instances via Multihit::Instant2 are handled in the damage
                calculation by just multiplying the damage by the attack count. It's easier and
                faster to assume that the missiles will hit at the same time rather than
                conditionally spawning 2 of them.
            */
            let (attack_cd, new_state) = match weapon.multihit {
                Multihit::Offset2(x) if unit.state == State::Attack => (x, State::Attack2),
                Multihit::Single | Multihit::Instant2 | Multihit::Offset2(_) => (
                    weapon.attack_speed + weapon.get_delay(&mut self.rng),
                    // damage_point is stored as a timestamp, i.e. "what time will it be when the
                    // damage is applied?")
                    State::DmgPoint(self.time + weapon.damage_point),
                ),
                Multihit::Offset4(x) => match unit.state {
                    State::Attack => (x[0], State::Attack2),
                    State::Attack2 => (x[1], State::Attack3),
                    State::Attack3 => (x[2], State::Attack4),
                    State::Attack4 => (
                        weapon.attack_speed + weapon.get_delay(&mut self.rng),
                        State::Attack,
                    ),
                    _ => panic!("Unreachable"),
                },
            };

            attk.attack_cds[u_hdl] = self.time + attack_cd;
            unit.state = new_state;
        }
    }

    // Nonsense necessary because the callsite requires a mutable borrow on an `Army` to iterate
    // over the units, so we can't call any methods that require references to that same `Army`.
    // That means we're limited to freestanding and associated functions.
    fn apply_damage(u: &mut Unit, t: &mut Unit, time: Real) {
        let weapon = u.try_get_weapon(t).unwrap();

        let mut hull_damage = Real::default();
        let mut overkill = Real::default();

        if t.shields != 0 {
            let mut shield_damage = MIN_DAMAGE.max(weapon.get_shield_damage(t));
            if weapon.multihit == Multihit::Instant2 {
                shield_damage *= 2;
            }
            t.shields -= shield_damage;
            u.damage_dealt += shield_damage;

            if t.shields < 0 {
                // yes carryover damage has the hull armor applied, I checked this in-game
                let spillover = t.shields.abs() - t.armor.hull;
                t.health -= spillover;
                hull_damage += spillover;

                // we don't ever want negative shields past the spillover damage, as shields can
                // be regenerated and we want to begin the healing from 0
                t.shields = real!(0);
            }
        } else {
            hull_damage += MIN_DAMAGE.max(weapon.get_damage(t));
            if weapon.multihit == Multihit::Instant2 {
                hull_damage *= 2;
            }

            t.health -= hull_damage;
            if t.health < 0 {
                overkill = t.health.abs();
                t.death_timestamp = Some(time);
            }
        }

        u.damage_dealt += hull_damage - overkill;
        u.overkill += overkill;
        t.last_damaged = Some(time);
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
                    .filter_map(|x| x.is_dead().then_some(x.cost))
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
                .filter_map(|x| x.is_alive().then_some(x.cost))
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
                .filter_map(|x| x.is_alive().then_some(x.base))
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
                .filter_map(|x| x.is_dead().then_some(x.base))
                .counts();
        }

        HashMap::new()
    }
}
