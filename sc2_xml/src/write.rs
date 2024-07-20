#![allow(unused_must_use)]

use crate::*;
use std::fmt::Write;

pub fn write_units(units: &[(&'static &'static str, &'static Tag)]) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "use crate::{{const_real, duration, rate, unit::*,}};\n\nimpl Unit {{"
    );

    // --------------------------------------- identifiers -------------------------------------- //
    for name in VALID_UNITS {
        let unit = UNIT_MAP.get(name).unwrap();

        let name = match name {
            "LurkerMP" => "Lurker",
            "LurkerMPBurrowed" => "LurkerBurrowed",
            "LurkerMPEgg" => "LurkerEgg",
            "ThorAP" => "ThorAlt",
            "VikingAssault" => "VikingGround",
            "VikingFighter" => "VikingAir",
            "SwarmHostMP" => "SwarmHost",
            "LocustMP" => "Locust",
            "LocustMPFlying" => "LocustFlying",
            // "QueenMP" => "Queen",
            "HellionTank" => "Hellbat",
            "LiberatorAG" => "LiberatorSieged",
            "BroodlingDefault" => "Broodling",

            x => x,
        };

        writeln!(output, "pub const {}: Unit = Unit {{", name.to_uppercase());
        writeln!(output, "base: Base::{name},");

        let faction = match unit.val_of("Race") {
            "Prot" => "Protoss",
            "Terr" => "Terran",
            "Zerg" => "Zerg",
            _ => panic!("invalid faction name"),
        };
        writeln!(output, "faction: Faction::{faction},");

        // -------------------------------------- collision ------------------------------------- //
        let plane_array = unit.children.get("PlaneArray");
        let coll = match (
            plane_array.is_some_and(|x| x.children.contains_key("Ground")),
            plane_array.is_some_and(|x| x.children.contains_key("Air")),
        ) {
            (true, false) => "Ground",
            (false, true) => "Flying",
            (true, true) => "Both",
            (false, false) => "Ground",
        };
        writeln!(output, "collision: Collision::{coll},");

        // ---------------------------------------- flags --------------------------------------- //
        let flags_array = unit.child_array("Attributes");
        writeln!(output, "flags: Flags::new(");
        writeln!(output, "{},", flags_array.contains_key("Light"));
        writeln!(output, "{},", flags_array.contains_key("Armored"));
        writeln!(
            output,
            "{},",
            flags_array.contains_key("Mechanical")
        );
        writeln!(
            output,
            "{},",
            flags_array.contains_key("Biological")
        );
        writeln!(output, "{},", flags_array.contains_key("Massive"));
        writeln!(output, "{},", flags_array.contains_key("Psionic"));
        writeln!(
            output,
            "{},",
            flags_array.contains_key("Structure")
        );
        writeln!(output, "{},", flags_array.contains_key("Heroic"));
        writeln!(
            output,
            "{},",
            matches!(
                name,
                "HighTemplar"
                    | "Oracle"
                    | "Disruptor"
                    | "WidowMine"
                    | "Raven"
                    | "Lurker"
                    | "Infestor"
                    | "SwarmHost"
                    | "Viper"
            )
        );
        writeln!(output, "),");

        // --------------------------------------- health --------------------------------------- //
        writeln!(output,
            "hull: Health {{ max: const_real!({}), regen: rate!({}), delay: duration!({}), armor: const_real!({}), }},",
            unit.try_val_of("LifeMax").unwrap_or("1"),
            unit.try_val_of("LifeRegenRate").unwrap_or("0"),
            unit.try_val_of("LifeRegenDelay").unwrap_or("0"),
            unit.try_val_of("LifeArmor").unwrap_or("0"),
        );
        writeln!(output,
            "shields: Health {{ max: const_real!({}), regen: rate!({}), delay: duration!({}), armor: const_real!({}), }},",
            unit.try_val_of("ShieldsMax").unwrap_or("0"),
            unit.try_val_of("ShieldRegenRate").unwrap_or("0"),
            unit.try_val_of("ShieldRegenDelay").unwrap_or("0"),
            unit.try_val_of("ShieldArmor").unwrap_or("0"),
        );

        // -------------------------------------- movement -------------------------------------- //
        writeln!(output,
            "movement: Movement {{ speed: rate!({}), accel: rate!({}), decel: rate!({}), turn_rate: rate!({}), lateral_accel: rate!({}), }},",
            unit.try_val_of("Speed").unwrap_or("0"),
            unit.try_val_of("Acceleration").unwrap_or("1000"),
            unit.try_val_of("Deceleration").unwrap_or("0"),
            match name {
                "SiegeTankSieged" => "360",
                // "SporeCrawler" | "SpineCrawler" => "360",
                // "PhotonCannon" | "Colossus" | "Hellion" => "999.8437",
                // "Cyclone" => "1440",
                // "PlanetaryFortress" => "90",
                _ => unit.try_val_of("TurningRate").unwrap_or("720")
            },
            unit.try_val_of("LateralAcceleration").unwrap_or("0"),
        );

        // ---------------------------------------- cost ---------------------------------------- //
        if let Some(cost_array) = &unit.children.get("CostResource") {
            writeln!(output,
                "cost: Cost {{ minerals: const_real!({}), gas: const_real!({}), supply: const_real!({}), build_time: duration!({}), }},",
                cost_array.try_val_of("Minerals").unwrap_or("0"),
                cost_array.try_val_of("Vespene").unwrap_or("0"),
                &unit.try_val_of("Food").map(|x| x.strip_prefix('-').unwrap_or("0")).unwrap_or("0"), // Food value is negative so we need to strip off the minus sign
                "0.0", // TODO
            );
        } else {
            writeln!(output,
                "cost: Cost {{ minerals: const_real!(0), gas: const_real!(0), supply: const_real!(0), build_time: duration!(0), }},",
            );
        }

        // ---------------------------------------- misc ---------------------------------------- //
        writeln!(
            output,
            "size: const_real!({}),",
            unit.try_val_of("Radius").unwrap_or("0.5")
        );

        writeln!(
            output,
            "cargo_size: const_real!({}),",
            unit.try_val_of("CargoSize").unwrap_or("0")
        );

        writeln!(
            output,
            "sight: const_real!({}),",
            unit.try_val_of("Sight").unwrap_or("0")
        );
        writeln!(
            output,
            "push_priority: const_real!({}),",
            unit.try_val_of("PushPriority").unwrap_or("10")
        );

        writeln!(
            output,
            "energy_start: const_real!({}),",
            unit.try_val_of("EnergyStart").unwrap_or("0")
        );

        writeln!(
            output,
            "energy_max: const_real!({}),",
            unit.try_val_of("EnergyMax").unwrap_or("0")
        );

        // --------------------------------------- weapons -------------------------------------- //
        let temp = Map::default();
        let weapon_array = unit
            .children
            .get("WeaponArray")
            .map(|x| &x.children)
            .unwrap_or_else(|| {
                if name == "BroodlingDefault" {
                    UNIT_MAP["Broodling"].child_array("WeaponArray")
                } else {
                    &temp
                }
            });

        let mut count = 0;
        write!(output, "weapons: [");

        for (k, v) in weapon_array {
            if k.is_empty()
                || name == "Carrier" // ignore carrier's pseudo "launch interceptors" weapon
                || v.attrs.get("Link").is_none()
                || v.attrs["Link"].is_empty()
                || v.attrs["Link"] == "Talons"
                // absolute hack because queen is the only unit that truly has 3 weapons and i'm beyond caring
                || v.attrs["Link"].ends_with("Fake")
            {
                continue;
            }
            count += 1;
            if count > 2 {
                panic!("More than 2 weapons for unit {name}");
            }

            writeln!(
                output,
                "Some(Weapon::{}_{}), ",
                name.to_ascii_uppercase(),
                v.attrs["Link"].to_ascii_uppercase()
            );
        }
        if count == 0 {
            writeln!(output, "None, None,");
        } else if count == 1 {
            writeln!(output, "None,");
        }

        writeln!(output, "],\n}};\n");
    }

    writeln!(output, "}}");

    output
}

pub fn write_weapons(units: &[(&'static &'static str, &'static Tag)]) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "use crate::{{const_real, duration, unit::*,}};\n\nimpl Weapon {{"
    );

    let upgrades = get_upgrades();

    for (name, unit) in units {
        // skip carrier because interceptor launch isn't really a weapon
        if !VALID_UNITS.contains(name) || **name == "Carrier" {
            continue;
        }
        let name = match **name {
            "LurkerMP" => "Lurker",
            "LurkerMPBurrowed" => "LurkerBurrowed",
            "LurkerMPEgg" => "LurkerEgg",
            "ThorAP" => "ThorAlt",
            "VikingAssault" => "VikingGround",
            "VikingFighter" => "VikingAir",
            "SwarmHostMP" => "SwarmHost",
            "LocustMP" => "Locust",
            "LocustMPFlying" => "LocustFlying",
            // "QueenMP" => "Queen",
            "HellionTank" => "Hellbat",
            "LiberatorAG" => "LiberatorSieged",
            "BroodlingDefault" => "Broodling",
            x => x,
        };

        // let if + let else should be a crime
        let Some(weapons) = (if name != "Broodling" {
            unit.children.get("WeaponArray")
        } else {
            UNIT_MAP
                .get("Broodling")
                .unwrap()
                .children
                .get("WeaponArray")
        }) else {
            continue;
        };

        for (_idx, w) in &weapons.children {
            if w.attrs.get("Link").is_none()
                || w.attrs["Link"].is_empty() // links are blank if weapon was removed
                || w.attrs["Link"].ends_with("Fake")
            // handles things like siege tank's "90mmCannonsFake" placeholder
            {
                continue;
            }

            let w_name = w.attrs["Link"];

            let weapon = WEAPON_MAP.get(w_name).unwrap();
            let de_name = weapon
                .try_val_of("DisplayEffect")
                .unwrap_or_else(|| weapon.attrs["id"]);

            let display_effect = match de_name {
                "VolatileBurst" => &EFFECT_MAP["VolatileBurstU"],
                _ => EFFECT_MAP.get(de_name).unwrap(),
            };

            let e_name = weapon
                .try_val_of("Effect")
                .unwrap_or_else(|| weapon.attrs["id"]);

            let w_effect = EFFECT_MAP.get(e_name);

            writeln!(
                output,
                "pub const {}_{}: Weapon = Weapon {{",
                name.to_ascii_uppercase(),
                w.attrs["Link"].to_ascii_uppercase()
            );

            // ---------------------------------- melee/ranged ---------------------------------- //

            // we have to check parent first and possibly overwrite it because of things like viking
            let mut kind = if let Some(p) = display_effect.attrs.get("parent") {
                match *p {
                    "DU_WEAP_MISSILE" | "DU_WEAP" => "Hitscan",
                    _ => "Melee",
                }
            } else {
                "Melee"
            };

            if let Some(x) = display_effect.try_val_of("Kind") {
                match x {
                    "Ranged" | "Splash" => kind = "Hitscan",
                    _ => kind = "Melee",
                }
            }

            // check for LaunchMissile effect, indicating a non-hitscan weapon
            if kind == "Hitscan" && w_effect.is_some_and(|x| x.kind == "CEffectLaunchMissile")
                || weapon.children.get("PeriodicEffectArray").is_some_and(|x| {
                    EFFECT_MAP
                        .get(x.children.first().unwrap().1.attrs["value"])
                        .is_some_and(|y| y.kind == "CEffectLaunchMissile")
                })
            {
                kind = "Projectile";
            }

            writeln!(output, "kind: WeaponKind::{kind},");

            // ------------------------------------- damage ------------------------------------- //

            assert_eq!(display_effect.kind, "CEffectDamage");

            let dmg = display_effect.try_val_of("Amount").unwrap_or_else(|| {
                // some weapons like roach melee attack derive most of their stats from a parent
                // so sometimes the Amount value isn't present in the display effect we have

                let parent = display_effect.attrs["parent"];
                EFFECT_MAP[parent].val_of("Amount")
            });

            writeln!(output, "damage: const_real!({dmg}),",);

            let (bonus_damage, bonus_vs) =
                if let Some(bonus) = display_effect.children.get("AttributeBonus") {
                    (
                        bonus.attrs["value"],
                        format!("Some(Flag::{})", bonus.attrs["index"]),
                    )
                } else {
                    ("0", "None".to_owned())
                };

            writeln!(output, "bonus_damage: const_real!({bonus_damage}),");
            writeln!(output, "bonus_vs: {bonus_vs},");

            // ------------------------------------- target ------------------------------------- //

            let filters = weapon.val_of("TargetFilters");

            let can_target = if filters.contains("Ground") {
                "Ground"
            } else if filters.contains("Air") {
                "Flying"
            } else {
                "Both"
            };

            writeln!(output, "can_target: Collision::{can_target},");

            // ------------------------------------ upgrades ------------------------------------ //

            // upgrade lookup fails on units with weapons that don't upgrade (i.e. workers)
            // so we need to make sure we have a default for that
            let up = upgrades.get(de_name).cloned().unwrap_or((0.0, 0.0));

            writeln!(
                output,
                "upgrade_inc: [const_real!({}), const_real!({})],",
                up.0, up.1
            );

            // -------------------------------- range/arc + slop -------------------------------- //

            let min_range = weapon.try_val_of("MinimumRange").unwrap_or("0");
            let range = weapon.try_val_of("Range").unwrap_or("5");
            let slop = weapon.try_val_of("RangeSlop").unwrap_or("0");

            writeln!(
                output,
                "range: RangeInclusive::new(const_real!({min_range}), const_real!({range})),"
            );
            writeln!(output, "range_slop: const_real!({slop}),");

            let arc = weapon.try_val_of("Arc").unwrap_or("0");
            let arc_slop = weapon.try_val_of("ArcSlop").unwrap_or("11.25");

            writeln!(output, "arc: const_real!({arc}),");
            writeln!(output, "arc_slop: const_real!({arc_slop}),");

            // -------------------------------------- speed ------------------------------------- //

            let speed = weapon.try_val_of("Period").unwrap_or("0.8332");

            writeln!(output, "attack_speed: duration!({speed}),");

            let delay_min = weapon.try_val_of("RandomDelayMin").unwrap_or("-0.0625");
            let delay_max = weapon.try_val_of("RandomDelayMax").unwrap_or("0.125");

            writeln!(
                output,
                "random_delay: RangeInclusive::new(duration!({delay_min}), duration!({delay_max})),"
            );

            // ------------------------------ dmg point & backswing ----------------------------- //

            let dmg_point = weapon.try_val_of("DamagePoint").unwrap_or("0.167");
            let backswing = weapon.try_val_of("Backswing").unwrap_or("0.5");

            writeln!(output, "damage_point: duration!({dmg_point}),");
            writeln!(output, "backswing: duration!({backswing}),");

            // ------------------------------------ priority ------------------------------------ //

            let priority = match weapon.try_val_of("AquirePrioritization") {
                Some("ByAngle") => "LeastAngle",
                Some("ByDistanceFromTarget") => "DistPrevTarget",
                _ => "Normal",
            };

            writeln!(output, "priority: Priority::{priority},");

            // ----------------------------- multihit/search/effect ----------------------------- //

            /*
                I'm just gonna manually enter the details of these. The potential layouts and nested
                effects and other nonsense is more annoying to deal with than a bit of manual entry.
                Most units that hit more than once, and any unit with a bouncing/AoE attack will
                require weapon adjustments.
            */

            if let Some(p) = weapon.try_val_of("DisplayAttackCount") {
                writeln!(output, "multihit: Multihit::Instant(const_real!({p})),");
            } else {
                writeln!(output, "multihit: Multihit::Single,");
            }

            writeln!(output, "search: SearchType::Single,");

            writeln!(output, "effect: None,");

            writeln!(output, "}};\n");
        }
    }

    writeln!(output, "}}");

    output
}

fn get_upgrades() -> Map<&'static str, (f32, f32)> {
    let mut upgrades = Map::default();

    for (up_name, up) in UPGRADE_MAP.iter() {
        if !WEAP_UPGRADES.contains(up_name) {
            continue;
        }

        let eff_array = &up.children["EffectArray"].children;

        for (_idx, elmt) in eff_array {
            if elmt.attrs.get("Reference").is_none() {
                continue;
            }
            let mut reference = elmt.attrs["Reference"].split(',');
            if reference.next().is_some_and(|x| x == "Effect") {
                let link = reference.next().unwrap();
                let entry: &mut (f32, f32) = upgrades.entry(link).or_default();
                let modifier = reference.next().unwrap();

                let value = *elmt.attrs.get("Value").unwrap_or(&"0");
                match modifier {
                    "Amount" => entry.0 = value.parse().unwrap(),
                    _ => entry.1 = value.parse().unwrap(),
                }
            }
        }
    }

    upgrades
}

const WEAP_UPGRADES: [&str; 16] = [
    "TerranInfantryWeapons",
    "TerranVehicleWeapons",
    "TerranShipWeapons",
    "ProtossGroundWeapons",
    "ProtossAirWeapons",
    "ZergMeleeWeapons",
    "ZergMissileWeapons",
    "ZergFlyerWeapons",
    "TerranInfantryWeaponsLevel1",
    "TerranVehicleWeaponsLevel1",
    "TerranShipWeaponsLevel1",
    "ProtossGroundWeaponsLevel1",
    "ProtossAirWeaponsLevel1",
    "ZergMeleeWeaponsLevel1",
    "ZergMissileWeaponsLevel1",
    "ZergFlyerWeaponsLevel1",
];

const VALID_UNITS: [&str; 70] = [
    "LurkerMP",
    "LurkerMPBurrowed",
    "LurkerMPEgg",
    "Ravager",
    "RavagerCocoon",
    "MULE",
    "Probe",
    "Zealot",
    "HighTemplar",
    "DarkTemplar",
    "Observer",
    "Carrier",
    "Interceptor",
    "Archon",
    "Phoenix",
    "VoidRay",
    "WarpPrism",
    "Stalker",
    "Colossus",
    "Mothership",
    "SCV",
    "Marine",
    "Reaper",
    "Ghost",
    "SiegeTank",
    "SiegeTankSieged",
    "Thor",
    "ThorAP",
    "Banshee",
    "Medivac",
    "Battlecruiser",
    "Raven",
    "VikingAssault",
    "VikingFighter",
    "Larva",
    "Drone",
    "Roach",
    "RoachBurrowed",
    "Overlord",
    "Overseer",
    "Zergling",
    "Hydralisk",
    "Mutalisk",
    "Ultralisk",
    "Baneling",
    "Infestor",
    "InfestorBurrowed",
    "Immortal",
    "Marauder",
    "BroodLord",
    "BroodlingDefault",
    "Corruptor",
    "Sentry",
    "Queen",
    "Hellion",
    "Changeling",
    "Oracle",
    "HellionTank",
    "SwarmHostMP",
    "LocustMP",
    "Tempest",
    "Viper",
    "WidowMine",
    "WidowMineBurrowed",
    "Cyclone",
    "LocustMPFlying",
    "Disruptor",
    // "QueenMP",
    "Adept",
    "Liberator",
    "LiberatorAG",
];
