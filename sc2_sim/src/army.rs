use crate::{
    effect::{Effect, Stat},
    unit::{Base, BasicUpgrades, Collision, Cost, Projectile, Unit},
    *,
};
use rand::prelude::*;
use utils::unsafe_borrow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ActionState {
    #[default]
    Wait,
    Attack,
    /// Contains the timestamp at which DmgPoint ends, as well as the current attack number
    DmgPoint(Real, u8),
    Move,
    Dead,
    /// Contains the handle of the unit that contains this unit
    Cargo(u32),
}

#[derive(Debug, Clone)]
pub struct State {
    // the extra indirection here hurts performance a little, but improves readability by separating
    // static information from individual unit information
    // a raw pointer is used mostly because I don't want to deal with explicit lifetimes. The pointer
    // is valid for as long as the containing Army is.
    pub base: Base,
    pub state: ActionState,
    pub max_speed: Real,
    pub hull: Real,
    pub shields: Real,
    pub energy: Option<Real>,
    pub target: Option<u32>,
    pub attack_cd: Real,
    pub last_damaged: Option<Real>,
    pub invisible: bool,
    pub burrowed: bool,
    pub move_and_shoot: bool,
    /// for mechanics like stasis
    pub untargetable: bool,
    /// for mechanics like graviton beam or reaper grenade
    pub can_attack: bool,
    pub collision: Collision,
    // pub spells: Vec<Effect>,
    pub effects: Vec<Effect>,
    pub parent: Option<u32>,
}

impl State {
    pub fn new(unit: &Unit) -> Self {
        Self {
            base: unit.base,
            state: ActionState::Wait,
            hull: unit.hull.max,
            shields: unit.shields.max,
            target: None,
            attack_cd: const_real!(0),
            last_damaged: None,
            invisible: false,
            move_and_shoot: unit.base == Base::Phoenix,
            untargetable: false,
            can_attack: unit.weapons[0].is_some() || unit.weapons[1].is_some(),
            energy: (unit.energy_max > 0).then_some(unit.energy_start),
            burrowed: false,
            collision: unit.collision,
            effects: Vec::new(),
            parent: None,
            max_speed: unit.movement.speed,
        }
    }

    fn with_parent(mut self, handle: usize) -> Self {
        self.parent = Some(handle as u32);
        self
    }

    pub fn is_alive(&self) -> bool {
        self.hull > 0
    }

    pub fn is_dead(&self) -> bool {
        self.hull <= 0
    }

    pub fn reset_speed(&mut self) {
        let effects = unsafe_borrow(&self.effects);
        for effect in effects {
            let Effect::StatModTemp {
                stat: Stat::Speed,
                apply,
                remove: _,
                timestamp: _,
            } = effect
            else {
                continue;
            };
            apply(self)
        }
    }

    // pub fn base_stats(& self) -> &'static Unit {
    //     // SAFETY: this is a hack, and definitely an unsafe one if this struct is ever cloned outside
    //     // of the containing army. Saves me from having to worry about lifetimes while i'm
    //     // prototyping though
    //     unsafe { &*self.base_stats }
    // }
}

#[derive(Debug, Clone, Default)]
pub struct Tracker {
    pub damage_dealt: Real,
    pub overkill: Real,
    pub death_timestamp: Option<Real>,
}

#[derive(Debug, Clone, Default)]
pub struct Army {
    pub id: usize,
    pub upgrades: BasicUpgrades,
    pub base_units: Map<Base, Unit>,
    pub units: Vec<State>,
    pub positions: Vec<CollCircle>,
    pub trackers: Vec<Tracker>,
    pub projectiles: Vec<Projectile>,
    pub live_carriers: u32,
    pub live_interceptors: u32,
}

// impl Default for Army {
//     fn default() -> Self {
//         Self {
//             id: Default::default(),
//             base_unit: Default::default(),
//             units: Vec::with_capacity(Self::DEFAULT_UNIT_CAPACITY),
//             trackers: Vec::with_capacity(Self::DEFAULT_UNIT_CAPACITY),
//             projectiles: Vec::with_capacity(Self::DEFAULT_UNIT_CAPACITY),
//         }
//     }
// }

impl Army {
    // const DEFAULT_UNIT_CAPACITY: usize = 128;
    pub fn reset(&mut self) {
        for u in self.units.iter_mut() {
            let base = &self.base_units[&u.base];
            u.hull = base.hull.max;
            u.shields = base.shields.max;
            u.state = ActionState::Wait;
            u.target = None;
            u.attack_cd = const_real!(0);
            u.can_attack = base.weapons[0].is_some() || base.weapons[1].is_some();
            u.energy = (base.energy_max > 0).then_some(base.energy_start);
            u.effects.clear();
        }
        self.positions.iter_mut().for_each(|x| {
            x.pos = Pos {
                x: const_real!(0),
                y: const_real!(0),
            };
        });
        self.projectiles.clear();
        self.trackers.fill(Tracker::default());
    }

    /// Used internally to bump the capacity of all non-projectile vecs in the army
    fn reserve(&mut self, count: usize) {
        self.units.reserve(count);
        self.positions.reserve(count);
        self.trackers.reserve(count);
    }

