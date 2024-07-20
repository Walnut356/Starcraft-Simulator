use crate::{const_real, unit::Unit, Real};

#[derive(Debug, Clone)]
pub enum BasicUpgrades {
    Generic { weapons: u8, armor: u8 },
    Protoss { ground_weapons: u8, ground_armor: u8, air_weapons: u8, air_armor: u8, shields: u8},
    Terran { infantry_weapons: u8, infantry_armor: u8, vehicle_weapons: u8, ship_weapons: u8, plating: u8},
    Zerg { melee_weapons: u8, ranged_weapons: u8, ground_armor: u8, air_weapons: u8, air_armor: u8 },
}

impl Default for BasicUpgrades {
    fn default() -> Self {
        Self::Generic { weapons: 0, armor: 0 }
    }
}

impl Unit {

}
