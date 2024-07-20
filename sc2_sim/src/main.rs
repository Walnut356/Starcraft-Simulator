use coordinator::{Coordinator, Team};
use quanta::Clock;

use itertools::Itertools;
use sc2_sim::{
    unit::Cost,
    *,
};
use strum::VariantNames;
use unit::Unit;



fn main() {
    env_logger::init();

    let one = -17.0f64;
    let two = 45.0;

    dbg!(one.atan2(two).to_degrees());
    dbg!(Real::atan2(real!(one), real!(two)));



    // dbg!(rot.to_bits());
    // dbg!(base.to_bits());

    // let rot = 999.84375f32.to_radians();
    // let base = 1000.0f32.to_radians();

    // dbg!(real!(rot * (1.0 / 16.0)));
    // dbg!(real!(base * (1.0 / 16.0)));
    // dbg!(360.0f32.to_radians() / (rot * (1.0 / 16.0)));
    // dbg!(360.0f32.to_radians() / (base * (1.0 / 16.0)));

    // let clock = Clock::new();
    // let start = clock.now();
    // for name in Base::VARIANTS {
    //     let path = format!("G:\\Coding\\My Projects\\Crates\\starcraft_numerics\\editor_export\\{}.xml", name);
    //     let unit = parse_unit(name);
    // }
    // let unit = parse_unit("");
    // parse_mods();
    // let end = clock.now();

    // dbg!(end.duration_since(start));
    // dbg!(unit.unwrap());
    // let mut c = Coordinator::default();
    // c.randomize_seed();
    // // dbg!(c.seed);
    // c.a1.add_unit(Unit::STALKER, 5);
    // c.a1.add_unit(Unit::ARCHON, 1);
    // c.a2.add_unit(Unit::MARINE, 6);
    // c.a2.add_unit(Unit::MARAUDER, 5);

    // simulate(&mut c, 1000);

    // let o = c.simulate();

    // dbg!(o.team_1().trackers.iter().map(|x| x.damage_dealt).sum::<Real>());

    // println!("Team 1 cost: {:?}", o.team_1().total_cost());
    // println!("Team 2 cost: {:?}", o.team_2().total_cost());
    // println!("Cost Diff: {:?}\n", o.cost_difference(Team::Team1));
    // println!("Fight Duration: {}s", o.duration());
    // println!("Winner: {:?}", o.winner());
    // println!("Units Remaining: {:?}", o.units_remaining());
    // println!("Units Lost (Winner): {:?}", o.units_lost());
    // println!("Resources Lost (Winner): {:?}", o.resources_lost().unwrap());
}

pub fn simulate(c: &mut Coordinator, run_count: usize) {
    let mut results = [0, 0, 0];
    let mut fight_dur = Real::default();

    let clock = Clock::new();
    let start = clock.now();

    for _ in 0..run_count {
        c.randomize_seed();
        let w = c.simulate();
        match w.winner() {
            Some(Team::Team1) => results[0] += 1,
            Some(Team::Team2) => results[1] += 1,
            None => results[2] += 1,
        }
        fight_dur += c.time;
        c.reset();
    }

    let end = clock.now();
    println!("Simulation time ({} runs): {:?}", run_count, end.duration_since(start));
    println!(
        "Team 1: {:?}",
        c.a1.units().iter().map(|u| u.base).counts()
    );
    println!(
        "Team 2: {:?}",
        c.a2.units().iter().map(|u| u.base).counts()
    );
    println!(
        "Team 1 wins: {} | Team 2 wins: {} | Draws: {} ",
        results[0], results[1], results[2]
    );
    println!(
        "Average in-game fight duration: {}s",
        fight_dur / real!(run_count)
    );
}
