use crate::input::input_as_string;

pub(crate) fn day_1() {
    let mut nums: Vec<i32> = input_as_string("day_1.txt")
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split('\n')
                .map(|num| num.parse::<i32>().unwrap_or_default())
                .sum::<i32>()
        })
        .collect();
    println!("max = {:?}", nums.iter().max());
    nums.sort();
    eprintln!("top three = {:?}", nums.iter().rev().take(3).sum::<i32>());
}
