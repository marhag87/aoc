fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    let first = day_1_1(&input);
    println!("{:?}", first);
    let second = day_1_2(&input);
    println!("{:?}", second);
}

fn day_1_1(input: &str) -> usize {
    let (first_list, second_list) = parse(input);
    first_list
        .into_iter()
        .zip(second_list)
        .map(|(first, second)| first.abs_diff(second))
        .sum()
}

fn day_1_2(input: &str) -> usize {
    let (first_list, second_list) = parse(input);
    first_list
        .iter()
        .map(|first| {
            let second_count = second_list.iter().filter(|second| &first == second).count();
            first * second_count
        })
        .sum()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    input.lines().for_each(|line| {
        let mut split = line.split_ascii_whitespace();
        let first = split.next().unwrap().parse::<usize>().unwrap();
        let second = split.next().unwrap().parse::<usize>().unwrap();
        first_list.push(first);
        second_list.push(second);
    });
    first_list.sort();
    second_list.sort();
    (first_list, second_list)
}

#[test]
fn test_day_1_1() {
    let input = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;
    assert_eq!(day_1_1(input), 11);
}

#[test]
fn test_day_1_2() {
    let input = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;
    assert_eq!(day_1_2(input), 31);
}

#[test]
fn test_day_1_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    assert_eq!(day_1_1(&input), 1938424);
}

#[test]
fn test_day_1_2_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_1.txt"));
    assert_eq!(day_1_2(&input), 22014209);
}
