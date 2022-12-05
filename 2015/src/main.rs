mod day_1;

use day_1::*;

fn main() {
    println!("Day 1");
    let day_1_input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    let (floor, position) = day_1_1(&day_1_input);
    println!("{}\n{}\n", floor, position);
}
