#[macro_use]
extern crate scan_fmt;

mod day_1;
mod day_2;

use day_1::*;
use day_2::*;

fn main() {
    println!("Day 1");
    let day_1_input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    let (floor, position) = day_1_1(&day_1_input);
    println!("{}\n{}\n", floor, position);

    println!("Day 2");
    let day_2_input = String::from_utf8_lossy(include_bytes!("../input/day_2.txt"));
    println!("{}", day_2_1(&day_2_input));
    println!("{}", day_2_2(&day_2_input));
    println!();
}
