use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Target {
    Register(String),
    Number(usize),
}

impl Target {
    fn new(s: &str, n: usize) -> Self {
        match s.split(' ').nth(n).unwrap().parse::<usize>() {
            Ok(num) => Self::Number(num),
            Err(_) => Self::Register(s.split(' ').nth(n).unwrap().to_string()),
        }
    }
}

#[derive(Debug)]
enum Command {
    Rshift(Target, Target, Target),
    Lshift(Target, Target, Target),
    Or(Target, Target, Target),
    Direct(Target, Target),
    Not(Target, Target),
    And(Target, Target, Target),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        use Command::*;
        match s
            .split(' ')
            .take(2)
            .collect_tuple()
            .expect("should have tuples")
        {
            (_, "RSHIFT") => Rshift(Target::new(s, 0), Target::new(s, 2), Target::new(s, 4)),
            (_, "LSHIFT") => Lshift(Target::new(s, 0), Target::new(s, 2), Target::new(s, 4)),
            (_, "OR") => Or(Target::new(s, 0), Target::new(s, 2), Target::new(s, 4)),
            (_, "->") => Direct(Target::new(s, 0), Target::new(s, 2)),
            ("NOT", _) => Not(Target::new(s, 1), Target::new(s, 3)),
            (_, "AND") => And(Target::new(s, 0), Target::new(s, 2), Target::new(s, 4)),
            _ => {
                panic!("invalid command")
            }
        }
    }
}

pub(crate) fn day_7_1(input: &str) -> usize {
    use Command::*;
    use Target::*;
    let commands = input
        .lines()
        .map(|line| line.into())
        .collect::<Vec<Command>>();
    let mut registers: HashMap<&String, Option<usize>> = HashMap::new();
    commands.iter().for_each(|command| match command {
        Command::Direct(Number(_), Register(a)) => {
            registers.insert(a, None);
        }
        Rshift(Register(a), _, Register(b))
        | Lshift(Register(a), _, Register(b))
        | Direct(Register(a), Register(b))
        | Not(Register(a), Register(b))
        | And(Number(_), Register(a), Register(b)) => {
            registers.insert(a, None);
            registers.insert(b, None);
        }
        Or(Register(a), Register(b), Register(c)) | And(Register(a), Register(b), Register(c)) => {
            registers.insert(a, None);
            registers.insert(b, None);
            registers.insert(c, None);
        }
        _ => {
            panic!("invalid command");
        }
    });
    while registers.iter().any(|(_, v)| v.is_none()) {
        commands.iter().for_each(|command| match command {
            Rshift(Register(a), Number(b), Register(c)) => {
                if let Some(Some(a)) = registers.get(a) {
                    registers.insert(c, Some(a >> b));
                };
            }
            Lshift(Register(a), Number(b), Register(c)) => {
                if let Some(Some(a)) = registers.get(a) {
                    registers.insert(c, Some(a << b));
                };
            }
            Or(Register(a), Register(b), Register(c)) => {
                if let Some(Some(a)) = registers.get(a) {
                    if let Some(Some(b)) = registers.get(b) {
                        registers.insert(c, Some(a | b));
                    }
                };
            }
            Direct(Register(a), Register(b)) => {
                if let Some(a) = registers.get(a) {
                    registers.insert(b, *a);
                };
            }
            Direct(Number(a), Register(b)) => {
                registers.insert(b, Some(*a));
            }
            Not(Register(a), Register(b)) => {
                if let Some(Some(a)) = registers.get(a) {
                    registers.insert(b, Some(!*a));
                };
            }
            And(Register(a), Register(b), Register(c)) => {
                if let Some(Some(a)) = registers.get(a) {
                    if let Some(Some(b)) = registers.get(b) {
                        registers.insert(c, Some(a & b));
                    }
                };
            }
            And(Number(a), Register(b), Register(c)) => {
                if let Some(Some(b)) = registers.get(b) {
                    registers.insert(c, Some(a & b));
                }
            }
            _ => {
                panic!("invalid command")
            }
        });
    }
    registers.get(&String::from("a")).unwrap().unwrap()
}
