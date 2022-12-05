use itertools::Itertools;

fn char_to_prio(char: char) -> u64 {
    let num = char as u64 - 38;
    if num > 52 {
        num - 58 // lowercase
    } else {
        num // uppercase
    }
}

pub(crate) fn day_3_1(input: &str) -> u64 {
    input
        .lines()
        .map(|rucksack| {
            let first_comp = rucksack[0..(rucksack.len() / 2)].to_string();
            let second_comp = rucksack[(rucksack.len() / 2)..].to_string();
            assert_eq!(first_comp.len(), second_comp.len());
            first_comp
                .chars()
                .unique()
                .map(|char| {
                    if second_comp.contains(char) {
                        char_to_prio(char)
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

pub(crate) fn day_3_2(input: &str) -> u64 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let first = chunk.next().expect("should exist");
            let second = chunk.next().expect("should exist");
            let third = chunk.next().expect("should exist");
            first
                .chars()
                .unique()
                .map(|char| {
                    if second.matches(char).count() > 0 && third.matches(char).count() > 0 {
                        char_to_prio(char)
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

#[test]
fn test_day_3_1() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    assert_eq!(day_3_1(input), 157)
}

#[test]
fn test_day_3_2() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    assert_eq!(day_3_2(input), 70)
}
