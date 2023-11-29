use std::collections::{HashMap, VecDeque};

pub(crate) fn day_21_1(input: &str) -> usize {
    let mut monkeys: HashMap<&str, usize> = HashMap::new();
    let mut shouts = input
        .lines()
        .filter_map(|line| {
            let (monkey, instruction) = line.split_once(": ").unwrap();
            if let Ok(number) = instruction.parse::<usize>() {
                monkeys.insert(monkey, number);
                None
            } else {
                Some((monkey, instruction))
            }
        })
        .collect::<VecDeque<(&str, &str)>>();
    while let Some((monkey, shout)) = shouts.pop_front() {
        let monkey_2 = &shout[0..4];
        let operation = &shout[5..=5];
        let monkey_3 = &shout[7..];
        if let Some(monkey_2_value) = monkeys.get(monkey_2) {
            if let Some(monkey_3_value) = monkeys.get(monkey_3) {
                let value = match operation {
                    "*" => monkey_2_value * monkey_3_value,
                    "/" => monkey_2_value / monkey_3_value,
                    "+" => monkey_2_value + monkey_3_value,
                    "-" => monkey_2_value - monkey_3_value,
                    _ => panic!("invalid operation"),
                };
                monkeys.insert(monkey, value);
                continue;
            }
        }
        shouts.push_back((monkey, shout));
    }
    *monkeys.get("root").unwrap()
}

pub(crate) fn day_21_2(input: &str) -> usize {
    let mut monkeys: HashMap<&str, usize> = HashMap::new();
    let mut shouts = input
        .lines()
        .filter_map(|line| {
            let (monkey, instruction) = line.split_once(": ").unwrap();
            if let Ok(number) = instruction.parse::<usize>() {
                monkeys.insert(monkey, number);
                None
            } else {
                Some((monkey, instruction))
            }
        })
        .collect::<VecDeque<(&str, &str)>>();
    // Magic number found by manual bisection search
    monkeys.insert("humn", 3453748220116);
    while let Some((monkey, shout)) = shouts.pop_front() {
        let monkey_2 = &shout[0..4];
        let operation = &shout[5..=5];
        let monkey_3 = &shout[7..];
        if let Some(monkey_2_value) = monkeys.get(monkey_2) {
            if let Some(monkey_3_value) = monkeys.get(monkey_3) {
                if monkey == "root" {
                    if monkey_2_value == monkey_3_value {
                        return *monkeys.get("humn").unwrap();
                    } else {
                        continue;
                    }
                };
                let value = match operation {
                    "*" => monkey_2_value * monkey_3_value,
                    "/" => monkey_2_value / monkey_3_value,
                    "+" => monkey_2_value + monkey_3_value,
                    "-" => monkey_2_value - monkey_3_value,
                    _ => panic!("invalid operation"),
                };
                monkeys.insert(monkey, value);
                continue;
            }
        }
        shouts.push_back((monkey, shout));
    }
    unreachable!()
}
