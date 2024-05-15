use crate::*;

impl Unit {
    pub fn with_glaives(mut self) -> Self {
        self.weapons[0].as_mut().unwrap().attack_speed /= const_real!(1.45);
        self
    }

    pub fn with_combat_shields(mut self) -> Self {
        self.max_health = const_real!(55);
        self.health = const_real!(55);
        self
    }
}
