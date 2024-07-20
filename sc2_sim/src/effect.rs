mod spell_data;
pub use spell_data::*;
mod spell;
pub use spell::*;
mod attack;
pub use attack::*;

use crate::{army::State, const_real, Real};

#[derive(Debug, Clone, Copy)]
pub enum Affects {
    Friendly,
    Enemy,
    Both,
}

#[derive(Debug, Clone, Copy)]
pub enum Stat {
    Speed,
}

#[derive(Debug, Clone,)]
pub enum Effect {
    StatModOnce { apply: fn(&mut State) },
    StatModTemp { stat: Stat, apply: fn(&mut State), remove: fn(&mut State), timestamp: Real },
    // DelayedStatMod,
    // DamageNegate,
    // CreateEntity,
    // CreateUnit,
    // Search,
}

impl Effect {
    pub const CONCUSSIVE: Self = Self::StatModTemp {
        stat: Stat::Speed,
        apply: |state: &mut State| { state.max_speed *= const_real!(0.5)},
        remove: |state: &mut State| { state.reset_speed() },
        timestamp: const_real!(0),
    };
}
