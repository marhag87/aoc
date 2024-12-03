use regex::Regex;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_3.txt"));
    let first = day_3_1(&input);
    println!("{:?}", first);
    let second = day_3_2(&input);
    println!("{:?}", second);
}

fn day_3_1(input: &str) -> usize {
    let re = Regex::new(r"mul\((?<first>\d*),(?<second>\d*)\)").unwrap();
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|captures| {
                    let first = captures["first"].parse::<usize>().unwrap();
                    let second = captures["second"].parse::<usize>().unwrap();
                    first * second
                })
                .sum::<usize>()
        })
        .sum()
}

fn day_3_2(input: &str) -> usize {
    let re = Regex::new(r"(?<do>do\(\))|(?<dont>don\'t\(\))|mul\((?<first>\d*),(?<second>\d*)\)")
        .unwrap();
    let mut enabled = true;
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|captures| {
                    if captures.name("do").is_some() {
                        enabled = true;
                    }
                    if captures.name("dont").is_some() {
                        enabled = false;
                    }
                    if let (Some(first), Some(second), true) = (
                        captures
                            .name("first")
                            .map(|m| m.as_str().parse::<usize>().unwrap()),
                        captures
                            .name("second")
                            .map(|m| m.as_str().parse::<usize>().unwrap()),
                        enabled,
                    ) {
                        first * second
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test_day_3_1() {
    let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    assert_eq!(day_3_1(input), 161);
}

#[test]
fn test_day_3_2() {
    let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    assert_eq!(day_3_2(input), 48);
}

#[test]
fn test_day_3_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_3.txt"));
    assert_eq!(day_3_1(&input), 155955228);
}

#[test]
fn test_day_3_2_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_3.txt"));
    assert_eq!(day_3_2(&input), 100189366);
}
