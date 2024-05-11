use crate::*;

#[derive(Debug, Clone, Default)]
pub struct TTKStats {
    pub time: f32,
    pub shots: u32,
    pub time_shield: Option<f32>,
    pub shots_shield: Option<u32>,
    pub dmg_per_shot: f32,
    pub shield_dmg_per_shot: Option<f32>,
    pub total_damage: f32,
    pub effective_dps: f32,
    pub dmg_blocked: f32,
    pub healing: f32,
    pub overkill: f32,
}

// pub fn ttk(attacker: &Unit, defender: &Unit) -> Option<TTKStats> {
//     let weapon = attacker.try_get_weapon(defender)?;
//     let mut stats = TTKStats::default();
//     // starcraft 2's tickrate is 16/s in blizzard time, or 22.4 in real seconds
//     let tick: f32 = 1.0 / 22.4;
//     let mut attack_cd = attacker.attack.cooldown;
//     let reset = attack_cd;
//     let mut time_shield = 0.0;
//     let mut shots_shield = 0;

//     let base_damage = attacker.get_damage(&defender.flags);

//     let mut health = defender.defense.health;
//     let zerg_regen = match defender.faction {
//         Faction::Zerg => 0.38 * tick,
//         _ => 0.0,
//     };

//     if defender.faction == Faction::Protoss {
//         let mut shields = defender.defense.shields;

//         let shield_damage =
//             ((base_damage - defender.defense.shield_armor) * attacker.attack.num_attacks).max(0.5);
//         stats.shield_dmg_per_shot = Some(shield_damage);

//         while shields >= 0.0 {
//             attack_cd -= tick;
//             time_shield += tick;

//             if attack_cd <= 0.0 {
//                 shields -= shield_damage;
//                 stats.total_damage += shield_damage;

//                 attack_cd = reset;
//                 shots_shield += 1;

//                 if shields <= 0.0 {
//                     let spillover = (shields.abs() - defender.defense.armor).max(0.5);
//                     health -= spillover;

//                     stats.total_damage += shields; // if it's below zero, undo the spillover
//                     stats.total_damage += spillover; // and add the correct spillover amount back in

//                     break;
//                 }
//             }
//         }
//         stats.time_shield = Some(time_shield);
//         stats.shots_shield = Some(shots_shield);

//         stats.shots = shots_shield;
//         stats.time = time_shield;

//         stats.dmg_blocked = (base_damage - shield_damage) * shots_shield as f32;
//     }

//     let health_damage = ((base_damage - defender.defense.armor) * attacker.attack.num_attacks).max(0.5);
//     stats.dmg_per_shot = health_damage;

//     while health >= 0.0 {
//         attack_cd -= tick;
//         stats.time += tick;

//         health += zerg_regen;
//         stats.healing += zerg_regen;

//         if attack_cd <= 0.0 {
//             health -= health_damage;
//             stats.total_damage += health_damage;

//             attack_cd = reset;
//             stats.shots += 1;

//             if health <= 0.0 {
//                 stats.total_damage += health; // remove any overkill damage
//                 stats.overkill = health.abs();
//                 break;
//             }
//         }
//     }

//     stats.effective_dps = stats.total_damage / stats.time;

//     stats.dmg_blocked +=
//         (base_damage - health_damage) * (stats.shots - stats.shots_shield.unwrap_or(0)) as f32;
//     stats
// }
