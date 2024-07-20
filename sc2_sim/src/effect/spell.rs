use crate::{army::State, coordinator::Team, CollCircle, Pos, Real};

use super::Affects;




#[derive(Debug, Clone)]
pub struct AoE {
    pub circle: CollCircle,
    pub expires: Real,
    pub team: Team,
    pub affects: Affects,
    pub effect: fn(&mut State),
}

impl AoE {
    pub fn collides_with(&self, other: CollCircle) -> bool {
        self.circle.collides_with(other)
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.circle.pos = pos;
    }
}

// #[derive(Debug, Clone)]
// pub struct DoT {
//     pub aoe: AoE,
//     pub tick_rate:
// }



/*
all aoe spells:
storm
guardian shield
ff
disruptor
revelation(?)
stasis
time warp
cloaking field
reaper grenade
emp
nuke?
widowmine
anti-armor missile
microbial shroud
bile
blinding cloud
parasitic bomb
*/