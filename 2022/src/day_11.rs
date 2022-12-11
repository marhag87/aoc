use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn new(input: &str) -> Self {
        let mut parts = input.split(' ').skip(6);
        let operation = parts.next().expect("should have an operation");
        let number = parts.next().expect("should have a number");
        match (operation, number) {
            ("*", "old") => Operation::Square,
            ("*", number) => {
                Operation::Multiply(number.parse::<usize>().expect("should be a number"))
            }
            ("+", number) => Operation::Add(number.parse::<usize>().expect("should be a number")),
            _ => panic!("invalid operation"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divider: usize,
    monkey_true: usize,
    monkey_false: usize,
    inspections: usize,
    worry_divider: bool,
}

impl Monkey {
    fn new(input: &str, worry_divider: bool) -> Self {
        let mut lines = input.lines().skip(1);
        let items = lines
            .next()
            .expect("should have starting items")
            .split(' ')
            .map(|c| c.trim_matches(','))
            .skip(4)
            .map(|num| num.parse::<usize>().expect("should be a number"))
            .collect::<Vec<usize>>();
        let operation = Operation::new(lines.next().expect("should have an operation"));
        let nums = lines
            .map(|line| {
                line.split(' ')
                    .last()
                    .map(|num| num.parse::<usize>().expect("should be a number"))
                    .expect("should be a number")
            })
            .collect::<Vec<usize>>();
        Monkey {
            items,
            operation,
            divider: nums[0],
            monkey_true: nums[1],
            monkey_false: nums[2],
            inspections: 0,
            worry_divider,
        }
    }

    fn inspect_items(&mut self, mod_product: usize) -> Vec<(usize, usize)> {
        self.items
            .drain(..)
            .map(|item| {
                self.inspections += 1;
                let item = item % mod_product;
                let worry_level = match self.operation {
                    Operation::Square => item * item,
                    Operation::Add(num) => item + num,
                    Operation::Multiply(num) => item * num,
                };
                let bored_level = match self.worry_divider {
                    true => worry_level / 3,
                    false => worry_level,
                };
                match bored_level % self.divider == 0 {
                    true => (self.monkey_true, bored_level),
                    false => (self.monkey_false, bored_level),
                }
            })
            .collect()
    }
}

fn shenanigans(input: &str, rounds: usize, worry_divider: bool) -> usize {
    // Parse input
    let mut monkeys = input
        .split("\n\n")
        .map(|monkey_input| Monkey::new(monkey_input, worry_divider))
        .collect::<Vec<Monkey>>();

    // I had to cheat on this one, looked this up on reddit.
    // Product of all monkey mods can be used to not get overflows
    let mod_product = monkeys.iter().map(|monkey| monkey.divider).product();

    // Rounds of monkey shenanigans
    (0..rounds).for_each(|_| {
        (0..monkeys.len()).for_each(|n| {
            monkeys[n]
                .inspect_items(mod_product)
                .into_iter()
                .for_each(|(monkey, item)| monkeys[monkey].items.push(item));
        });
    });

    // Product of two most active stuff-slingers
    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub(crate) fn day_11_1(input: &str) -> usize {
    shenanigans(input, 20, true)
}

pub(crate) fn day_11_2(input: &str) -> usize {
    shenanigans(input, 10000, false)
}

#[test]
fn test_day_11_1() {
    let input = String::from_utf8_lossy(include_bytes!("../test_input/day_11.txt"));
    assert_eq!(day_11_1(&input), 10605);
}

#[test]
fn test_day_11_2() {
    let input = String::from_utf8_lossy(include_bytes!("../test_input/day_11.txt"));
    assert_eq!(day_11_2(&input), 2713310158);
}
