use itertools::Itertools;
use serde_json::Value;
use std::cmp::{max, Ordering};

fn ordered(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),
        (Value::Array(a), Value::Array(b)) => {
            for i in 0..max(a.len(), b.len()) {
                match (a.get(i), b.get(i)) {
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(x), Some(y)) => match ordered(x, y) {
                        Ordering::Equal => {}
                        comp => return comp,
                    },
                }
            }
            Ordering::Equal
        }
        (Value::Number(_), Value::Array(_)) => ordered(&Value::Array(vec![left.clone()]), right),
        (Value::Array(_), Value::Number(_)) => ordered(left, &Value::Array(vec![right.clone()])),
        _ => unreachable!(),
    }
}

pub(crate) fn day_13_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>()
        .iter()
        .tuples()
        .positions(|(left, right)| ordered(left, right) != Ordering::Greater)
        .map(|n| n + 1)
        .sum()
}

pub(crate) fn day_13_2(input: &str) -> usize {
    let dividers = [
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ];
    let mut lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    lines.extend(dividers.iter().cloned());
    lines.sort_by(ordered);
    lines
        .iter()
        .positions(|b| dividers.contains(b))
        .map(|i| i + 1)
        .product()
}
