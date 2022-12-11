mod day_1;

fn main() {
    println!("Day 1");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    println!("{}", day_1::day_1_1(&input));
    println!("{}", day_1::day_1_2(&input));
}
