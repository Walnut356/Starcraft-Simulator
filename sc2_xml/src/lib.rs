pub mod parser;
pub mod write;
pub type Map<K, V> = IndexMap<K, V, FxBuildHasher>;
use fxhash::FxBuildHasher;
use indexmap::IndexMap;
use parser::*;
use std::fmt::Write;

pub fn mp_units() -> Vec<(&'static &'static str, &'static Tag)> {
    UNIT_MAP
        .iter()
        .filter(|(_k, v)| {
            v.attrs.get("default").is_none()
                && v.children.get("EditorCategories").is_some_and(|x| {
                    x.attrs
                        .get("value")
                        .is_some_and(|y| y.contains("Unit") && !y.contains("Structure"))
                })
                && v.children
                    .get("Mob")
                    .is_some_and(|y| y.attrs.get("value").is_some_and(|z| *z == "Multiplayer"))
        })
        .collect()
}

pub fn get_structures() -> Vec<(&'static &'static str, &'static Tag)> {
    UNIT_MAP
        .iter()
        .filter(|(_k, v)| {
            v.attrs.get("default").is_none()
                && v.children.get("EditorCategories").is_some_and(|x| {
                    x.attrs
                        .get("value")
                        .is_some_and(|y| y.contains("Structure"))
                })
                && v.children
                    .get("Mob")
                    .is_some_and(|y| y.attrs.get("value").is_some_and(|z| *z == "Multiplayer"))
        })
        .collect()
}

// /// Returns tuples of unit name and build time
// pub fn build_times() -> Vec<(&'static str, f64)> {
//     let mut result = Vec::new();
//     let structures = get_structures();
//     for (struct_name, _tag) in structures {
//         // filter is required over find because some structures like the Nexus use multiple abilities
//         for (_abil_name, desc) in ABIL_MAP.iter().filter(|x| x.0.starts_with(struct_name) && x.0.contains("Train")) {
//             if desc.children.get("InfoArray").is_none() {
//                 continue;
//             }
//             let info = desc.children.get("InfoArray").unwrap();

//             /*
//                 TODO problem is warpgatetrain, which has a slightly different format
//              */
//             for (_, child) in &info.children {
//                 let time = child.attrs.get("Time").unwrap().parse::<f64>().unwrap();
//                 let unit = *child
//                     .children
//                     .get("Unit")
//                     .unwrap_or
//                     .attrs
//                     .get("value")
//                     .unwrap();
//                 result.push((unit, time));
//             }
//         }
//     }

//     result
// }

// fn structure_units() -> Vec<(&'static str, f64)> {
//     let structures = get_structures();

//     for (_name, data) in structures {

//     }

//     Vec::new()
// }

// fn get_abil(link: &'static str) {
//     let abil = ABIL_MAP.get(link).unwrap();
//     if let Tag::Node { attrs, children } = abil {}
// }
