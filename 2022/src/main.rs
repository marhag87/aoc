mod day_1;
mod day_2;
mod input;

use crate::input::input_as_string;
use day_1::*;
use day_2::*;

fn main() {
    println!("Day 1");
    day_1();
    println!();

    println!("Day 2");
    println!("{}", day_2_1(input_as_string("day_2.txt")));
    println!("{}", day_2_2(input_as_string("day_2.txt")));
}