    /// Adds `count` copies of the specified unit to the army. Only one copy of each `Base` unit is
    /// stored.
    ///
    /// Note: Carriers are added with 8 interceptors
    pub fn add_unit(&mut self, unit: Unit, count: usize) {
        match unit.base {
            Base::Carrier => {
                self.reserve(count + (count * 8));
                self.base_units.insert(Base::Interceptor, Unit::INTERCEPTOR);
            }
            Base::BroodLord => {
                self.reserve(count + (count * 8));
                self.base_units.insert(Base::Broodling, Unit::BROODLING);
            }
            Base::SwarmHost => {
                self.base_units.insert(Base::Locust, Unit::LOCUST);
            }
            _ => self.reserve(count),
        }

        for _ in 0..count {
            self.units.push(State::new(&unit));
            self.positions.push(CollCircle {
                pos: Pos {
                    x: const_real!(0),
                    y: const_real!(0),
                },
                r: unit.size,
                plane: unit.collision,
            });
            if unit.base == Base::Carrier {
                let handle = self.units.len() - 1;
                for _ in 0..8 {
                    self.units
                        .push(State::new(&Unit::INTERCEPTOR).with_parent(handle))
                }
            }
            self.trackers.push(Tracker::default());
        }

        self.base_units.insert(unit.base, unit);
    }

    // /// Adds `count` copies of the specified unit to the army, each containing `cargo.1` copies of `cargo.0`
    // ///
    // /// Useful for things like carriers, medivacs, bunkers, etc.
    // pub fn add_unit_cargo(&mut self, unit: Unit, count: usize, cargo: (Unit, usize)) {
    //     self.reserve(count + (count * cargo.1));
    //     for _ in 0..count {
    //         self.units.push(State::new(&unit));
    //         let handle = self.units.len() - 1;
    //         for _ in 0..cargo.1 {
    //             self.units.push()
    //         }
    //         self.trackers.push(Tracker::default());
    //     }

    //     self.base_units.insert(unit.base, unit);
    // }

    pub fn unit_from_handle<N: TryInto<usize>>(&self, handle: N) -> &Unit
    where
        <N as TryInto<usize>>::Error: std::fmt::Debug,
    {
        // this try_into should be a noop, but at least makes calling this function less annoying
        // let map = &self.base_units;
        // &map[&self.units[handle.try_into().unwrap()].base]

        let get_value = || &self.base_units[&self.units[handle.try_into().unwrap()].base];
        get_value()
    }

    // pub fn unit_from_base(&self, base: Base) -> &Unit {

    // }

    pub fn units(&self) -> &Vec<State> {
        &self.units
    }

    pub(crate) fn heal(&mut self, time: Real) {
        self.units.iter_mut().for_each(|unit| {
            if unit.is_alive() {
                let base = &self.base_units[&unit.base];
                unit.hull = base.hull.max.min(unit.hull + (base.hull.regen * TICK));

                if unit
                    .last_damaged
                    .is_some_and(|t| time - t > const_real!(10))
                {
                    unit.shields = base
                        .shields
                        .max
                        .min(unit.shields + (base.shields.regen * TICK));
                }

                if let Some(x) = unit.energy {
                    unit.energy = Some(base.energy_max.min(x + ENERGY_REGEN))
                }
            }
        });
    }

    /// Any units with no target or a dead target swap
    pub(crate) fn acquire_targets(&mut self, opnt: &mut Army, rng: &mut StdRng) {
        for unit in self.units.iter_mut() {
            if unit
                .target
                .is_some_and(|x| opnt.units[x as usize].is_dead())
            {
                unit.target = None;
            }

            if unit.target.is_none() {
                let mut handle = rng.gen_range(0..opnt.units.len());
                while opnt.units[handle].is_dead()
                    || self.base_units[&unit.base]
                        .try_get_weapon(&opnt.base_units[&opnt.units[handle].base])
                        .is_none()
                {
                    handle = rng.gen_range(0..opnt.units.len());
                }

                unit.target = Some(handle as u32);
                unit.state = ActionState::Attack;
            }
        }
    }

    pub(crate) fn reset_speed(&mut self, handle: usize, stat: Stat) {
        let speed = self.unit_from_handle(handle).movement.speed;
        let state = &mut self.units[handle];
        state.max_speed = speed;
        let effects = unsafe_borrow(&state.effects);

        for effect in effects {
            let Effect::StatModTemp {
                stat: Stat::Speed,
                apply,
                remove: _,
                timestamp: _,
            } = effect
            else {
                continue;
            };
            apply(state)
        }
    }

    pub fn total_cost(&self) -> Cost {
        self.units
            .iter()
            .map(|x| self.base_units[&x.base].cost)
            .sum()
    }

    pub fn total_health(&self) -> Real {
        self.units
            .iter()
            .map(|x| {
                let base = &self.base_units[&x.base];
                base.hull.max + base.shields.max
            })
            .sum()
    }

    pub fn total_health_curr(&self) -> Real {
        self.units.iter().map(|x| x.hull + x.shields).sum()
    }

    pub fn damage_dealt(&self) -> Real {
        self.trackers.iter().map(|x| x.damage_dealt).sum()
    }
}
