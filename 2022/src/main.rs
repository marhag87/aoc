#[macro_use]
extern crate scan_fmt;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    println!("Day 1");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    let (max, top_three) = day_1::day_1(&input);
    println!("{}\n{}\n", max, top_three);

    println!("Day 2");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_2.txt"));
    println!("{}", day_2::day_2_1(&input));
    println!("{}", day_2::day_2_2(&input));
    println!();

    println!("Day 3");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_3.txt"));
    println!("{}", day_3::day_3_1(&input));
    println!("{}", day_3::day_3_2(&input));
    println!();

    println!("Day 4");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_4.txt"));
    println!("{}", day_4::day_4_1(&input));
    println!("{}", day_4::day_4_2(&input));
    println!();

    println!("Day 5");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_5.txt"));
    println!("{}", day_5::day_5_1(&input));
    println!("{}", day_5::day_5_2(&input));
    println!();

    println!("Day 6");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_6.txt"));
    println!("{}", day_6::day_6_1(&input));
    println!("{}", day_6::day_6_2(&input));
    println!();

    println!("Day 7");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_7.txt"));
    let (part_1, part_2) = day_7::day_7(&input);
    assert_eq!(part_1, 1084134);
    println!("{}", part_1);
    println!("{}", part_2);
    println!();

    println!("Day 8");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_8.txt"));
    let (part_1, part_2) = day_8::day_8(&input);
    println!("{}", part_1);
    println!("{}", part_2);
    println!();

    println!("Day 9");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_9.txt"));
    println!("{}", day_9::day_9_1(&input));
    println!("{}", day_9::day_9_2(&input));
    println!();

    println!("Day 10");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_10.txt"));
    println!("{}", day_10::day_10_1(&input));
    println!("{}", day_10::day_10_2(&input));

    println!("Day 11");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_11.txt"));
    println!("{}", day_11::day_11_1(&input));
    println!("{}", day_11::day_11_2(&input));
    println!();

    println!("Day 12");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_12.txt"));
    let (part_1, part_2) = day_12::day_12(&input);
    println!("{}", part_1);
    println!("{}", part_2);
    println!();

    println!("Day 13");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_13.txt"));
    println!("{}", day_13::day_13_1(&input));
    println!("{}", day_13::day_13_2(&input));
    println!();

    println!("Day 14");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_14.txt"));
    let (part_1, part_2) = day_14::day_14(&input);
    println!("{}", part_1);
    println!("{}", part_2);
    println!();

    println!("Day 15");
    let input = String::from_utf8_lossy(include_bytes!("../input/day_15.txt"));
    println!("{}", day_15::day_15_1(&input, 2000000));
    println!("{}", day_15::day_15_2(&input, 4000000));
}
