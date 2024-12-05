use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_5.txt"));
    let first = day_5_1(&input);
    println!("{:?}", first);
    let second = day_5_2(&input);
    println!("{:?}", second);
}

fn day_5_1(input: &str) -> usize {
    let (rule_map, updates) = parse(input);
    updates
        .iter()
        .map(|line_numbers| {
            if line_numbers.iter().enumerate().all(|(i, num)| {
                let (_, rest) = line_numbers.split_at(i);
                rest.iter()
                    .all(|rest_num| !rule_map.get(num).unwrap_or(&Vec::new()).contains(rest_num))
            }) {
                *line_numbers.get(line_numbers.len() / 2).unwrap()
            } else {
                0
            }
        })
        .sum()
}

fn day_5_2(input: &str) -> usize {
    let (rule_map, updates) = parse(input);
    let incorrect = updates
        .into_iter()
        .filter(|line_numbers| {
            line_numbers.iter().enumerate().any(|(i, num)| {
                let (_, rest) = line_numbers.split_at(i);
                rest.iter()
                    .any(|rest_num| rule_map.get(num).unwrap_or(&Vec::new()).contains(rest_num))
            })
        })
        .collect::<Vec<Vec<usize>>>();
    let rules = Rules { rule_map };
    incorrect
        .iter()
        .map(|line_numbers| {
            let mut numbers = line_numbers.clone();
            numbers.sort_by(|left, right| rules.compare(left, right));
            *numbers.get(numbers.len() / 2).unwrap()
        })
        .sum()
}

fn parse(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap();
    let updates = split.next().unwrap();
    let mut rule_map = HashMap::new();
    rules.lines().for_each(|line| {
        let mut split = line.split('|');
        let first = split.next().unwrap().parse::<usize>().unwrap();
        let second = split.next().unwrap().parse::<usize>().unwrap();
        rule_map
            .entry(second)
            .and_modify(|value: &mut Vec<usize>| value.push(first))
            .or_insert(vec![first]);
    });
    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    (rule_map, updates)
}

struct Rules {
    rule_map: HashMap<usize, Vec<usize>>,
}

impl Rules {
    fn compare(&self, left: &usize, right: &usize) -> Ordering {
        if Some(true) == self.rule_map.get(right).map(|rule| rule.contains(left)) {
            Ordering::Less
        } else if Some(true) == self.rule_map.get(left).map(|rule| rule.contains(right)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[test]
fn test_day_5_1() {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    assert_eq!(day_5_1(input), 143);
}

#[test]
fn test_day_5_2() {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    assert_eq!(day_5_2(input), 123);
}

#[test]
fn test_day_5_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_5.txt"));
    assert_eq!(day_5_1(&input), 5948);
}

#[test]
fn test_day_5_2_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_5.txt"));
    assert_eq!(day_5_2(&input), 3062);
}
