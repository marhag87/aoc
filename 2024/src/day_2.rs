use itertools::Itertools;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_2.txt"));
    let first = day_2_1(&input);
    println!("{:?}", first);
    let second = day_2_2(&input);
    println!("{:?}", second);
}

fn day_2_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let split = line.split_ascii_whitespace();
            let numbers = split
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            level_is_safe(numbers)
        })
        .count()
}

fn day_2_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let split = line.split_ascii_whitespace();
            let numbers = split
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut numbers_mutations = vec![numbers.clone()];
            for (i, _) in numbers.iter().enumerate() {
                let mut numbers = numbers.clone();
                numbers.remove(i);
                numbers_mutations.push(numbers);
            }
            numbers_mutations.into_iter().any(level_is_safe)
        })
        .count()
}

fn level_is_safe(numbers: Vec<usize>) -> bool {
    let mut growing = None;
    numbers.iter().tuple_windows().all(|(a, b)| {
        let this_growing = Some(a < b);
        if (growing.is_some() && growing != this_growing) || a == b {
            false
        } else {
            growing = this_growing;
            a.abs_diff(*b) <= 3
        }
    })
}

#[test]
fn test_day_2_1() {
    let input = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;
    assert_eq!(day_2_1(input), 2);
}

#[test]
fn test_day_2_2() {
    let input = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;
    assert_eq!(day_2_2(input), 4);
}

#[test]
fn test_day_2_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_2.txt"));
    assert_eq!(day_2_1(&input), 510);
}

#[test]
fn test_day_2_2_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_2.txt"));
    assert_eq!(day_2_2(&input), 553);
}
