#[macro_use]
extern crate scan_fmt;

mod day_1;
mod day_2;
mod day_3;
// mod day_4;
mod day_5;
// mod day_6;
// mod day_7;

use day_1::*;
use day_2::*;
use day_3::*;
// use day_4::*;
use day_5::*;
// use day_6::*;
// use day_7::*;

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

    println!("Day 3");
    let day_3_input = String::from_utf8_lossy(include_bytes!("../input/day_3.txt"));
    println!("{}", day_3_1(&day_3_input));
    println!("{}", day_3_2(&day_3_input));
    println!();

    // println!("Day 4");
    // let day_4_input = String::from_utf8_lossy(include_bytes!("../input/day_4.txt"));
    // println!("{}", day_4_1(&day_4_input));
    // println!("{}", day_4_2(&day_4_input));
    // println!();

    println!("Day 5");
    let day_5_input = String::from_utf8_lossy(include_bytes!("../input/day_5.txt"));
    println!("{}", day_5_1(&day_5_input));
    println!("{}", day_5_2(&day_5_input));
    println!();

    // println!("Day 6");
    // let day_6_input = String::from_utf8_lossy(include_bytes!("../input/day_6.txt"));
    // println!("{}", day_6_1(&day_6_input));
    // println!("{}", day_6_2(&day_6_input));
    // println!();

    // println!("Day 7");
    // let day_7_input = String::from_utf8_lossy(include_bytes!("../input/day_7.txt"));
    // println!("{}", day_7_1(&day_7_input));
    // println!("{}", day_7_2(&day_7_input));
    // println!();
}
