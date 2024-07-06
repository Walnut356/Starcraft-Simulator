use std::{fs::File, io::Write};

use fxhash::FxHashMap;
use parser::{init_units, Tag, UNIT_MAP};
use quanta::Clock;
use sc2_xml::*;
use write::{write_units, write_weapons};

pub fn main() {
    let clock = Clock::new();
    let now = clock.now();
    let all_units = &UNIT_MAP;
    // let stalker = all_units.get("Stalker").unwrap();
    let units = mp_units();
    // let output = write_units(&units);

    // let mut file = File::create("unit_data.rs").unwrap();
    // file.write_all(output.as_bytes()).unwrap();

    // let dur = now.elapsed();
    // dbg!(dur);

    let output = write_weapons(&units);

    let mut file = File::create("weapon_data.rs").unwrap();
    file.write_all(output.as_bytes()).unwrap();

    let dur = now.elapsed();
    dbg!(dur);


    // let val = all_units.get("Ultralisk").unwrap();
    // dbg!(val);
}
