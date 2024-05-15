use std::time::Instant;

use fixed::traits::Fixed;
use itertools::Itertools;
use sc2_sim::{unit::Cost, *};

fn main() {
    let mut c = Coordinator::default();
    c.randomize_seed();
    // dbg!(c.seed);
    c.a1.add_unit(Unit::MARINE.with_combat_shields(), 20);
    c.a2.add_unit(Unit::MARINE, 23);

    simulate(&mut c, 1000);
    // let now = Instant::now();
    // let o = c.simulate();
    // let dur = now.elapsed();
    // println!("Simulation run time: {:?}", dur);

    // dbg!(o.team_1().units[0].effective_dps(o.duration()));

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
    let now = Instant::now();
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
    let dur = now.elapsed();
    println!("Simulation time ({} runs): {:?}", run_count, dur);
    println!("Team 1: {:?}", c.a1.units.iter().map(|x| x.base).counts());
    println!("Team 2: {:?}", c.a2.units.iter().map(|x| x.base).counts());
    println!(
        "Team 1 wins: {} | Team 2 wins: {} | Draws: {} ",
        results[0], results[1], results[2]
    );
    println!(
        "Average in-game fight duration: {}s",
        fight_dur / real!(run_count)
    );
}
