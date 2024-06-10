use fxhash::FxHashMap;
use quanta::Clock;
use sc2_xml::*;

pub fn main() {
    let clock = Clock::new();
    let now = clock.now();
    let units = parser::init_weapons();

    let val = units.get("ParticleDisruptors").unwrap();
    dbg!(val);
    // let filtered: FxHashMap<&&str, &Tag> = units
    //     .iter()
    //     .filter(|(k, v)| match v {
    //         Tag::Node { attrs, children } => {
    //             attrs.get("default").is_none() &&
    //             children.get("EditorCategories").is_some_and(|x| match x {
    //                 Tag::Leaf { attrs } => attrs
    //                     .get("value")
    //                     .is_some_and(|x| x.contains("Unit") || x.contains("Structure")),
    //                 _ => false,
    //             }) && children.get("Mob").is_some_and(|x| match x {
    //                 Tag::Leaf { attrs } => attrs.get("value").is_some_and(|x| *x == "Multiplayer"),
    //                 _ => false,
    //             })
    //         }
    //         _ => false,
    //     })
    //     .collect();
    // let dur = now.elapsed();
    // dbg!(dur);

    // dbg!("here");
    // let val = units.get("Ultralisk").unwrap();
    // dbg!(val);
}
