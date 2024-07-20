use std::{hint::black_box, time::Instant};

use const_soft_float::soft_f64::SoftF64;
use fixed32::*;
use num::integer::Roots;
use fixed::types::I20F12;

fn main() {
    dbg!(5.75f64.sqrt());
    let x = Fixed32::from(5.75);
    let y = x.as_bits();
    dbg!(x.sqrt());
    dbg!(Fixed32::from_bits(y.sqrt() << 6));


    // let now = Instant::now();
    // floats();

    // dbg!(now.elapsed());

    // let now = Instant::now();

    // fixed();

    // dbg!(now.elapsed());

    // let now = Instant::now();

    // ints();

    // dbg!(now.elapsed());

    // let now = Instant::now();

    // fixedcrate();

    // dbg!(now.elapsed());
}

#[inline(never)]
fn floats() {
    for _ in 0..1_000_000 {
        let mut x: f64 = 5.25;
        let mut y: f64 = 3.75;
        black_box(&mut x);
        black_box(&mut y);

        black_box(y.sqrt());
    }
}

#[inline(never)]
fn fixed() {
    for _ in 0..1_000_000 {
        let mut x = Fixed32::from_f64(5.25);
        let mut y = Fixed32::from_f64(3.75);
        black_box(&mut x);
        black_box(&mut y);

        black_box(y.sqrt());
    }
}

#[inline(never)]
fn ints() {
    for _ in 0..1_000_000 {
        let mut x = 6;
        let mut y = 4;
        black_box(&mut x);
        black_box(&mut y);

        black_box(x.sqrt());
    }
}

#[inline(never)]
fn fixedcrate() {
    for _ in 0..1_000_000 {
        let mut x = I20F12::from_num(5.25f64);
        let mut y = I20F12::from_num(3.75f64);
        black_box(&mut x);
        black_box(&mut y);

        black_box(x.sqrt());
    }
}