use std::mem::size_of;

use effect::Effect;

use super::*;

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

impl Unit {
    // I can't decide if this is disgusting or sick as fuck
    builder_fn! {base, Base}
    builder_fn! {faction, Faction}
    builder_fn! {collision, Collision}
    builder_fn! {flags, Flags}
    builder_fn! {hull, Health}
    pub const fn with_weapon(mut self, weapon: Option<Weapon>, idx: usize) -> Self {
        self.weapons[idx] = weapon;
        self
    }
    builder_fn! {cost, build_time, Real}
    builder_fn! {shields, Health}

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
        self.with_build_time(t.const_add(Self::FAST_WARPIN))
    }

    /// Modifies build time to include warpgate (if applicable) + slow warpin duration
    pub const fn with_slow_warpin(mut self) -> Self {
        self = self.via_warpgate();
        let t = self.cost.build_time;
        self.with_build_time(t.const_add(Self::SLOW_WARPIN))
    }

    pub const fn with_chronoboost(self) -> Self {
        let val = self.cost.build_time.const_mul(CHRONOBOOST_MOD);
        self.with_build_time(val)
    }

    pub const fn with_concussive_shell(mut self, weapon_idx: usize) -> Self {
        self.weapons[0] = match self.weapons[0].as_ref() {
            Some(x) => {
                let wep = Weapon {
                    effect: Some(Effect::CONCUSSIVE),
                    ..*x
                };
                Some(wep)
            },
            None => panic!("Cannot give concussive shell to unit without weapon"),
        };

        self
    }

    pub fn with_glaives(mut self) -> Self {
        self.weapons[0].as_mut().unwrap().attack_speed /= const_real!(1.45);
        self
    }

    pub fn with_combat_shields(mut self) -> Self {
        self.hull.max = const_real!(55);
        self
    }
}