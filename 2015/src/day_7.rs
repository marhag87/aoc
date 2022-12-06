use std::{fmt::Error, str::FromStr};

enum Target {
    Register(String),
    Direct(usize),
}

enum Command {
    Rshift(Target, usize, Target),
    Lshift(Target, usize, Target),
    Or(Target, Target, Target),
    Direct(Target, Target),
    Not(Target, Target),
    And(Target, Target, Target),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        todo!()
    }
}

pub(crate) fn day_7_1(input: &str) -> u64 {
    let commands = input
        .lines()
        .map(|line| line.into())
        .collect::<Vec<Command>>();
    todo!()
}
