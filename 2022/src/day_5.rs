fn parse_state(input: &str) -> Vec<Vec<char>> {
    let mut state: Vec<Vec<char>> = vec![
        vec![], // Empty vec to make instructions match 1-based vec
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    input.lines().rev().skip(1).for_each(|line| {
        let mut n = 1;
        line.chars().skip(1).step_by(4).for_each(|char| {
            if char != ' ' {
                state[n].push(char)
            };
            n += 1;
        })
    });
    state
}

fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize)
                .expect("should match format")
        })
        .collect()
}

fn parse_end_state(state: Vec<Vec<char>>) -> String {
    state
        .iter()
        .skip(1)
        .map(|stack| stack.last().expect("should have chars"))
        .collect()
}

pub(crate) fn day_5_1(input: String) -> String {
    let mut input = input.split("\n\n");
    let start_state = input.next().expect("should have start state");
    let instructions = input.next().expect("should have instructions");

    let mut state = parse_state(start_state);
    parse_instructions(instructions)
        .into_iter()
        .for_each(|(num, from, to)| {
            (0..num).for_each(|_| {
                let moved = state[from].pop().expect("should exist");
                state[to].push(moved)
            })
        });
    parse_end_state(state)
}

pub(crate) fn day_5_2(input: String) -> String {
    let mut input = input.split("\n\n");
    let start_state = input.next().expect("should have start state");
    let instructions = input.next().expect("should have instructions");

    let mut state = parse_state(start_state);
    parse_instructions(instructions)
        .into_iter()
        .for_each(|(num, from, to)| {
            let at = state[from].len() - num;
            let mut moved = state[from].split_off(at);
            state[to].append(&mut moved)
        });
    parse_end_state(state)
}
