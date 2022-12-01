use std::fs::File;
use std::io::Read;

pub(crate) fn day_1() {
    let mut file = File::open("input/day_1.txt").expect("should be able to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("should be able to read input");
    let mut nums: Vec<i32> = contents
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
