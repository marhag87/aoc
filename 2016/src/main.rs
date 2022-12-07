// #[macro_use]
// extern crate scan_fmt;

mod day_1;
// mod day_2;
// mod day_3;
// mod day_4;
// mod day_5;
// mod day_6;
// mod day_7;

use day_1::*;
// use day_2::*;
// use day_3::*;
// use day_4::*;
// use day_5::*;
// use day_6::*;
// use day_7::*;

fn main() {
    println!("Day 1");
    let day_1_input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    println!("{}", day_1_1(&day_1_input));
    println!("{}", day_1_2(&day_1_input));

    // println!("Day 2");
    // let day_2_input = String::from_utf8_lossy(include_bytes!("../input/day_2.txt"));
    // println!("{}", day_2_1(&day_2_input));
    // println!("{}", day_2_2(&day_2_input));
    // println!();
}
