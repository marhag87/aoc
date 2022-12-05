#[macro_use]
extern crate scan_fmt;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod input;

use crate::input::input_as_string;
use day_1::*;
use day_2::*;
use day_3::*;
use day_4::*;
use day_5::*;

fn main() {
    println!("Day 1");
    day_1();
    println!();

    println!("Day 2");
    println!("{}", day_2_1(input_as_string("day_2.txt")));
    println!("{}", day_2_2(input_as_string("day_2.txt")));
    println!();

    println!("Day 3");
    println!("{}", day_3_1(input_as_string("day_3.txt")));
    println!("{}", day_3_2(input_as_string("day_3.txt")));
    println!();

    println!("Day 4");
    println!("{}", day_4_1(input_as_string("day_4.txt")));
    println!("{}", day_4_2(input_as_string("day_4.txt")));
    println!();

    println!("Day 5");
    println!("{}", day_5_1(input_as_string("day_5.txt")));
    println!("{}", day_5_2(input_as_string("day_5.txt")));
}
