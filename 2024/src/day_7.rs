fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_7.txt"));
    let first = day_7_1(&input);
    println!("{:?}", first);
    let second = day_7_2(&input);
    println!("{:?}", second);
}

fn day_7_1(input: &str) -> usize {
    let lines = parse(input);
    lines
        .iter()
        .map(|(sum, numbers)| {
            let mut calcs = Vec::new();
            numbers.iter().for_each(|num| {
                if calcs.is_empty() {
                    calcs.push(*num);
                } else {
                    let mut new = Vec::new();
                    calcs.iter().for_each(|calc| new.push(*calc * num));
                    calcs.iter().for_each(|calc| new.push(*calc + num));
                    calcs = new;
                }
            });
            if calcs.iter().any(|calc| calc == sum) {
                *sum
            } else {
                0
            }
        })
        .sum()
}

fn day_7_2(input: &str) -> usize {
    let lines = parse(input);
    lines
        .iter()
        .map(|(sum, numbers)| {
            let mut calcs = Vec::new();
            numbers.iter().for_each(|num| {
                if calcs.is_empty() {
                    calcs.push(*num);
                } else {
                    let mut new = Vec::new();
                    calcs.iter().for_each(|calc| new.push(*calc * num));
                    calcs.iter().for_each(|calc| new.push(*calc + num));
                    calcs.iter().for_each(|calc| {
                        new.push(format!("{}{}", calc, num).parse::<usize>().unwrap())
                    });
                    calcs = new;
                }
            });
            if calcs.iter().any(|calc| calc == sum) {
                *sum
            } else {
                0
            }
        })
        .sum()
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(':');
            let sum = split.next().unwrap().parse::<usize>().unwrap();
            let numbers = split.next().unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (sum, numbers)
        })
        .collect::<Vec<(usize, Vec<usize>)>>()
}

#[test]
fn test_day_7_1() {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    assert_eq!(day_7_1(input), 3749);
}

#[test]
fn test_day_7_2() {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    assert_eq!(day_7_2(input), 11387);
}

#[test]
fn test_day_7_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_7.txt"));
    assert_eq!(day_7_1(&input), 5702958180383);
}

// Disabled because slow
// #[test]
// fn test_day_7_2_answer() {
//     let input = String::from_utf8_lossy(include_bytes!("../input/day_7.txt"));
//     assert_eq!(day_7_2(&input), 92612386119138);
// }
